use std::rc::Rc;

mod pattern_matchin;

#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    // --
    let mut i = 32;

    let mut r = i; // implementa tratto copy diventano due entità separate
    i += 1;
    r += 2;
    println!("r: {}", r);


    println!("i: {}, {}", i, r);

    // --
    let mut x = 5;

    let z = &mut x;
    println!("z: {}", z);

    let y = &x;
    println!("y: {}", y);
    //println!("z: {}", z); invalida, ci sarebbero due rif: uno immutabile e uno mutabile


    // --
    // per riferimento l'entità è la stessa.
    let mut i = 32;

    let mut r = &mut i; // al contrario se possesso preso per riferimento (mutabile opz)
    *r += 2;                     // l'esistenza di r è valida finchè il valore originale non cambia.
    println!("r: {}", r);        // O meglio finchè esistono riferimenti (mutabili o meno) in uso, il valore originale non è modificabile.

    i += 1; // r smette di esistere qui
    r = &mut i; // ricreo, se tolto uso di r sotto dà errore
    *r = 5;
    println!("r: {}", r);
    println!("i: {}", i);

    // --
    // Array, valori sullo stack
    let a = [1,2,3,4,5];
    let b = [0; 5]; // array statico da 5 elementi inizializzati a 0
    let fat_pointer = &a[0..3]; // dim conosciuta all'atto dell'esecuzione. slice è l'operazione fatta sul riferimento di a range [0, 3)

    // --
    // Slice
    let mut a = [ 1, 2, 3, 4 ]; // per rif mut a deve essere mutabile!
    let s2 = &mut a[0..2]; // no .clone esplicito perciò di default rust effettua passaggio di possesso!! a non più utilizzabile.
    s2[0] = 10;
    //println!("array {:?}", a);
    println!("slice {:?}", s2);

    let mut i = 32;
    let r = &mut i;
    println!("{}", *r); // è la deferenziazione a consumare l'oggetto no la print
    i = i+1;
    let r = &i;
    println!("{}", *r);

    // --
    let mut s = String::from("hello");

    let r = &mut s;
    //s.push('!');
    println!("{}", r);    // Ultimo uso di r
    // r "muore" qui

    // --
    // Vettore, valori sullo heap
    let vec1: Vec<u32> = Vec::new(); // :Vec<tipo specifico> e funz di libreria per istanziarlo
    let vec2 = Vec::from([1,2,3,4,5]); // con valori
    let vec3 = vec![1,2,3,4,5]; // typo equivalente per creare con valori già dentro
    println!("vec1: {:?}, {:?}, {:?}", vec1, vec2, vec3);

    // --
    let mut s1 = "hello".to_string();
    println!("s1: {}", s1); // print non consuma
    let s2 = s1;
    println!("s2: {}", s2); // s2: hello, in s1 c’è la stessa cosa:
    // ma NON è più accessibile
    //println!("s1.to_uppercase(): {}", s1.to_uppercase());

    // --
    let p1 = Punto { x: 1, y: 2 };
    let mut p2 = p1;
    // p1 è stato consumato e non è più accessibile
    // println!("Punto 1: {:?}", p1);
    println!("Punto 2: {:?}", p2);
    p2 = Punto { x: 3, y: 4 };
    println!("Punto 2 (modificato): {:?}", p2);

    // --
    let s = String::from("hello");
    takes_ownership(&s);
    println!("{}", s); // stampa hello

    // --
    fn cambia (par: &mut i32, val: i32)
    {
        *par = val;
    }
    let r:&mut i32;
    {
        let mut v = vec![1, 2, 3];
        let x = &v;
        // r = &mut v[1];
        println!("{:?}", x);
        //cambia(r, 100);
        v.push(4);
        println!("{:?}", v);
    }
    //cambia(r, 200);

    // --
    // lifetime
    fn confronta<'a> (str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }
    let mut s1 = String::from("hello");
    let s2 = String::from("world!");
    let risultato;
    risultato = confronta(&s1, &s2);
    // s1 = String::from("hello"); // intersezione primo invalidato invalida anche 'risultato'
    println!("La stringa più lunga è: {}", risultato);


    // --
    fn insert<'a>(vet: &mut Vec<&'a str>, s: &'a str) {
        vet.push(s);
        //println!("{}", s);
    }
    let mut v = Vec::<&str>::new();
    let binding = "Inserisco una stringa".to_string();
    insert(&mut v, & binding);
    println!("{:?}", v);

    // --
    // iteratori
    let numbers = vec![1,2,3,4,5,6,7,8];
    let res = numbers.iter().filter(|&x| x%2 == 0 ).zip('a'..'z');
    let last = res.clone().map(|(a,b)| { format!("{}-{}", b, a) }).last();

    println!("> last: {:?}", last);
    println!("> res count: {:?}", res.count());

    // --
    // Tratti
    #[derive(Debug)]
    struct S { i: i32 }
    impl From<i32> for S { fn from(i:i32) -> Self { S{ i } } }
    impl Clone for S { fn clone(&self) -> Self { S{ i: self.i } } }

    let mut vec = Vec::<S>::new();
    let s: S = 42.into();
    for i in 0..3 {
        vec.push(s.clone());
    }
    println!("{:?}", vec);

    // --
    // Chiusure
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // --
    let mut count = 0;
    let mut increment = || {
        count += 1; // Incrementiamo la variabile catturata
        println!("Il conteggio è: {}", count);
    };
    increment();

    //count += 1;
    println!("Il conteggio è: {}", count); // chiusure per possesso guardare var a cui sono assegnate. Qui la validità di increment  tale fino a che non viene usata count che è il vero possessore
    // increment(); // dopo il rif mut è invalido!

    // --
    let mut data = vec![1, 2, 3, 4, 5];
    let mut process_data = || {
        data.push(6);
        let sum: i32 = data.iter().sum();
        println!("La somma dei dati è: {}", sum);
    };
    // Chiamiamo la closure
    process_data();
    data.push(7);
    println!("{:?}", data);

    // --
    let range = 1..10;
    let f = || {range.count()}; // count implementa self come parametro, quindi prende il possesso di range e lo consuma
    let n1 = f();                    // move implicita, e metodo restituisce usize range si perde.
    //let n2 = f();

    // move esplicita:
    let v = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        // move è necessario per spostare `v` nel thread
        println!("{:?}", v);
    });
    handle.join().unwrap();

    // fnMut: ripetibile n volte e ricorda lo stato.
    let mut sum = 0;
    let mut chiusura = |x| { sum += x }; // ricorda che i parametri vengono passati al momento della chiamata, è sum ad essere passato come riferimento mutabile.
    chiusura(5);
    chiusura(5);
    println!("chiusura: {}", sum);

    // fn: ripetibile, con move
    let sum = String::from("ciao");
    let chiusura = move || println!("{}", sum); // sum usando move viene spostata di scope non esiste più al di fuori della chiusura
    //println!("chiusura: {}", sum); errore
    chiusura();
    chiusura(); // chiamabile più volte poichè non viene conusmata sum dentro la chiusura

    // --
    // Puntatori dinamici
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }
    let mut nipote1 = Rc::new(Node {
        value: 3,
        children: vec![],
    });
    let nipote2 = Rc::new(Node {
        value: 6,
        children: vec![],
    });
    let padre = Rc::new(Node {
        value: 9,
        children: vec![Rc::clone(&nipote1), Rc::clone(&nipote2)],
    });
    let nonno = Rc::new(Node {
        value: 27,
        children: vec![Rc::clone(&padre)],
    });
    match Rc::get_mut(&mut nipote1) { // MA NIPOTE HA STRONG_REFERENCE_COUNTER = 2 (padre e nonno)
        Some(v) => v.children.push(Rc::clone(&nonno)),
        None => println!("Non è possibile ottenere un riferimento mutabile."),
    }
    println!("{:#?}", nonno);


    // --
    // Thread
    let mut numero = 5;
    let handle = std::thread::spawn(move || {
        let x = 2;
        //numero += x;
        println!("Numero incrementato di {}. Nuovo valore: {}", x, numero);
        //numero
    });
    println!("numero old: {}", numero);
    let result = handle.join();
    match result {
        Ok(res) => {println!("Il risultato è {:?}", res);},
        Err(err) => {println!("Errore {:?}", err);}
    }

    // --
    let mut numero = 5;
    let numero_copy = numero; // Copia il valore
    let handle = std::thread::spawn(move || {
        let x = 2;
        println!("Numero incrementato di {}. Nuovo valore: {}", x, numero_copy);
    });

    // --
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..10 {
        let tx_clone = tx.clone();

        std::thread::scope(|s| {
            s.spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                tx_clone.send("test").unwrap();
            }); // <- Nota il punto e virgola!
        });

        /*std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            tx_clone.send("test").unwrap();
        } );*/
    }
    drop(tx);   // i cloni vengono chiusi dalla closure ma il creato va chiuso a mano.
                // ATTENZIONE! il ricevitore rimane in attesa finchè di messaggi finchè non vengono chiusi i TX !!!
    while let Ok(msg) = rx.recv() {
        println!("{}", msg);
    }

    // --
    use std::{
        sync::{Arc, Condvar, Mutex},
        thread::{sleep},
        time::Duration,
    };
    struct Counter {
        value: Mutex<u32>,
        condvar: Condvar,
    }
    let counter = Arc::new(Counter {
        value: Mutex::new(0),
        condvar: Condvar::new(),
    });
    let counter_clone = counter.clone();
    let counting_thread = std::thread::spawn(move || loop {
        sleep(Duration::from_millis(100));
        let mut value = counter_clone.value.lock().unwrap();
        *value += 1;
        counter_clone.condvar.notify_all();
        if *value >= 15 {
            break;
        }
    });
    // Wait until the value more or equal to 15
    let mut value = counter.value.lock().unwrap();
    value = counter.condvar.wait_while(value, |val| *val < 15).unwrap();
    println!("Condition met. Value is now {}.", *value);
    // Wait for counting thread to finish
    counting_thread.join().unwrap();
}


fn takes_ownership(some_string: &str) {
    let s = some_string.to_uppercase(); // il quale non consuma, &self
    println!("{}", s); // stampa HELLO
}