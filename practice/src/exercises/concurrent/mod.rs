/**
Arc：原子引用计数（Atomically Reference-Counted）指针，可以在多线程环境下使用，以延长某些数据的使用寿命，直到所有线程都使用完为止。
Barrier：屏障。确保多个线程相互等待对方到达程序中的某个点，然后再一起继续执行。
Condvar：条件变量，提供在等待事件发生时阻止一个线程的能力。
mpsc：多生产者，单消费（Multi-producer, single-consumer）队列，用于基于消息的通信。可以提供轻量级的线程间同步机制，代价是增加一些额外的内存。
互斥机制(Mutex)。互斥机制，确保每次最多只有一个线程能够访问一些数据。
Once：用于全局变量的线程安全的一次性初始化。
RwLock：用于全局变量的初始化。提供了一个相互排斥机制，允许多个读同时访问，同时一次只允许一个写。在某些情况下，这可能比mutex更有效。
 **/

pub mod threads_test;
mod atomic;
mod index;