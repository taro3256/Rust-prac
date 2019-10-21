use std::f32;

fn main() {
    let angle = f32::consts::PI*2.0;
    let x = angle.cos();
    let y = angle.sin();
    println!("{}", x);
    println!("{}", y);
}
