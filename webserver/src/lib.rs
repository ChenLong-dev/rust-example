pub mod mult_thread {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };
    pub struct ThreadPool {
        #[allow(dead_code)]
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    #[warn(dead_code)]
    impl ThreadPool {
        /// 创建线程池。
        ///
        /// 线程池中线程的数量。
        ///
        /// # Panics
        ///
        /// `new` 函数在 size 为 0 时会 panic。
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

        pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }
    
    #[allow(dead_code)]
    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || {
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
            });

            Worker { id, thread }
        }
    }   
}

pub mod grace {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };
    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    #[warn(dead_code)]
    impl ThreadPool {
        /// 创建线程池。
        ///
        /// 线程池中线程的数量。
        ///
        /// # Panics
        ///
        /// `new` 函数在 size 为 0 时会 panic。
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            
            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());

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
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
}

/* ***************************************************************** */
    
use std::path::{PathBuf};
use std::env;
use regex::Regex;

// 获取html文件路径
pub fn get_html_path(target: String) -> String {
    let mut base_path:String = env::var("MY_ENV_HTML_DIR_VAR").unwrap_or_else(|e| {
        eprintln!("无法读取环境变量: {}", e);
        "".to_string()
    });
    if &base_path == "" {
        base_path = "src/html".to_string();
    }
    println!("xxx base_path: {}", base_path);
    
    let re = Regex::new(r"[\\/]").unwrap(); // 使用正则表达式匹配“/”或“\”
    let parts: Vec<&str> = re.split(&base_path).collect(); // 分割字符串并收集到向量中

    let mut path_buf = PathBuf::new();
    for part in parts {
        path_buf.push(part);
    }
    path_buf.push(target);
    format!("{}", path_buf.display())
}

#[cfg(test)]
mod tools_tests {
    use super::*;
    #[test]
    fn test_get_html_path() {
        let path = get_html_path(String::from("hello.html"));
        println!("xxx [test_get_html_path] path:{path}");
        if cfg!(windows) {
            assert_eq!(path, "src\\html\\hello.html")
        } else if cfg!(unix) {
            assert_eq!(path, "src/html/hello.html")
        }
    }
}