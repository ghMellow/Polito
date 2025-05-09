#![allow(warnings)]

pub mod mem_inspect {

    // dump object info:
    // size, address, bytes
    pub fn dump_object<T>(obj: &T) {
        let ptr = obj as *const T as *const u8;
        let _size = size_of::<T>();
        let _ptr = ptr as usize;
        println!("Object size: {_size}; address: {_ptr:x}");

        dump_memory(ptr, _size);
    }

    // dump memory info
    pub fn dump_memory(start: *const u8, size: usize) {
        let bytes = unsafe { std::slice::from_raw_parts(start, size) };

        println!("Bytes:");
        for (i, byte) in bytes.iter().enumerate() {
            print!("{:02x} ", byte);
            if i % 8 == 7 {
                println!();
            }
        }
        println!()
    }

    #[test]
    fn dump_object_example() {
        let s = "hello".to_string();
        dump_object(&s);

        let b = Box::new(s);
        // before running try to answer:
        // 1. what is the size of b?
        // 2. what is the content of b?
        dump_object(&b);

        // how to the the pointer of the wrapped object?
        let ptr = b.as_ref() as *const String as *const u8;
        println!("Pointer: {ptr:?}");

        assert!(true);
    }
}


pub mod List1 {
    use std::mem;

    pub enum Node<T> {
        Cons(T, Box<Node<T>>),
        Nil,
    }

    pub struct List<T> {
        /// Usiamo mem::replace perchè non si può spostare un elemento della struttura. O si sposta tutto o niente
        /// mentre modificare un elemento solo si può fare
        head: Node<T>,
    }

    // Definisci la struttura ListIter che conterrà un riferimento al nodo corrente
    pub struct ListIter<'a, T> {
        current: &'a Node<T>,
    }

    impl<T: std::fmt::Display> List<T> {
        pub fn new() -> Self {
            List{ head: Node::Nil } // Nil è equivalente di Null
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // problem:
        // 1. you need to build a new list Node (elem: elem, self.head)
        // 2. but you can't move self.head, because self.head would be undefined
        // 3. you can't copy it either, because Box can't be copied
        // solution: use mem::replace to move the value of self.head into a new variable and replace it with Nil
        // 4. let self.head point to the new created node
        pub fn push(&mut self, elem: T) {
            let old_head = mem::replace(&mut self.head, Node::Nil);
            self.head = Node::Cons(elem, Box::new(old_head));
        }

        // pop the first element of the list and return it
        pub fn pop(&mut self) -> Option<T> {
            let enum_pair = mem::replace(&mut self.head, Node::Nil);

            /// Ricorda! Gli 'enum' vanno acceduti con un pattern matching
            match enum_pair {
                Node::Cons(head, tail) => {
                    self.head = *tail; // asterisco per deferenziare il box nel heap
                                       // è un nodo enum ovvero facendo questa assegnazione in automatico avrò testa e coda
                    Some(head)
                }
                Node::Nil => {None}
            }
        }

        // return a reference to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            /// Ricorda! Gli 'enum' vanno acceduti con un pattern matching
            match &self.head {
                Node::Cons(head, tail) => {
                    Some(head)
                },
                Node::Nil => None,
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an iterator over the list values
        pub fn iter(&self) -> ListIter<T> {
            /// Metodo per ottenere un iteratore immutabile della lista
            ListIter{ current: &self.head }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> where T: Clone{
            let mut temp_res = Vec::new();
            let mut res = List::new();
            let mut count = 0;

            let mut iter = self.iter(); // create iterator once

            while let Some(next) = iter.next() {
                if count == n {
                    break;
                }
                temp_res.push(next.clone());
                count += 1;
            }

            // reverse the vector before pushing to maintain order
            temp_res.reverse();

            for value in temp_res {
                res.push(value);
            }

            res
        }
    }

    // Implementazione del trait Iterator per ListIter
    impl<'a, T> Iterator for ListIter<'a, T> {
        // Il tipo di elemento che l'iteratore produce
        type Item = &'a T;

        // Metodo principale dell'iteratore che restituisce il prossimo elemento
        fn next(&mut self) -> Option<Self::Item> {
            match self.current {
                Node::Cons(value, next) => {
                    // Sposta il puntatore al prossimo nodo
                    self.current = next;
                    // Restituisci un riferimento al valore corrente
                    Some(value)
                },
                Node::Nil => None, // Fine della lista
            }
        }
    }
}

pub mod List2 {

    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>; // alias

    pub struct List<T> {
        head: NodeLink<T>,
    }

    /// Reference & al valore corrente della lista
    pub struct ListIter<'a, T> {
        current: &'a NodeLink<T>,
    }

    // for this implementation, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        // same methods as List1
        pub fn new() -> Self {
            List{ head: None }
        }
        pub fn push(&mut self, elem: T) {
            let mut old_head = self.head.take();
            self.head = NodeLink::Some(Box::new(Node{elem: elem, next: old_head}))
        }
        pub fn pop(&mut self) -> Option<T> {

            // if let Some(value) = self.head.take() {
            //     self.head = value.next;
            //     Some(value.elem)
            // } else {
            //     None
            // }

            match self.head.take() { /// Prendo val e metto none con take
                None => {None}
                Some(value) => {
                    self.head = value.next; /// Nuovo val
                    Some(value.elem)
                }
            }
        }
        pub fn peek(&self) -> Option<&T> {
            match &self.head{
                None => {None}
                Some(value) => {
                    Some(&value.elem)
                }
            }
        }

        pub fn iter(&self) -> ListIter<T> {
            /// Metodo per ottenere un iteratore immutabile della lista
            ListIter { current: &self.head }
        }
        pub fn take(&mut self, n: usize) -> List<T> where T: Clone{
            let mut res = List::new();
            let mut temp_res = Vec::new();

            for (count, next) in self.iter().enumerate() {
                if count == n {
                    break;
                }

                temp_res.push(next.clone());
            }

            temp_res.reverse();
            for value in temp_res {
                res.push(value);
            }

            res
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current {
                Some(nodeLink) => {
                    self.current = &nodeLink.next;
                    Some(&nodeLink.elem)
                },
                None => {None}
            }
        }
    }
}

