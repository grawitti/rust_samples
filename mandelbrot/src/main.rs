use num::Complex;

fn main() {

    for i in 0..100 {
        let opt = escape_time(Complex {re: 5.0, im: 4.0}, 100);
        if opt != None {
            println!("{:?},", opt);
        }
        else {
            println!("None,");
        }
    }
    println!("\n");
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
