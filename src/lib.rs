use std::{sync::{mpsc, Arc, Mutex}, thread};
use core::ops::FnOnce;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop { 
            // extract job from channel and execute
            let job = receiver.lock().unwrap().recv().unwrap(); // Todo: expect instead
            println!("Worker {id} got a job; executing.");
            job();
        }); // in production, return Result instead
        Worker { id, thread }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // use `with_capacity` to preallocates space in the vector
        let mut workers: Vec<Worker> = Vec::with_capacity(size); 
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));    
            // so the workers can share ownership of the receiver.
        }
        ThreadPool {workers, sender}
    }
    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
        // we need Send to transfer the closure from one thread to another
        // 'static because we don’t know how long the thread will take to execute
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    } 
}