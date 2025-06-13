// 1. Match normale con enum
enum Color {
    Red,
    Green,
    Blue,
}

fn example1() {
    let c = Color::Green;

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

// 2. Con Option<T>
fn example2() {
    let maybe_number = Some(10);

    match maybe_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
}

// 3. Con Result<T, E>
fn example3() {
    let result: Result<i32, &str> = Ok(5);

    match result {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// 4. Con pattern binding (if let)
fn example4() {
    let maybe = Some("ciao");

    if let Some(val) = maybe {
        println!("Val: {}", val);
    }
}

// 5. Con while let
fn example5() {
    let mut iter = vec![1, 2, 3].into_iter();

    while let Some(x) = iter.next() {
        println!("Val: {}", x);
    }
}

// 6. Destructuring tuple
fn example6() {
    let pair = (1, "ciao");

    match pair {
        (1, msg) => println!("msg: {}", msg),
        _ => println!("altro"),
    }
}

// 7. Match con guard (if)
fn example7() {
    let x = Some(5);

    match x {
        Some(n) if n > 3 => println!("Maggiore di 3"),
        Some(_) => println!("Qualche numero"),
        None => println!("Niente"),
    }
}

// 8. Destructuring struct
struct Point { x: i32, y: i32 }

fn example8() {
    let p = Point { x: 3, y: 7 };

    match p {
        Point { x, y: 7 } => println!("x = {}, y = 7", x),
        Point { x, y } => println!("x = {}, y = {}", x, y),
    }
}

// 9. Binding con @
fn example9() {
    let value = Some(10);

    match value {
        n @ Some(10) => println!("Trovato dieci: {:?}", n),
        _ => println!("Altro"),
    }
}

// 10. Pattern nei parametri di funzione
fn print_coords((x, y): (i32, i32)) {
    println!("x = {}, y = {}", x, y);
}

fn example10() {
    print_coords((5, 8));
}

// 11. Destructuring in let
fn example11() {
    let (a, b) = (1, 2);
    println!("a = {}, b = {}", a, b);
}

// 12. Match multiplo con |
fn example12() {
    let n = 2;

    match n {
        1 | 2 => println!("uno o due"),
        _ => println!("altro"),
    }
}

// 13. ref e ref mut
fn example13() {
    let s = String::from("ciao");

    match s {
        ref r => println!("Reference: {}", r),
    }

    let mut s = String::from("ciao");

    match s {
        ref mut r => r.push_str(" mondo"),
    }
}

// 14. Pattern annidati (nested match)
enum Msg {
    Login { user: String, pass: String },
    Ping,
}

fn example14() {
    let m = Msg::Login { user: "alice".into(), pass: "123".into() };

    match m {
        Msg::Login { user, pass } => println!("User: {}, Pass: {}", user, pass),
        Msg::Ping => println!("Ping"),
    }
}

// 15. Pattern in closure
fn example15() {
    let tuple_vec = vec![(1, 2), (3, 4)];

    tuple_vec.iter().for_each(|(a, b)| {
        println!("a = {}, b = {}", a, b);
    });
}

// 16. Pattern ignorando valori (_, ..)
struct Point3D { x: i32, y: i32, z: i32 }

fn example16() {
    let t = (1, 2, 3);

    match t {
        (1, _, _) => println!("starts with 1"),
        _ => println!("other"),
    }

    let p = Point3D { x: 1, y: 2, z: 3 };

    match p {
        Point3D { x, .. } => println!("x = {}", x),
    }
}

// 17. Pattern irrefutabili
fn example17() {
    let (x, y) = (1, 2); // always matches
    println!("x = {}, y = {}", x, y);
}

// 18. Pattern refutabili con if let
fn example18() {
    // let Some(x) = Some(5); // error if uncommented

    if let Some(x) = Some(5) {
        println!("x = {}", x);
    }
}

// 19. let else
fn process(input: Option<i32>) {
    let Some(x) = input else {
        println!("No value!");
        return;
    };
    println!("Val: {}", x);
}

fn example19() {
    process(Some(10));
    process(None);
}

// 20. Pattern con box
enum List {
    Cons(i32, Box<List>), // cons sta per tupla quando si definisce in un enum o struct
    Nil,
}

fn example20() {
    use List::*;

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    match list {
        Cons(head, tail) => match *tail {
            Cons(next, _) => println!("First: {}, Second: {}", head, next),
            Nil => println!("Only one element"),
        },
        Nil => println!("Empty list"),
    }
}

// 21. Pattern su slice / array
fn example21() {
    let nums = [1, 2, 3];

    match nums {
        [1, _, _] => println!("Inizia con 1"),
        _ => println!("Altro"),
    }

    let slice = &[10, 20, 30][..];

    match slice {
        [a, rest @ ..] => println!("a = {}, rest = {:?}", a, rest),
        _ => (),
    }
}

// 22. Macro matches!
fn example22() {
    let val = Some(3);

    if matches!(val, Some(x) if x > 0) {
        println!("Positive number");
    }
}

// main per eseguire tutti gli esempi
fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    example8();
    example9();
    example10();
    example11();
    example12();
    example13();
    example14();
    example15();
    example16();
    example17();
    example18();
    example19();
    example20();
    example21();
    example22();
}
