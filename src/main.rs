use boxarray::boxarray;

fn main() {
    let v: Box<[[u32; 3]; 4]> = boxarray(1);
    println!("{v:?}");
}
