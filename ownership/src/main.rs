fn main() {
    let mut s = String::from("Oi");

    let r1 = &s; // Referemcia imutável
    let r2 = &s;

    println!("{} e {}", r1, r2); // ✅ ok
    
    let r3 = &mut s; // Referemcia mutável (exclusiva)

    r3.push_str(" , Silvanei");
    println!("{}", r3);
}
