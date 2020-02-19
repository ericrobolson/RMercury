/// Data structure that uses a single, fixed-size buffer. Useful for data streams.
pub struct RingBuffer<T> {
    elements: Vec<Option<T>>,
    max_elements: usize,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> RingBuffer<T>
where
    T: Clone,
{
    /// Initialize a new Ring Buffer with the maximum number of elements.
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

    /// Returns the item at the front of the buffer.
    pub fn front(&self) -> &Option<T> {
        return &self.elements[self.tail];
    }

    /// Returns the item at a given index.
    pub fn item(&self, index: usize) -> &Option<T> {
        return &self.elements[(self.tail + index) % self.max_elements];
    }

    /// Returns the maximum number of items.
    pub fn max_elements(&self) -> usize {
        return self.max_elements;
    }

    /// Push the given item onto the buffer. Will panic if it overflows.
    pub fn push(&mut self, item: T) {
        if self.size == self.max_elements {
            panic!("Buffer overflow!");
        }

        self.elements[self.head] = Some(item);
        self.head = (self.head + 1) % self.max_elements;
        self.size += 1;
    }

    /// Pop the next item from the buffer.
    pub fn pop(&mut self) -> Option<T> {
        let element = self.elements[self.tail].clone();

        self.elements[self.tail] = None;
        self.tail = (self.tail + 1) % self.max_elements;

        if self.size > 0 {
            self.size -= 1;
        }

        return element;
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

    // new tests
    #[test]
    fn ringbuffer_new_initializes_proper_value() {
        let max_elements = 4;
        let buf = RingBuffer::<i32>::new(max_elements);

        assert_eq!(0, buf.size());
        assert_eq!(&None, buf.front());
        assert_eq!(max_elements, buf.max_elements());

        for i in 0..max_elements {
            assert_eq!(&None, buf.item(i));
        }
    }

    // front tests
    #[test]
    fn ringbuffer_front_no_items_returns_none() {
        let buf = RingBuffer::<i32>::new(4);

        let actual = buf.front();
        assert_eq!(&None, actual);
    }

    #[test]
    fn ringbuffer_front_single_item_returns_some() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(2);

        let actual = buf.front();
        assert_eq!(&Some(2), actual);
    }

    #[test]
    fn ringbuffer_front_multiple_items_returns_first_item() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);
        buf.push(2);

        let actual = buf.front();
        assert_eq!(&Some(1), actual);
    }

    // item tests
    #[test]
    fn ringbuffer_item_no_items_returns_none() {
        let max_elements = 4;

        let buf = RingBuffer::<i32>::new(max_elements);

        for i in 0..max_elements {
            let actual = buf.item(i);
            assert_eq!(&None, actual);
        }
    }

    #[test]
    fn ringbuffer_item_no_items_index_wraps_returns_none() {
        let max_elements = 4;

        let buf = RingBuffer::<i32>::new(max_elements);

        for i in 0..(max_elements * 2) {
            let actual = buf.item(i);
            assert_eq!(&None, actual);
        }
    }

    #[test]
    fn ringbuffer_item_multiple_items_returns_expected_item() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);
        buf.push(2);
        buf.push(3);

        let expected = &Some(1);
        let actual = buf.item(0);
        assert_eq!(expected, actual);

        let expected = &Some(2);
        let actual = buf.item(1);
        assert_eq!(expected, actual);

        let expected = &Some(3);
        let actual = buf.item(2);
        assert_eq!(expected, actual);

        let expected = &None;
        let actual = buf.item(3);
        assert_eq!(expected, actual);

        let expected = &Some(1);
        let actual = buf.item(4);
        assert_eq!(expected, actual);

        let expected = &Some(2);
        let actual = buf.item(5);
        assert_eq!(expected, actual);
    }

    // max_elements tests
    #[test]
    fn ringbuffer_max_elements_2_returns_2() {
        let buf = RingBuffer::<i32>::new(2);
        let expected = 2;
        let actual = buf.max_elements();
        assert_eq!(expected, actual);
    }

    #[test]
    fn ringbuffer_max_elements_23_returns_23() {
        let buf = RingBuffer::<i32>::new(23);
        let expected = 23;
        let actual = buf.max_elements();
        assert_eq!(expected, actual);
    }

    // push tests
    #[test]
    fn ringbuffer_push_no_items_adds_item() {
        let mut buf = RingBuffer::<i32>::new(2);
        buf.push(1);
        assert_eq!(1, buf.size());

        let expected = Some(1);
        let actual = buf.pop();
        assert_eq!(expected, actual);
    }

    #[test]
    fn ringbuffer_push_multiple_items_adds_item() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert_eq!(3, buf.size());

        let expected = Some(1);
        let actual = buf.pop();
        assert_eq!(expected, actual);

        let expected = Some(2);
        let actual = buf.pop();
        assert_eq!(expected, actual);

        let expected = Some(3);
        let actual = buf.pop();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn ringbuffer_push_overflow_panics() {
        let mut buf = RingBuffer::<i32>::new(1);
        buf.push(1);
        buf.push(1);
    }

    // pop tests
    #[test]
    fn ringbuffer_pop_no_items_returns_none() {
        let mut buf = RingBuffer::<i32>::new(1);
        let expected = None;
        let actual = buf.pop();

        assert_eq!(expected, actual);
        assert_eq!(0, buf.size())
    }

    #[test]
    fn ringbuffer_pop_single_item_returns_item() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);
        let expected = Some(1);
        let actual = buf.pop();

        assert_eq!(expected, actual);
        assert_eq!(0, buf.size())
    }

    #[test]
    fn ringbuffer_pop_multiple_items_returns_popped_item() {
        let mut buf = RingBuffer::<i32>::new(4);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        let expected = Some(1);
        let actual = buf.pop();

        assert_eq!(expected, actual);
        assert_eq!(2, buf.size())
    }

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
