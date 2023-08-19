use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    
    recv_tasks: spmc::Receiver<TaskType>, //  single producer, multiple consumer queue 
    // enqueued by the main thread, recieved by workers


    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    
    recv_output: mpsc::Receiver<TaskType::Output>, //  multiple producer, single consumer queue 
    // enqueued (send results by workers), recieved by main thread
    
    workers: Vec<thread::JoinHandle<()>>,
    // array of workers waiting for the tasks
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    // argument is number of workers
    // output is the work queue structure
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // creating channels for main thread and worker threads
        
        // single producer, multiple consumer (for recieving tasks)
        // send_tasks (single main thread), recv_task multiple consumer (workers)
        let (send_tasks, recv_tasks) = spmc::channel();

        // multiple producer, single consumer queue (for output)
        // each producer sending their own output, only one main thread recieving it
        let (send_output, recv_output) = mpsc::channel();

        let mut workers = Vec::new();

        // creating threads
        for _ in 0..n_workers {
            let recv_tasks_clone = recv_tasks.clone();
            let send_output_clone = send_output.clone();
            // inputing tasks queue and output queue
            let worker_handle = thread::spawn(move || {
                Self::run(recv_tasks_clone, send_output_clone);
            });
            workers.push(worker_handle);
        }

        // create Queue and return it
        WorkQueue {
            send_tasks: Some(send_tasks), //since send tasks is an option
            recv_tasks,
            recv_output,
            workers,
        }
    }

    // worker thread's function:
    // while there is tasks being send through recv_task channel handle them and output it to 
    // send_output channel
    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // loop and process tasks as long as tasks are being received. 
        // When recv() returns an error, the loop will exit.
        while let Ok(task) = recv_tasks.recv() {
            // if there is a result not a None
            if let Some(result) = task.run() {
                // send the result through send_output clones
                send_output.send(result).unwrap();
            }
        }
    }

    // Main thread's function:
    // producing tasks based on input and sending it through send_tasks channel to worker threads
    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        // refer to send tasks as sender and send the message through the channel
        if let Some(sender) = self.send_tasks.as_mut() {
            sender.send(t)
        }

        // the work queue has been shut down as send_tasks does not exist anymore
        else { 
            Err(spmc::SendError(t))
        }
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // Destroy the spmc::Sender so everybody knows no more tasks are incoming;
        self.send_tasks.take(); // Closes the task channel

        // Try to receive any remaining tasks from the queue, effectively draining it
        while let Ok(_) = self.recv_tasks.try_recv() {}

        // Wait for each worker thread to finish, draining them from the vector
        for worker in self.workers.drain(..) {
            if let Err(e) = worker.join() {
                eprintln!("Error joining worker thread: {:?}", e);
            }
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
