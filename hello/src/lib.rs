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

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
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
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job, executing.", id);

                job.call_box();
            }
        });
        Self { id, thread }
    }
}
