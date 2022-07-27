/// Singly linked list.
#[derive(Clone)]
pub struct LinkedList<T>(pub(super) Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> where T: Copy {
    pub fn new() -> Self {
        Self(None)
    }

    /// Add data to the end of the list
    ///
    /// ```
    /// let mut ll = data_structures::linkedlist::LinkedList::from_iter(vec![1, 2]);
    /// ll.append(3);
    /// let list: Vec<i32> = ll.collect();
    /// assert_eq!(list, vec![1, 2, 3]);
    /// ```
    pub fn append(&mut self, data: T) {
        match self.0 {
            Some(ref mut ll) => ll.1.append(data),
            None => self.0 = Some((data, Box::new(LinkedList::new())))
        };
    }

    /// Add data to the front of the list
    ///
    /// ```
    /// let mut ll = data_structures::linkedlist::LinkedList::from_iter(vec![1, 2]);
    /// ll.push(3);
    /// let list: Vec<i32> = ll.collect();
    /// assert_eq!(list, vec![3, 1, 2]);
    /// ```
    pub fn push(&mut self, data: T) {
        let mut new_ll = LinkedList::new();
        new_ll.0 = self.0.take();
        self.0 = Some((data, Box::new(new_ll)))
    }

    /// Remove and return the first value in the list in an Option
    ///
    /// When the list is empty, None is returned.
    ///
    /// ```
    /// let mut ll = data_structures::linkedlist::LinkedList::from_iter(vec![1, 2]);
    /// assert_eq!(ll.pop(), Some(1));
    /// assert_eq!(ll.pop(), Some(2));
    /// assert_eq!(ll.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let (data, child) = self.0.as_mut()?; // borrow the current value in self
        let data = *data; // copy the data value

        // magic
        let mut dummy = Box::new(LinkedList::new());
        std::mem::swap(child, &mut dummy);
        *self = *dummy;

        Some(data)
    }

    /// Inspect the first value in the list without removing it
    ///
    /// When the list is empty, None is returned.
    ///
    /// ```
    /// let ll = data_structures::linkedlist::LinkedList::from_iter(vec![1, 2]);
    /// assert_eq!(ll.peek(), Some(1));
    /// assert_eq!(ll.peek(), Some(1));
    ///
    /// let ll = data_structures::linkedlist::LinkedList::<i32>::new();
    /// assert_eq!(ll.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<T> {
        Some(self.0.as_ref()?.0)
    }

    /// Insert data at specific index in the list
    ///
    /// When the index is out of range, the value is added at the end.
    ///
    /// ```
    /// let mut ll = data_structures::linkedlist::LinkedList::from_iter(vec![1, 3]);
    ///
    ///
    /// ll.insert(2, 1); // insert number 2 at index 1
    /// let list: Vec<i32> = ll.clone().collect();
    /// assert_eq!(list, vec![1, 2, 3]);
    ///
    /// ll.insert(-1, 0); // insert number -1 at the beginning
    /// let list: Vec<i32> = ll.clone().collect();
    /// assert_eq!(list, vec![-1, 1, 2, 3]);
    ///
    /// ll.insert(5, 99); // insert number -1 at the end (99 is out of range)
    /// let list: Vec<i32> = ll.clone().collect();
    /// assert_eq!(list, vec![-1, 1, 2, 3, 5]);
    /// ```
    pub fn insert(&mut self, data: T, n: usize) {
        match self.0 {
            None => self.append(data),
            Some((_, ref mut child)) => {
                if n > 0 {
                    child.insert(data, n-1);
                } else {
                    self.insert_here(data)
                }
            }
        }
    }

    pub (super) fn insert_here(&mut self, data: T) {
        // let next = self;
        let mut new = LinkedList::new();
        new.append(data);

        std::mem::swap(self, &mut new);
        let mut child = self.0.as_mut().unwrap();
        child.1 = Box::new(new)
    }

    fn from_helper<I>(&mut self, iter: &mut I) where I: Iterator<Item=T> {
        match iter.next() {
            None => return,
            Some(item) => {
                self.append(item);
                self.0.as_mut().unwrap().1.from_helper(iter);
            }
        };
    }
}

