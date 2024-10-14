/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str>
    where
        T: Clone,
    {
        self.elements.first().ok_or("Queue is empty")
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T>
where
    T: Clone,
{
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
            Err("Stack is empty")
        } else {
            // 将 q1 中的所有元素除了最后一个移动到 q2
            let mut temp = None;
            loop {
                match self.q1.dequeue() {
                    Ok(elem) => {
                        if self.q1.is_empty() {
                            temp = Some(elem);
                            break;
                        } else {
                            self.q2.enqueue(elem);
                        }
                    },
                    Err(_) => break,
                }
            }
            // 将 q2 中的所有元素移回 q1
            while let Ok(elem) = self.q2.dequeue() {
                self.q1.enqueue(elem);
            }
            // temp 中的元素是栈顶元素
            Ok(temp.expect("Stack is empty"))
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}