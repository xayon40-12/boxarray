use boxarray::boxarray;
fn main() {
    let v = 7.0;
    let a: Box<[[[f64; 3]; 2]; 4]> = boxarray(v);
    println!("{a:?}");
}
