fn maior<'a>(s1: &'a str, s2: &'a str) -> &'a str  {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    let a = String::from("Rust");
    let b = String::from("DevSecOps");

    let maior_str = maior(&a, &b);
    println!("Maior string: {}", maior_str);
}
