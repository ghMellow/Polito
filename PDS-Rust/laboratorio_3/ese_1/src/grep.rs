/*"""
Scrivere I oppure T è la stessa cosa?

Sì, usare `I` o `T` è equivalente dal punto di vista del compilatore: sono solo nomi di tipi generici.
La differenza è semantica:
- `T` è generico e usato di default.
- `I` è spesso usato per indicare un iteratore (per convenzione).
- Altri esempi: `K`, `V` (key/value), `E` (error), `R` (reader), ecc.
Quindi:
✔ `struct MyStruct<T> where T: Iterator<Item = String>`
✔ `struct MyStruct<I> where I: Iterator<Item = String>`

Funzionano uguale, ma `I` rende più chiaro che si tratta di un iteratore.

"""*/

// to warm up: the define step by step an adapter for filtering even numbers

pub mod simple_even_iter {

    // (1) let start with a simple iterator adapter for just one type, "i32"
    // see the adapter pattern example in the pdf "Adapter Pattern..."
    struct EvenIter<I> {
        inner: I // hint: it's a generic type... here we don't care about bounds yet
    }

    impl<I> EvenIter<I> {
        fn new(iter: I) -> Self {
            EvenIter { inner: iter }
        }
    }

    impl<I> Iterator for EvenIter<I>
    where
        I: Iterator<Item = i32>  // here we need to define the bounds for the generic type
                                 // T it must be an iterator over i32
    {
        type Item = i32; // <== it will work just for i32

        fn next(&mut self) -> Option<Self::Item> {
            // uso di while al posto di if così facendo esegue next finchè non trova un valore valido
            // esce sia quando trova un pari che quando l'iteratore finisce None è un break del while.
            // Va implementato così poichè la funzione di test che lo chiama è un for e in generale le
            // strutture iterative quando vedono un None si interrompono. Perciò se ritornassi none se il
            // valore è dispari si ferma anche il for del test.
            while let Some(inner) = self.inner.next() {
                if inner % 2 == 0 {
                    return Some(inner);
                }
            }
            None
            /*
            self.inner.next().and_then(|inner| {
                (inner % 2 == 0).then_some(inner)
            })*/
        }
    }

    // if EvenIter works the test will compile and pass
    #[test]
    fn test_simple_even_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // why iter() does not work here?
        let it = EvenIter::new(v.into_iter());
        for i in it {
            println!("i: {}", i);
        }
    }

    // (2) now let's add the adapter to all Iterator<Item=i32> (adavanced)
    trait AddEvenIter: Iterator
    where
        Self: Sized
    {
        // add even() to anyone implementing this trait
        // usage: v.into_iter().even() ....
        fn even(self) -> EvenIter<Self>{
            EvenIter::new(self)
        }
    }

    // (3) add here the generic implementation, you can supply it for all the iterators
    // impl .... ?

    // in pratica AddEvenIter è un wrapper dell'iteratore EvenIter
    impl<T> AddEvenIter for T
    where
        T: Iterator<Item = i32> + Sized,
    {}

    #[test]
    fn test_adapter() {
        let v = vec![1,2,3,4,5];
        for i in v.into_iter().even() {
            println!("{}", i);
        }
    }


    pub mod even_iter {
        // (4) more adavanced: implement for all integer types
        // => install the external crate "num" to have some Traits identifying all number types
        use num;

        // the generic parameters I and U are already defined for you in the struct deinition
        // (5) write in a comment in plain english the meaning of the generic parameters
        // and their constraints:

        // The generic parameters in this struct represent:
        // - I: The iterator type we're wrapping/adapting. This must implement the Iterator trait.
        // - U: The type of items produced by the iterator.
        //
        // Constraints:
        // - I must be an iterator that yields items of type U (indicated by I: Iterator<Item = U>)
        // - Later in the implementation, U will need to satisfy additional constraints
        //   (being an Integer and supporting Copy) to allow the even number filtering logic.
        //
        // This design allows EvenIter to work with any iterator regardless of its concrete type,
        // as long as it produces items that can be checked for being even numbers.
        struct EvenIter<I, U>
            where
            I: Iterator<Item = U> {
            iter: I
        }

        impl<I,U> Iterator for EvenIter<I, U>
            where
            U: num::Integer + Copy,
            I: Iterator<Item = U> {
            type Item = U;

            fn next(&mut self) -> Option<Self::Item> {
                // Continua a richiedere il prossimo elemento all'iteratore interno
                // finché non trovi un numero pari o l'iteratore termina
                while let Some(value) = self.iter.next() {
                    // Verifica se il valore è pari usando il trait num::Integer
                    if value.is_even() {
                        return Some(value);
                    }
                }
                // Se l'iteratore interno è terminato o non sono stati trovati altri numeri pari
                None
            }

        }

        // (6) once implemented, the test will compile and pass
        #[test]
        fn test_even_iter() {
            let v: Vec<u64> = vec![1, 2, 3, 4, 5];
            let it = EvenIter { iter: v.into_iter() };
            for i in it {
                println!("i: {}", i);
            }
        }

    }


    // finally let's implement the grep command
    // (1) install the "walkdir" crate for walking over directories using an iterator
    // install also the "regex" crate for regular expressions
    use walkdir;

    // (2) define the match result
    struct Match {
        file: String,
        line: usize,
        text: String
    }

    // (3) test walkdir iterator, see how errors are handled
    #[test]
    fn test_walk_dir() {
        let wdir = walkdir::WalkDir::new("/tmp");
        for entry in wdir.into_iter() {
            match entry {
                Ok(entry) => println!("{:?}", entry),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    // (3) define the grep adapter for the iterator
    // add anything you need implement it
    struct GrepIter {
        inner: walkdir::IntoIter,
    }

    impl GrepIter {
        fn new(iter: walkdir::IntoIter) -> Self {
            GrepIter { inner: iter }
        }
    }

    impl Iterator for GrepIter {

        type Item = Result<Match, walkdir::Error>;

        fn next(&mut self) -> Option<Self::Item> {
        }
    }

    #[test]
    fn test_grep_iter() {
        let wdir = walkdir::WalkDir::new("/tmp");
        let grep_iter = GrepIter::new(wdir.into_iter());
        for entry in grep_iter {
            match entry {
                Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
                Err(e) => { println!("Error: {}", e); }
            }
        }
    }

    // (5) add grep() to IntoIter  (see the first example in EvenIter for i32)

    trait Grep {
        //....
    }

    /*
    #[test]
    fn test_grep() {
        let wdir = walkdir::WalkDir::new("/tmp");
        let grep_iter = wdir.into_iter().grep();
        for entry in grep_iter {
            match entry {
                Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
                Err(e) => { println!("Error: {}", e); }
            }
        }
    }*/

}


