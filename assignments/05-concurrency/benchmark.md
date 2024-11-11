# Benchmark Results for Parallel Merge Sort

I benchmarked the parallel merge sort on an array of 10 million elements using different numbers of threads. The chunk size was set to 10,000 elements.

| Number of Threads | Time Taken |
|-------------------|------------|
| 1                 | 1.59s      |
| 2                 | 878.88ms   |
| 4                 | 489.03ms   |
| 8                 | 303.57ms   |
| 16                | 294.62ms   |
| 24                | 291.87ms   |
| 32                | 296.25ms   |
| 48                | 315.86ms   |
| 64                | 295.30ms   |
| 96                | 302.45ms   |
| 100               | 302.83ms   |

**Fastest Runtime:** The fastest runtime was achieved with 24 threads, taking 291.87ms.

Pros and Cons

**More Threads**

Pros: Utilizes multiple CPU cores for parallel processing, potentially reducing execution time.

Cons: Excessive threading can lead to overhead from context switching and resource contention, which may degrade performance after a certain point.

**Fewer Threads**

Pros: Lower overhead with less context switching, and simpler thread management.

Cons: May underutilize available CPU resources, leading to longer execution times due to less parallelism.