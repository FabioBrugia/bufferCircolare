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
        todo!()
    }
}



fn main() {
    println!("Hello, world!");
}
