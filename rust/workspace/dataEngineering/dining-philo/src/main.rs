use std::{thread, sync::{Arc, Condvar, Mutex}, time::Duration};

struct Semaphore {
    count: Mutex<usize>,
    condvar: Condvar,
}

struct CS {
    id: u32,
}

struct Philo {
    id: u32,
    left_cs: Arc<Mutex<CS>>,
    right_cs: Arc<Mutex<CS>>,
}

impl Semaphore {
    fn new(count: usize) -> Self {
        Semaphore {
            count: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }
    fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.condvar.wait(count).unwrap();
        }
        *count -= 1;
    }
    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.condvar.notify_one();
    }
}

impl Philo {
    fn eat(&self) {
        // avoid deadlock, last philosopher picks up right first
        if self.id == 5 {
            let right = self.right_cs.lock().unwrap();
            let left = self.left_cs.lock().unwrap();
            
            println!("philosopher {} picked up right chopstick {} and left chopstick {}. eating.", self.id, right.id, left.id);
            thread::sleep(Duration::from_millis(200));
            println!("philosopher {} finished.", self.id);
        } else {
            let left = self.left_cs.lock().unwrap();
            let right = self.right_cs.lock().unwrap();

            println!("philosopher {} picked up left chopstick {} and right chopstick {}. eating", self.id, left.id, right.id);
            thread::sleep(Duration::from_millis(200));
            println!("philosopher {} finished.", self.id);
        }
    }
}

fn make_cs() -> Vec<Arc<Mutex<CS>>> {
    (0..5)
        .map(|i| Arc::new(Mutex::new(CS { id: i })))
        .collect()
}

fn make_philo() -> Vec<Arc<Mutex<Philo>>> {
    let cs = make_cs();
    (0..5)
        .map(|i| {
            Arc::new(Mutex::new(Philo {
                id: (i + 1) as u32,
                left_cs: cs[i].clone(),
                right_cs: cs[(i + 1) % 5].clone(),
            }))
        })
        .collect()
}

fn host(philos: Vec<Arc<Mutex<Philo>>>) {
    // 5 philosophers, allow 4 to try at once
    let semaphore = Arc::new(Semaphore::new(4));
    let handles: Vec<_> = philos.into_iter().map(|philo| {
        let sem = semaphore.clone();
        thread::spawn(move || {
            for _ in 0..3 {
                sem.acquire();
                philo.lock().unwrap().eat();
                sem.release();
            }
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

fn main() {
    let philos = make_philo();
    host(philos);
}
