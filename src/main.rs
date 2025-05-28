use std::ops::{Deref, Index, IndexMut};

pub struct CircularBuffer<T>{
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    full: bool,
}
pub enum CircularBufferError {
    BufferFull,
    BufferEmpty,
    InvalidSize,
}
impl <T> CircularBuffer<T>{
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; size],
            head: 0,
            tail: 0,
            size,
            full: false,
        }
    }

    pub fn write(&mut self, item: T) -> Result<& mut CircularBuffer<T>,CircularBufferError::BufferFull> {
        if self.full{
            return Err(CircularBufferError::BufferFull);
        }
        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.size;
        if self.head == self.tail {
            self.full = true;
        }
        Ok(self)

    }
    pub fn read(&mut self) -> Option(T) {
        if self.empty() {
            return None;
        }
        let Some(item) = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.size;
        item
    }
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.full = false;
    }
    pub fn size(&self) -> usize{
        self.size
    }
    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio
    pub fn overwrite(&mut self, item: T) {
        if self.full{
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % self.size;

    } else { let _ = self.write(item); }
    }
    // vedi sotto*
    pub fn make_contiguous(&mut self) {
        // if it's empty, we can just reset the pointers
        if self.size == 0 {
            self.head = 0;
            self.tail = 0;
        } else {
            // otherwise we need to make it contiguos: just rotate it until head is zero
            while self.head != 0 {
                let element = self.read().unwrap();
                self.write(element).unwrap();
            }
        }
    }
    fn is_contiguous(&self) -> bool {
        self.size == 0 || self.head <= self.tail || self.tail == 0
    }
}
impl<T> Index<usize> for CircularBuffer<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index > self.size {
            panic!("Index out of bounds");
        }
        let real_index = (index + self.head)% self.size;
        self.buffer[real_index].as_ref().expect("accessing an empty buffer")
    }

}
impl <T> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index > self.size {
            panic!("Index out of bounds");
        }
        let real_index = (index + self.head) % self.size;
        self.buffer[real_index].as_mut().expect("accessing an empty buffer")
    }
}
impl<T> Deref for CircularBuffer<T> {
    type Target = [Option<T>];

    fn deref(&self) -> &Self::Target {
        let size = self.size;
        if size == 0 {
            return &[]
        }
       if self.is_contiguous() {
            &self.buffer[self.head..self.head+size]
        } else {
            panic!("Buffer is not contiguous");
        }
    }
}

fn main() {
    println!("Hello, world!");
}
