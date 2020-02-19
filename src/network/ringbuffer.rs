pub struct RingBuffer<T> {
    elements: Vec<Option<T>>,
    max_elements: usize,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> RingBuffer<T> {
    //TODO: test
    pub fn new(max_elements: usize) -> Self {
        let mut elements = vec![];

        for _i in 0..max_elements {
            elements.push(None);
        }

        return Self {
            elements: elements,
            max_elements: max_elements,
            head: 0,
            tail: 0,
            size: 0,
        };
    }

    //TODO: test
    pub fn front(&self) -> &Option<T> {
        return &self.elements[self.tail];
    }

    //TODO: test
    pub fn item(&self, index: usize) -> &Option<T> {
        return &self.elements[(self.tail + index) % self.max_elements];
    }

    //TODO: test
    pub fn pop(&mut self) {
        //if self.size == self.max_elements {
        //    panic!("Buffer overflow!");
        // }

        self.elements[self.tail] = None;
        self.tail = (self.tail + 1) % self.max_elements;
        self.size -= 1;
    }

    //TODO: test
    pub fn push(&mut self, item: T) {
        //if self.size == (self.max_elements - 1) {
        //    panic!("Buffer overflow!");
        // }

        self.elements[self.head] = Some(item);
        self.head = (self.head + 1) % self.max_elements;
        self.size += 1;
    }

    /// Returns the size of the buffer.
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Returns whether the buffer is empty or not.
    pub fn empty(&self) -> bool {
        return self.size == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // size tests
    #[test]
    fn ringbuffer_size_no_items_returns_0() {
        let buf = RingBuffer::<i32>::new(1);
        let expected = 0;
        let actual = buf.size();
        assert_eq!(expected, actual);
    }

    #[test]
    fn ringbuffer_empty_has_1_item_returns_1() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);

        let expected = 1;
        let actual = buf.size();
        assert_eq!(expected, actual);
    }

    #[test]
    fn ringbuffer_empty_has_3_item_returns_3() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);
        buf.push(1);
        buf.push(1);

        let expected = 3;
        let actual = buf.size();
        assert_eq!(expected, actual);
    }

    // empty tests
    #[test]
    fn ringbuffer_empty_no_items_returns_true() {
        let buf = RingBuffer::<i32>::new(1);
        let expected = true;
        let actual = buf.empty();
        assert_eq!(expected, actual);
    }

    #[test]
    fn ringbuffer_empty_has_items_returns_false() {
        let mut buf = RingBuffer::<i32>::new(1);
        buf.push(1);

        let expected = false;
        let actual = buf.empty();
        assert_eq!(expected, actual);
    }
}
