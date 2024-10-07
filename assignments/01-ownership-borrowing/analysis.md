It's not possible to implement a doubly linked list in safe rust because one object cannot have two owners.

Another example of something that is hard to implement because of Rust's ownership system is UI state. Wh