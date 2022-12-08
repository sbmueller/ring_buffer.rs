pub struct RingBuffer<T: Sized + Default + Copy, const CAPACITY: usize> {
    data: [T; CAPACITY],
    size: usize,
    end_index: usize,
}

impl<T: Sized + Default + Copy, const CAPACITY: usize> RingBuffer<T, CAPACITY> {
    pub fn new() -> Self {
        RingBuffer {
            data: [T::default(); CAPACITY],
            size: 0,
            end_index: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        CAPACITY
    }

    /// Add new item at the end of the buffer
    pub fn push(&mut self, item: T) {
        self.data[self.end_index % CAPACITY] = item;
        self.end_index = (self.end_index + 1) % CAPACITY;
        if self.size < CAPACITY {
            self.size += 1;
        }
    }

    /// Return and remove the oldest item at the beginning of the buffer
    pub fn pop(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }
        let return_value = self.data[(self.end_index + CAPACITY - self.size) % CAPACITY];
        self.size -= 1;
        return Some(return_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let mut buffer = RingBuffer::<i32, 5>::new();
        assert_eq!(buffer.size(), 0);
        assert_eq!(buffer.capacity(), 5);
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn push_items() {
        let mut buffer = RingBuffer::<i32, 5>::new();
        buffer.push(42);
        // Logical content after push: [0, 0, 0, 0, 42]
        assert_eq!(buffer.size(), 1);
        buffer.push(21);
        // Logical content after push: [0, 0, 0, 42, 21]
        assert_eq!(buffer.size(), 2);
        assert_eq!(buffer.pop(), Some(42));
        assert_eq!(buffer.size(), 1);
        assert_eq!(buffer.pop(), Some(21));
    }

    #[test]
    fn overwrite_old_items() {
        let mut buffer = RingBuffer::<i32, 5>::new();
        buffer.push(1); // overwritten
        buffer.push(2); // overwritten
        buffer.push(3);
        buffer.push(4);
        buffer.push(5);
        buffer.push(6);
        buffer.push(7);
        assert_eq!(buffer.size(), 5);
        assert_eq!(buffer.pop(), Some(3));
    }

    #[test]
    fn pop_items() {
        let mut buffer = RingBuffer::<i32, 5>::new();
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);
        buffer.push(5);
        assert_eq!(Some(1), buffer.pop());
        assert_eq!(4, buffer.size());
        assert_eq!(Some(2), buffer.pop());
        assert_eq!(3, buffer.size());
        assert_eq!(Some(3), buffer.pop());
        assert_eq!(2, buffer.size());
        buffer.push(6);
        assert_eq!(Some(4), buffer.pop());
        assert_eq!(2, buffer.size());
    }

    #[test]
    fn push_pop_items() {
        let mut buffer = RingBuffer::<i32, 5>::new();
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);
        buffer.push(5);
        buffer.pop();
        buffer.pop();
        buffer.pop();
        buffer.pop();
        buffer.push(6);
        buffer.push(7);
        assert_eq!(buffer.size(), 3);
        assert_eq!(buffer.pop(), Some(5))
    }
}
