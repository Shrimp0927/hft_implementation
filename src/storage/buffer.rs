use std::sync::{Arc, Mutex, mpsc::{self, Sender}};
use std::thread;
use rust_decimal::Decimal;
use core::error::Error;

const BUFFER_SIZE: usize = 256;

pub struct Buffer {
    sender: Sender<(usize, usize, Decimal)>,
    buffers: Arc<Mutex<[[Decimal; BUFFER_SIZE]; 3]>>,
}

impl Buffer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let buffers = Arc::new(Mutex::new([[Decimal::ZERO; BUFFER_SIZE]; 3]));

        let buffer_pointer = Arc::clone(&buffers);
        thread::spawn(move || {
            while let Ok((buf_index, index, value)) = rx.recv() {
                if index >= BUFFER_SIZE || buf_index > 2 {
                    println!("buffer index must be [0 to 2], index [0 to 255], buffer: {}, index: {}", buf_index, index);
                    continue;
                }
                let mut buffers2 = buffer_pointer.lock().unwrap();
                buffers2[buf_index as usize][index] = value;
            }

            println!("Thread exiting... buffers data dropped");
        });

        Buffer { sender: tx, buffers }
    }
    pub fn update(&self, buf_index: usize, index: usize, value: Decimal) {
        self.sender.send((buf_index, index, value)).unwrap();
    }
    pub fn get_buffer(&self, buf_index: usize) -> Result<[Decimal; BUFFER_SIZE], Box<dyn Error>> {
        if buf_index > 2 {
            println!("");
            return Err("buffer index must be [0 to 2]".into());
        }
        Ok(self.buffers.lock().unwrap()[buf_index])
    }
}
