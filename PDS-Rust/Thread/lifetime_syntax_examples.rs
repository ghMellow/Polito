// FUNZIONI - Lifetime nella dichiarazione <'a> e sui riferimenti &'a
fn get_first<'a>(x: &'a str, y: &str) -> &'a str {
    //       ^^^^ dichiarazione    ^^^ uso    ^^^ uso (valore ritorno)
    x
}

fn compare<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    //     ^^^^^^^^ dichiarazione      ^^^        ^^^    ^^^ uso
    first
}

// STRUCT - Lifetime nella dichiarazione e sui campi
struct Container<'a> {
    //           ^^^^ dichiarazione
    data: &'a str,  // ^^^ uso sul campo
}

struct TwoRefs<'a, 'b> {
    //         ^^^^^^^^ dichiarazione
    first: &'a str,   // ^^^ uso
    second: &'b str,  // ^^^ uso
}

// TRAIT - Lifetime nella dichiarazione
trait Processor<'a> {
    //          ^^^^ dichiarazione
    fn process(&self, input: &'a str) -> &'a str;
    //                       ^^^         ^^^ uso nei metodi
}

// IMPL - Lifetime su impl e dove necessario
impl<'a> Container<'a> {
    //^^^^ dichiarazione
    fn new(data: &'a str) -> Container<'a> {
        //       ^^^                 ^^^^ uso
        Container { data }
    }
    
    fn get_data(&self) -> &'a str {
        //                ^^^ uso nel return
        self.data
    }
}

impl<'a> Processor<'a> for Container<'a> {
    //^^^^ dichiarazione    ^^^^ uso nel trait
    fn process(&self, input: &'a str) -> &'a str {
        //                   ^^^         ^^^ uso
        if input.len() > self.data.len() {
            input
        } else {
            self.data
        }
    }
}

// ENUM - Lifetime nella dichiarazione
enum Either<'a, 'b> {
    //      ^^^^^^^^ dichiarazione
    Left(&'a str),   // ^^^ uso
    Right(&'b str),  // ^^^ uso
}

// ESEMPI PRATICI
fn demonstrate_usage() {
    let text1 = "Hello";
    let text2 = "World";
    
    // Uso con funzioni
    let result = get_first(text1, text2);
    println!("Result: {}", result);
    
    // Uso con struct
    let container = Container::new(text1);
    println!("Container data: {}", container.get_data());
    
    // Uso con enum
    let either = Either::Left(text1);
    match either {
        Either::Left(s) => println!("Left: {}", s),
        Either::Right(s) => println!("Right: {}", s),
    }
}

// CASI SPECIALI - Lifetime elision (compilatore deduce)
fn simple_case(x: &str) -> &str {
    // Equivale a: fn simple_case<'a>(x: &'a str) -> &'a str
    x
}

fn multiple_inputs(x: &str, y: &str) -> &str {
    // ERRORE! Compilatore non sa quale lifetime usare
    // Devi essere esplicito con <'a, 'b>
    x  // Questo compilerebbe solo se specifichi i lifetime
}

// LIFETIME BOUNDS - Vincoli tra lifetime
fn complex_example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
where
    'b: 'a,  // 'b deve vivere almeno quanto 'a
{
    // Ora posso usare y anche se ritorno con lifetime 'a
    if x.len() > y.len() { x } else { y }
}

// LIFETIME STATICO
fn get_static() -> &'static str {
    //             ^^^^^^^^^ lifetime speciale per valori che vivono per tutto il programma
    "This lives forever"
}

fn main() {
    demonstrate_usage();
    
    let s1 = "test1";
    let s2 = "test2";
    
    let result1 = simple_case(s1);
    println!("Simple: {}", result1);
    
    let result2 = complex_example(s1, s2);
    println!("Complex: {}", result2);
    
    let static_str = get_static();
    println!("Static: {}", static_str);
}



fn process_mutable<'a>(data: &'a mut String) -> &'a mut String {
        //                       ^^^^ mut     ^^^^ mut
        //                       lifetime     lifetime
        data.push_str(" processed");
        data
    }




fn __main__() {
    let mut contatore = Some(0);
    while let Some(c) = contatore {
    println!("Il contatore è: {}", c);
    contatore = if c < 3 { Some(c + 1) } else { None };
    }
}