use std::f32;

fn main() {
    let mut unko;
    let mut angle;
    let mut x;
    let mut y;
    let l = vec![0., -20., 40., -60., 80.];
    
    for i in 0..5 {
        unko = 0.1 * (l[i] as f32);
        angle = unko * f32::consts::PI;
        x = angle.sin();
        y = angle.cos();
        println!("[{:.1}π]", unko);
        println!("x: {:.2}", x);
        println!("y: {:.2}\n", y);
    }
}
