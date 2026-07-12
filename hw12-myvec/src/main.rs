use std::path::Iter;

pub struct MyVec<T> {
    data: [Option<T>; 32],
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            data: std::array::from_fn(|i| None),
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= self.data.len() {
            return Err(value);
        }

        self.data[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        let last_index = self.len;
        if last_index == 0 {
            return None;
        }

        let value = self.data[last_index - 1].take();
        self.len -= 1;
        value
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        if self.len == 0 {
            return true;
        }

        false
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            return None;
        }

        self.data[idx].as_ref()
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.len {
            return None;
        }

        self.data[idx].as_mut()
    }

    pub fn iter(&self) -> MyVecIter<'_, T> {
        MyVecIter { vec: self, pos: 0 }
    }

    pub fn clear(&mut self) {
       for i in 0..self.len {
        self.data[i].take();
       }

       self.len = 0;
    }
}

pub struct MyVecIter<'a, T> {
    vec: &'a MyVec<T>,
    pos: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.vec.len {
            return None;
        }

        let value = self.vec.data[self.pos].as_ref();
        self.pos += 1;

        value
    }
}

fn main() {
}

mod tests {
    use crate::MyVec;

    #[test]
    fn push_pop_basic() {
        let mut v: MyVec<i32> = MyVec::new();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn iter_borrows() {
        let mut v: MyVec<String> = MyVec::new();
        v.push("a".into()).unwrap();
        v.push("b".into()).unwrap();
        let collected: Vec<&String> = v.iter().collect();
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn capacity_overflow() {
        let mut v: MyVec<i32> = MyVec::new();
        for i in 0..32 {
            v.push(i).unwrap();
        }
        assert!(v.push(99).is_err());
    }

    #[test]
    fn drop_called() {
        use std::cell::Cell;
        use std::rc::Rc;

        #[derive(Debug)]
        struct Counter {
            counter: Rc<Cell<i32>>,
        }

        impl Drop for Counter {
            fn drop(&mut self) {
                self.counter.set(self.counter.get() + 1);
            }
        }

        let counter = Rc::new(Cell::new(0));

        let mut v: MyVec<Counter> = MyVec::new();
        for _ in 0..3 {
            v.push(Counter {
                counter: Rc::clone(&counter),
            })
            .unwrap();
        }

        assert_eq!(counter.get(), 0);

        v.clear();

        assert_eq!(counter.get(), 3);
        assert_eq!(v.len(), 0);
    }
}
