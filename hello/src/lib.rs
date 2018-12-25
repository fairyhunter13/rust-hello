use std::{
    fs,
    io::{prelude::*, Result},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn handle_connections(mut stream: TcpStream) -> Result<()> {
    let mut buffer = vec![0; 512];

    // println!("buffer: {:#?}", buffer);
    stream.set_write_timeout(Some(Duration::new(5, 0)))?;
    stream.read(&mut buffer)?;

    // println!("Buffer: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK \r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK \r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename)?;
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// struct Job;
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            // Using while let is ok, but the lock is released until job.call_box() done.
            // This is happening because the lifetime of MutexGuard<T> in
            // while expression is remained in scope for the duration of the block (outside the block).
            // The lock remains held for the duration of the call to job.call_box(),
            // meaning other workers cannot receive jobs.
            // It is also interpreted as other workers (threads) can't acquire the lock.
            // We need the lifetime of MutexGuard<T> to end when the let job statement ends.
            // The lock is received when calling recv(), but it is released before the call to
            // job.call_box().
            // It is more optimized to use loop one.
            // Best serve for concurrency.

            //    while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {} got a job; executing.", id);

            //     job.call_box();
            // }
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got job, executing.", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Self {
            id: id,
            thread: Some(thread),
        }
    }
}