impl<T> FromIterator<T> for LinkedList<T> where T: Copy {
    fn from_iter<I>(list: I) -> Self 
    where 
        I: std::iter::IntoIterator<Item = T> 
    {
        let mut ll = LinkedList::new();
        ll.from_helper(&mut list.into_iter());
        ll
    }

}

impl<T> Iterator for LinkedList<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (data, child) = self.0.as_mut()?;
        let data = *data;
        let mut dummy = Box::new(LinkedList::new());
        std::mem::swap(child, &mut dummy);
        *self = *dummy;
        Some(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ll_test() {
        let ll: LinkedList<i32> = LinkedList::new();
        assert!(ll.0.is_none())
    }

    #[test]
    fn append_ll_int_test() {
        let mut ll = LinkedList(None);
        ll.append(1);
        assert!(ll.0.is_some());
        let child = ll.0.unwrap();
        assert_eq!(child.0, 1);
        assert!(child.1.0.is_none());
    }

    #[test]
    fn append_ll_str_test() {
        let mut ll = LinkedList(None);
        ll.append("abc");
        assert!(ll.0.is_some());
        let child = ll.0.unwrap();
        assert_eq!(child.0, "abc");
        assert!(child.1.0.is_none());
    }

    #[test]
    fn pop_test() {
       let mut ll = LinkedList::new();
       ll.push(1);
       ll.push(2);

       assert_eq!(ll.pop(), Some(2));
       assert_eq!(ll.pop(), Some(1));
       assert_eq!(ll.pop(), None);
       assert_eq!(ll.pop(), None);

       ll.append(5);
       assert_eq!(ll.pop(), Some(5));
    }

    #[test]
    fn peek_test() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.peek(), None);
        ll.push(0);
        assert_eq!(ll.peek(), Some(0));
    }

    #[test]
    fn insert_test() {
        let mut ll = LinkedList::new();

        ll.push(3);
        ll.push(1);
        ll.insert(2, 1);

        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(3));

        let mut ll = LinkedList::new();
        ll.insert(2, 0);
        assert_eq!(ll.pop(), Some(2));
    }

    #[test]
    fn from_test() {
        let lst = vec![1, 2, 3, 4];
        let mut ll = LinkedList::from_iter(lst);
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(4));
    }

    #[test]
    fn from_empty_test() {
        let lst: Vec<i32> = Vec::new();
        let mut ll = LinkedList::from_iter(lst);
        assert_eq!(ll.pop(), None);
    }

    #[test]
    fn from_map_test() {
        let lst = vec![1, 2, 3, 4];
        let mut ll: LinkedList<i32> = lst.into_iter().map(|x| { x*x }).collect();
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), Some(4));
        assert_eq!(ll.pop(), Some(9));
        assert_eq!(ll.pop(), Some(16));
    }

    #[test]
    fn push_append_test() {
        let mut ll = LinkedList::new();
        ll.push(2);
        ll.append(3);
        ll.push(1);
        ll.append(4);
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(4));
    }

    #[test]
    fn iterator_test() {
        let ll = LinkedList::from_iter(vec![0, 1, 2, 3, 4]);
        for (a, b) in ll.enumerate() {
            assert_eq!(a, b)
        }

        let ll = LinkedList::from_iter(vec![0, 1, 2, 3, 4]);
        let vec: Vec<i32> = ll.collect();
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
    }
}


/// The Stack LIFO data structure.
///
/// LIFO is Last In First Out, and this realised by only using the pop and push methods on a Linked
/// List
/// ```
/// let mut stack = data_structures::linkedlist::Stack::new();
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
/// assert_eq!(stack.pop(), Some(3));
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(1));
/// assert_eq!(stack.pop(), None);
/// ```
pub type Stack<T> = LinkedList<T>;
