// TODO: maybe consider using linked list for faster insert
use std::sync::{Arc, Mutex, mpsc::{self, Sender}};
use std::thread;
use rust_decimal::Decimal;
use core::error::Error;

const BUFFER_SIZE: usize = 256;

pub enum BufferOperation {
    Update(usize, usize, Decimal),
    Insert(usize, Decimal),
}

pub struct Buffer {
    sender: Sender<BufferOperation>,
    buffers: Arc<Mutex<[[Decimal; BUFFER_SIZE]; 3]>>,
}

impl Buffer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let buffers = Arc::new(Mutex::new([[Decimal::ZERO; BUFFER_SIZE]; 3]));

        let buffer_pointer = Arc::clone(&buffers);
        thread::spawn(move || {
            while let Ok(buffer_operation) = rx.recv() {
                let mut buffers_ = buffer_pointer.lock().unwrap();
                match buffer_operation {
                    BufferOperation::Update(buf_index, index, value) => {
                        if index >= BUFFER_SIZE || buf_index > 2 {
                            println!("buffer index must be [0 to 2], index [0 to 255], buffer: {}, index: {}", buf_index, index);
                            continue;
                        }
                        buffers_[buf_index][index] = value;
                    }
                    BufferOperation::Insert(buf_index, value) => {
                        if buf_index > 2 {
                            println!("buffer index must be [0 to 2]");
                            continue;
                        }
                        for i in (1..BUFFER_SIZE).rev() {
                            buffers_[buf_index][i] = buffers_[buf_index][i - 1];
                        }
                        buffers_[buf_index][0] = value;
                    }
                }
            }

            println!("Thread exiting... buffers data dropped");
        });

        Buffer { sender: tx, buffers }
    }
    pub fn update(&self, buf_index: usize, index: usize, value: Decimal) {
        self.sender.send(BufferOperation::Update(buf_index, index, value)).unwrap();
    }
    pub fn insert(&self, buf_index: usize, value: Decimal) {
        self.sender.send(BufferOperation::Insert(buf_index, value)).unwrap();
    }
    pub fn get_buffer(&self, buf_index: usize) -> Result<[Decimal; BUFFER_SIZE], Box<dyn Error>> {
        if buf_index > 2 {
            return Err("buffer index must be [0 to 2]".into());
        }
        Ok(self.buffers.lock().unwrap()[buf_index])
    }
}
