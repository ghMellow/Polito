pub mod es0501;
mod es0502;

#[cfg(test)]
pub mod test_list_1{
    use super::es0501::List1::{List};

    #[test]
    fn test_push_and_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);

        list.push(10);
        assert_eq!(list.peek(), Some(&10));

        list.push(20);
        assert_eq!(list.peek(), Some(&20));
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_take() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut new_list = list.take(2);
        println!("{:?}", new_list.pop());
        println!("{:?}", new_list.pop());
        println!("{:?}", new_list.pop());
    }
}
#[cfg(test)]
pub mod test_list_2{

    use super::es0501::List2::{List};

    #[test]
    fn test_push_and_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        list.push(2);
        assert_eq!(list.peek(), Some(&2));
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.push(10);
        list.push(20);

        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3); // List is now [3, 2, 1]

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_take() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut new_list = list.take(2);
        println!("{:?}", new_list.pop());
        println!("{:?}", new_list.pop());
        println!("{:?}", new_list.pop());
    }
}
