use std::f32;

fn main() {
    let mut unko;
    let mut angle;
    let mut x;
    let mut y;
    
    for i in 0..20 {
        unko = 0.1 * (i as f32);
        angle = unko * f32::consts::PI;
        x = angle.sin();
        y = angle.cos();
        println!("[{:.1}π]", unko);
        println!("x: {:.2}", x);
        if x < 0. {
            println!("y: {:.2}\n", (x.abs()-1.0));
        } else {
            println!("y: {:.2}\n", (1.0-x.abs()));
        }
        // println!("y: {:.2}\n", y);
    }
}
