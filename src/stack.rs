#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    pub fn push(&mut self, item: T) {
        self.size += 1;
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // iterations
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        // let mut iterator = Iter { stack: Vec::new() };
        // for item in self.data.iter() {
        //     iterator.stack.push(item);
        // }
        // iterator
        Iter {
            stack: self.data.iter().collect(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        // let mut iterator = IterMut { stack: Vec::new() };
        // for item in self.data.iter_mut() {
        //     iterator.stack.push(item);
        // }
        // iterator
        IterMut {
            stack: self.data.iter_mut().collect(),
        }
    }
}

//
#[derive(Debug)]
pub struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        self.0.size -= 1;
        self.0.data.pop()
    }
}
//
#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
//
#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

///////////////////////
#[test]
fn test_basic() {
    let mut s = setup_stack();
    assert_eq!(3, s.size);

    assert_eq!(3, s.pop().unwrap());
    assert_eq!(2, s.len());

    assert!(!s.is_empty());
    s.clear();
    assert_eq!(0, s.size);
    assert_eq!(0, s.data.len());
}

#[test]
fn test_peek() {
    let mut s = setup_stack();
    assert_eq!(3, *s.peek().unwrap());

    if let Some(top) = s.peek_mut() {
        *top = 4;
    }
    assert_eq!(4, *s.peek().unwrap());
}

#[test]
fn test_iter() {
    let mut s = setup_stack();
    let initial_sum = s.iter().sum::<i32>();
    assert_eq!(6, initial_sum);

    let mut addend = 0;
    for item in s.iter_mut() {
        *item += 1;
        addend += 1;
    }
    let post_sum = s.iter().sum::<i32>();

    assert_eq!(3, addend);
    assert_eq!(9, post_sum);
    assert_eq!(9, s.into_iter().sum::<i32>());
}

fn setup_stack() -> Stack<i32> {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    s
}
