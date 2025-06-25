use ese_1::tratto::MySlug;

pub fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true
    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();

    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice
}
