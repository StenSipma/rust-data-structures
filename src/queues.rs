pub mod priority_queue {
    use crate::linkedlist::LinkedList;

    /// Priority queue, with increasing order based on a linked list
    pub struct PriorityQueue<T> {
        list: LinkedList<T>,
    }

    impl<T> PriorityQueue<T>
    where
        T: Copy + PartialOrd,
    {
        pub fn new() -> Self {
            Self {
                list: LinkedList::new(),
            }
        }

        /// Add data (in increasing order) to the priority queue.
        ///
        /// ```
        /// let mut queue = data_structures::queues::priority_queue::PriorityQueue::new();
        /// queue.insert(1);
        /// queue.insert(3);
        /// queue.insert(2);
        /// let list: Vec<i32> = queue.collect(); // convert to a vec
        /// assert_eq!(list, vec![1, 2, 3]);
        /// ```
        pub fn insert(&mut self, data: T) {
            insert_inorder(&mut self.list, data)
        }

        /// Remove data in increasing order from the queue
        ///
        /// When the queue is empty, None is returned.
        ///
        /// ```
        /// let mut queue = data_structures::queues::priority_queue::PriorityQueue::new();
        /// queue.insert(1);
        /// queue.insert(3);
        /// queue.insert(2);
        ///
        /// assert_eq!(queue.pop(), Some(1));
        /// assert_eq!(queue.pop(), Some(2));
        /// assert_eq!(queue.pop(), Some(3));
        /// assert_eq!(queue.pop(), None);
        /// ```
        pub fn pop(&mut self) -> Option<T> {
            self.list.pop()
        }
    }

    impl<T> Iterator for PriorityQueue<T>
    where
        T: Copy + PartialOrd,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.list.next()
        }
    }

    // Helper function for inserting items in order in the LinkedList
    fn insert_inorder<T: Copy + PartialOrd>(ll: &mut LinkedList<T>, data: T) {
        match ll.0 {
            None => ll.append(data),
            Some((it, ref mut child)) => {
                if data >= it {
                    insert_inorder(child, data)
                } else {
                    ll.insert_here(data)
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn init_test() {
            let queue: PriorityQueue<i32> = PriorityQueue::new();
            assert!(queue.list.peek().is_none());
        }

        #[test]
        fn insert_test() {
            let mut queue = PriorityQueue::new();
            queue.insert(1);
            assert_eq!(queue.list.peek(), Some(1));
        }

        #[test]
        fn insert_order_test() {
            let mut queue = PriorityQueue::new();
            queue.insert(1);
            queue.insert(3);
            queue.insert(2);
            assert_eq!(queue.pop(), Some(1));
            assert_eq!(queue.pop(), Some(2));
            assert_eq!(queue.pop(), Some(3));
            assert_eq!(queue.pop(), None);
        }
    }
}

pub mod queue {
    /// The default capacity a queue gets when it is initialized
    const DEFAULT_INIT_QUEUE_CAPACITY: usize = 32;

    #[derive(Debug)]
    pub struct Queue<T> {
        list: Vec<T>,
        head: usize,
        tail: usize,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue::with_capacity(DEFAULT_INIT_QUEUE_CAPACITY)
        }

        /// Initialize a Queue with a custom capacity
        ///
        /// This is mostly useful if you know for certain the queue is going to
        /// get large, or remain (very) small.
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                list: Vec::with_capacity(capacity),
                head: 0,
                tail: 0,
            }
        }

        /// Adds an item to the queue (FIFO)
        ///
        /// The data is moved into the queue, so clone/copy if you need it.
        ///
        /// ```
        /// let mut queue = data_structures::queues::queue::Queue::new();
        /// queue.enqueue(1);
        /// queue.enqueue(2);
        /// queue.enqueue(3);
        /// assert_eq!(queue.dequeue(), Some(1));
        /// assert_eq!(queue.dequeue(), Some(2));
        /// assert_eq!(queue.dequeue(), Some(3));
        /// assert_eq!(queue.dequeue(), None);
        /// ```
        pub fn enqueue(&mut self, data: T) {
            if !self.has_space() {
                self.resize();
            }
            // self.list.insert(self.tail, data);
            if self.list.len() > self.tail {
                self.list[self.tail] = data;
            } else {
                self.list.insert(self.list.len(), data);
            }
            self.incr_tail();
        }

        /// Removes an item from the queue (FIFO)
        ///
        /// Returns `None` if the queue is empty
        ///
        /// ```
        /// let mut queue = data_structures::queues::queue::Queue::new();
        /// queue.enqueue(1);
        /// queue.enqueue(2);
        /// queue.enqueue(3);
        /// assert_eq!(queue.dequeue(), Some(1));
        /// assert_eq!(queue.dequeue(), Some(2));
        /// assert_eq!(queue.dequeue(), Some(3));
        /// assert_eq!(queue.dequeue(), None);
        /// ```
        pub fn dequeue(&mut self) -> Option<T> {
            if self.empty() {
                None
            } else {
                let dummy = unsafe {
                    // We swap the item at head with a zero value of type T
                    let mut dummy = std::mem::zeroed();
                    let it = self.list.get_unchecked_mut(self.head);
                    std::mem::swap(it, &mut dummy);
                    dummy
                };
                self.incr_head();
                Some(dummy)
            }
        }

        /// Checks if there are items in the queue
        ///
        /// ```
        /// let mut queue = data_structures::queues::queue::Queue::new();
        /// assert!(queue.empty());
        /// queue.enqueue(1);
        /// assert!(!queue.empty());
        /// ```
        pub fn empty(&self) -> bool {
            self.head == self.tail
        }

        /// The number of items in the queue
        ///
        /// ```
        /// let mut queue = data_structures::queues::queue::Queue::new();
        /// queue.enqueue(1);
        /// queue.enqueue(1);
        /// queue.enqueue(1);
        /// assert_eq!(queue.len(), 3);
        /// ```
        pub fn len(&self) -> usize {
            if self.head > self.tail {
                self.list.capacity() - self.head + self.tail
            } else {
                self.tail - self.head
            }
        }

        // private helper functions

        fn has_space(&self) -> bool {
            self.head != (self.tail + 1) % self.list.capacity()
        }

        fn incr_head(&mut self) {
            self.head = (self.head + 1) % self.list.capacity();
        }

        fn incr_tail(&mut self) {
            self.tail = (self.tail + 1) % self.list.capacity();
        }

        /// Double the capacity of the interal list
        ///
        /// Creates a new vector with double the capacity and moves all items
        /// from the old list into it.
        fn resize(&mut self) {
            // make new vector with twice the capacity
            let mut new_list = Vec::with_capacity(self.list.capacity() * 2);
            // move items into this vector
            if self.head <= self.tail {
                for i in self.list.drain(self.head..self.tail) {
                    new_list.insert(new_list.len(), i);
                }
            } else {
                for i in self.list.drain(self.head..) {
                    new_list.insert(new_list.len(), i);
                }
                for i in self.list.drain(..self.tail) {
                    new_list.insert(new_list.len(), i);
                }
            }
            self.list = new_list;
            self.head = 0;
            self.tail = self.list.len();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn init_test() {
            let q: Queue<i32> = Queue::new();
            assert_eq!(q.list.capacity(), DEFAULT_INIT_QUEUE_CAPACITY);
            assert_eq!(q.head, 0);
            assert_eq!(q.tail, 0);
        }

        #[test]
        fn enqueue_test() {
            let mut q = Queue::new();
            q.enqueue(1);
            assert_eq!(q.head, 0);
            assert_eq!(q.tail, 1);
            assert_eq!(q.list.get(0), Some(&1));
        }

        #[test]
        fn dequeue_test() {
            let mut q = Queue::new();
            q.enqueue(1);
            assert_eq!(q.dequeue(), Some(1));
            assert_eq!(q.dequeue(), None);
            assert_eq!(q.head, 1);
            assert_eq!(q.tail, 1);
        }

        #[test]
        fn fifo_test() {
            let mut q = Queue::new();
            q.enqueue(1);
            q.enqueue(2);
            q.enqueue(3);
            q.enqueue(4);
            assert_eq!(q.dequeue(), Some(1));
            assert_eq!(q.dequeue(), Some(2));
            assert_eq!(q.dequeue(), Some(3));
            assert_eq!(q.dequeue(), Some(4));
            assert_eq!(q.dequeue(), None);
        }

        #[test]
        fn wrapping_index_test() {
            let mut q = Queue::with_capacity(3);
            q.enqueue(1); // tail = 1
            assert_eq!(q.dequeue(), Some(1));

            q.enqueue(2); // tail = 2
            assert_eq!(q.dequeue(), Some(2));

            q.enqueue(3); // tail = 0
            assert_eq!(q.dequeue(), Some(3));

            assert_eq!(q.head, 0);
            assert_eq!(q.tail, 0);

            q.enqueue(4); // tail = 1
            assert_eq!(q.dequeue(), Some(4));
        }

        #[test]
        fn resize_test() {
            let mut q: Queue<i32> = Queue::new();
            assert_eq!(q.list.capacity(), DEFAULT_INIT_QUEUE_CAPACITY);
            q.resize();
            assert_eq!(q.list.capacity(), DEFAULT_INIT_QUEUE_CAPACITY * 2);
            assert_eq!(q.head, 0);
            assert_eq!(q.tail, 0);
        }

        #[test]
        fn resize_with_items_test() {
            let mut q: Queue<i32> = Queue::new();
            assert_eq!(q.list.capacity(), DEFAULT_INIT_QUEUE_CAPACITY);
            q.enqueue(1);
            q.resize();
            assert_eq!(q.list.capacity(), DEFAULT_INIT_QUEUE_CAPACITY * 2);
            assert_eq!(q.head, 0);
            assert_eq!(q.tail, 1);
        }

        #[test]
        fn resize_trigger_test() {
            let mut q = Queue::with_capacity(3);
            q.enqueue(1);
            q.enqueue(2);
            assert_eq!(q.list.capacity(), 3);
            q.enqueue(3); // resize here
            assert_eq!(q.list.capacity(), 6);
        }

        #[test]
        fn resize_lifo_test() {
            let mut q = Queue::with_capacity(3);
            q.enqueue(1);
            q.enqueue(2);
            q.enqueue(3); // resize here
            q.enqueue(4);
            q.enqueue(5);
            q.enqueue(6);
            assert_eq!(q.dequeue(), Some(1));
            assert_eq!(q.dequeue(), Some(2));
            assert_eq!(q.dequeue(), Some(3));
            assert_eq!(q.dequeue(), Some(4));
            assert_eq!(q.dequeue(), Some(5));
            assert_eq!(q.dequeue(), Some(6));

            let mut q = Queue::with_capacity(3);
            q.enqueue(1);
            q.enqueue(2);
            assert_eq!(q.dequeue(), Some(1));
            assert_eq!(q.dequeue(), Some(2));
            q.enqueue(3);
            q.enqueue(4);
            q.enqueue(5); // resize here
            q.enqueue(6);
            assert_eq!(q.dequeue(), Some(3));
            assert_eq!(q.dequeue(), Some(4));
            assert_eq!(q.dequeue(), Some(5));
            assert_eq!(q.dequeue(), Some(6));
            q.enqueue(7);
            assert_eq!(q.dequeue(), Some(7));
        }

        #[test]
        fn empty_test() {
            let mut q = Queue::new();
            assert!(q.empty());
            q.enqueue(1);
            assert!(!q.empty());
            q.dequeue();
            assert!(q.empty());
        }

        #[test]
        fn len_test() {
            let mut q = Queue::new();
            assert_eq!(q.len(), 0);
            q.enqueue(1);
            assert_eq!(q.len(), 1);
            q.dequeue();
            assert_eq!(q.len(), 0);
            for i in 0..30 {
                q.enqueue(i);
            }
            for _ in 0..16 {
                q.dequeue();
            }
            for i in 0..16 {
                q.enqueue(i);
            }
            assert!(q.tail < q.head);
            assert_eq!(q.len(), 30);
        }
    }
} /* queue */
