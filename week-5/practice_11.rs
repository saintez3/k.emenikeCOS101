fn main() {
    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
        println!("true");
    }

    let c = 0;
    let d = 30;

    if (c > 10) || (d > 18) {
        println!("true");
    }

    let is_elder = false;

    if !is_elder {
        println!("Not Elder");
    }
}fn main() {
    let a: i32 = 10;
    let b: i32 = 11;

    let mut result: i32;

    result = a & b;
    println!("a & b = {}", result);

    result = a | b;
    println!("a | b = {}", result);

    result = a ^ b;
    println!("a ^ b = {}", result);

    result = !a;
    println!("!a = {}", result);

    result = a << b;
    println!("a << b = {}", result);

    result = a >> b;
    println!("a >> b = {}", result);
}