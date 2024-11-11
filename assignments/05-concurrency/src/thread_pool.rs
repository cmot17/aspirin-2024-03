use crate::error::ThreadPoolError;
use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

type Job<R> = Box<dyn FnOnce() -> R + Send + 'static>;

pub struct ThreadPool<R> {
    job_sender: Option<mpsc::Sender<Job<R>>>,
    result_receiver: mpsc::Receiver<Result<R, ThreadPoolError>>,
    threads: Vec<std::thread::JoinHandle<()>>,
}

impl<R: 'static + Send + std::fmt::Debug> ThreadPool<R> {
    /// Create a new ThreadPool with num_threads threads.
    ///
    /// Errors:
    /// - If num_threads is 0, return an error
    pub fn new(num_threads: usize) -> Result<ThreadPool<R>, ThreadPoolError> {
        if num_threads == 0 {
            return Err(ThreadPoolError::ZeroThreads);
        }

        let (job_sender, job_receiver): (mpsc::Sender<Job<R>>, mpsc::Receiver<Job<R>>) =
            mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut threads = Vec::with_capacity(num_threads);

        for id in 0..num_threads {
            let job_receiver = Arc::clone(&job_receiver);
            let result_sender = result_sender.clone();

            threads.push(std::thread::spawn(move || {
                println!("Worker thread {} started.", id);
                loop {
                    let job = {
                        let receiver = job_receiver.lock().unwrap();
                        receiver.recv()
                    };
                    match job {
                        Ok(job) => {
                            println!("Worker thread {} executing a job.", id);
                            // Catch panics during job execution
                            let result = panic::catch_unwind(AssertUnwindSafe(job));
                            match result {
                                Ok(val) => {
                                    result_sender.send(Ok(val)).unwrap();
                                }
                                Err(e) => {
                                    let panic_info = if let Some(s) = e.downcast_ref::<&str>() {
                                        s.to_string()
                                    } else if let Some(s) = e.downcast_ref::<String>() {
                                        s.clone()
                                    } else {
                                        "Unknown panic".to_string()
                                    };
                                    result_sender
                                        .send(Err(ThreadPoolError::ThreadPanic(panic_info)))
                                        .unwrap();
                                }
                            }
                        }
                        Err(_) => {
                            println!("Worker thread {} received shutdown signal.", id);
                            break;
                        }
                    }
                }
                println!("Worker thread {} exiting.", id);
            }));
        }

        Ok(ThreadPool {
            job_sender: Some(job_sender),
            result_receiver,
            threads,
        })
    }

    /// Execute the provided function on the thread pool
    ///
    /// Errors:
    /// - If we fail to send a message, report an error
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() -> R + Send + 'static,
    {
        println!("Sending a job to the thread pool.");
        if let Some(ref sender) = self.job_sender {
            sender
                .send(Box::new(f))
                .map_err(|_| ThreadPoolError::SendError)
        } else {
            Err(ThreadPoolError::SendError)
        }
    }

    /// Retrieve any results from the thread pool that have been computed
    #[allow(dead_code)]
    pub fn get_results(&self) -> Result<Vec<R>, ThreadPoolError> {
        let mut results = Vec::new();
        for res in self.result_receiver.try_iter() {
            match res {
                Ok(val) => results.push(val),
                Err(e) => return Err(e),
            }
        }
        Ok(results)
    }

    pub fn recv_result(&self) -> Result<R, ThreadPoolError> {
        match self.result_receiver.recv() {
            Ok(res) => res,
            Err(_) => Err(ThreadPoolError::ReceiveError(
                "Failed to receive result".to_owned(),
            )),
        }
    }
}

impl<R> Drop for ThreadPool<R> {
    fn drop(&mut self) {
        println!("Dropping ThreadPool...");
        // Explicitly drop the job_sender to close the channel
        self.job_sender.take();
        println!("Job sender dropped.");

        for (i, thread) in self.threads.drain(..).enumerate() {
            match thread.join() {
                Ok(_) => println!("Thread {} joined successfully.", i),
                Err(e) => eprintln!("Failed to join thread {}: {:?}", i, e),
            }
        }
        println!("All threads joined.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_threadpool_creation() {
        assert!(ThreadPool::<i32>::new(0).is_err());
        assert!(ThreadPool::<i32>::new(1).is_ok());
    }

    #[test]
    fn test_execute_single_task() {
        let pool = ThreadPool::new(1).unwrap();
        pool.execute(|| 42).unwrap();
        thread::sleep(Duration::from_millis(10));
        let results = pool.get_results().unwrap();
        assert_eq!(results, vec![42]);
    }

    #[test]
    fn test_multiple_tasks() {
        let pool = ThreadPool::new(2).unwrap();
        for i in 0..5 {
            pool.execute(move || i).unwrap();
        }
        thread::sleep(Duration::from_millis(50));
        let results = pool.get_results().unwrap();
        assert_eq!(results.len(), 5);
        for i in 0..5 {
            assert!(results.contains(&i));
        }
    }

    #[test]
    fn test_dropped_sender() {
        let mut pool = ThreadPool::new(1).unwrap();
        let (dummy_sender, _) = mpsc::channel();
        drop(std::mem::replace(&mut pool.job_sender, Some(dummy_sender)));
        assert!(pool.execute(|| 42).is_err());
    }

    #[test]
    fn test_different_types() {
        let pool = ThreadPool::new(1).unwrap();
        pool.execute(|| "test".to_string()).unwrap();
        thread::sleep(Duration::from_millis(10));
        let _ = pool.get_results();
    }

    #[test]
    fn test_thread_panic() {
        let pool = ThreadPool::new(1).unwrap();
        pool.execute(|| panic!("test panic")).unwrap();
        thread::sleep(Duration::from_millis(10));
        let res = pool.get_results();
        assert!(matches!(res, Err(ThreadPoolError::ThreadPanic(_))));
    }

    #[test]
    fn test_poison_error() {
        let pool = ThreadPool::new(1).unwrap();
        let mutex = Arc::new(Mutex::new(0));
        let mutex2 = mutex.clone();

        std::thread::spawn(move || {
            let _guard = mutex2.lock().unwrap();
            panic!("poison mutex");
        })
        .join()
        .unwrap_err();

        pool.execute(move || {
            let _guard = mutex.lock().unwrap();
            42
        })
        .unwrap();
        thread::sleep(Duration::from_millis(10));
        let res = pool.get_results();
        assert!(matches!(res, Err(ThreadPoolError::ThreadPanic(_))));
    }
}