pub mod dlist {
    // problema: nello stack non puoi avere più puntatori che puntano allo stesso elemento nel heap
    // usare i puntatori dinamici
// *****
// double linked list suggestions:
// the node has both a next and a prev link

    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use crate::es0501::List1::Node::Nil;

    struct DNode<T> {
        elem: T,
        prev: NodeBackLink<T>,  // which type do we use here?
        next: NodeLink<T>, // which type do we use here?
    }
    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>;
    type NodeBackLink<T> = Option<Weak<RefCell<DNode<T>>>>;

    struct DList<T> {
        head: NodeLink<T>,
        tail: NodeBackLink<T>
    }

// use Rc, since we need more than one reference to the same node. 
// You need to both strong and weak references

// For mutating the list and changing the next and prev fields we also need to be able to mutate the node, 
// therefore we can use RefCell too (as for the tree at lesson)

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// hint for pop: you can return either a reference to the value or take the value out of the Rc, 
// but usually it is not possible to take out the value from an Rc since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc
// otherwise you can impose the COPY trait on T 

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

    impl<T> DList<T> {
        pub fn new() -> Self {
            DList{ head: None, tail: None }
        }
        pub fn push_front(&mut self, elem: T) {
            let new_head = Rc::new(RefCell::new(DNode {
                elem,
                prev: None,
                next: self.head.clone(),
            }));

            // Aggiorna il prev del vecchio head, se esiste
            if let Some(ref old_head) = self.head {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
            } else {
                // Lista vuota: aggiorna anche il tail (deve essere un Weak pointer)
                self.tail = Some(Rc::downgrade(&new_head));
            }

            // Aggiorna il head con il nuovo nodo
            self.head = Some(new_head);
        }

        pub fn pop_front(&mut self) -> Option<T> {
            // Prendiamo possesso del nodo head
            self.head.take().and_then(|old_head| {
                // Estrai il valore interno
                let mut old_head_ref = old_head.borrow_mut();

                // Per prendere possesso dell'elemento, usiamo std::mem::replace
                // che sostituisce elem con un valore dummy temporaneo
                let old_elem = std::mem::replace(&mut old_head_ref.elem, unsafe { std::mem::zeroed() });

                // Aggiorniamo head al prossimo nodo
                match old_head_ref.next.take() {
                    Some(new_head) => {
                        // Il nuovo head non deve avere prev
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                    }
                    None => {
                        // La lista è ora vuota
                        self.tail = None;
                    }
                }

                // Rilascia il borrow_mut prima della fine dello scope
                drop(old_head_ref);

                // A questo punto, old_head viene droppato, poiché non lo usiamo più
                Some(old_elem)
            })
        }

        pub fn push_back(&mut self, elem: T) {
            // Crea il nuovo nodo
            let new_node = Rc::new(RefCell::new(
                DNode {
                    elem: elem,
                    prev: self.tail.clone(), // Mantiene il riferimento al vecchio tail
                    next: None,
                }
            ));

            // Se c'è un vecchio tail, collega il suo next al nuovo nodo
            if let Some(ref old_tail) = self.tail {
                if let Some(old_tail_strong) = old_tail.upgrade() {
                    old_tail_strong.borrow_mut().next = Some(new_node.clone());
                }
            } else {
                // Se non c'è un vecchio tail, la lista è vuota
                // quindi il nuovo nodo diventa anche head
                self.head = Some(new_node.clone());
            }

            // Il nuovo nodo diventa il nuovo tail
            self.tail = Some(Rc::downgrade(&new_node));
        }
    }

    /**

    TEST

    **/

    #[test]
    fn test_push_front() {
        let mut list = DList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_back() {
        let mut list = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_mixed_push() {
        let mut list = DList::new();
        list.push_front(1); // list: [1]
        list.push_back(2);  // list: [1, 2]
        list.push_front(0); // list: [0, 1, 2]

        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_empty_list() {
        let mut list: DList<i32> = DList::new();
        assert_eq!(list.pop_front(), None);
    }
}