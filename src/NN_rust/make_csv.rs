use std::fs::File;
use std::io::{BufWriter, Write};
use rand::Rng;

fn create_addition_dataset(filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::thread_rng();
    for j in 0..100000 {
        let num1 = rng.gen_range(0..=10) as f64 / 10.0;
        let num2 = rng.gen_range(0..=10) as f64 / 10.0;
        write!(writer, "{},{},-,{}\n", num1, num2, (num1 + num2)/20.0)?;
    }

    // for (i, element) in vec.iter().enumerate() {
    //     if i > 0 {
    //         writer.write(b",")?;
    //     }
    //     write!(writer, "{}", element)?;
    // }

    Ok(())
}

pub fn main() {
    create_addition_dataset("src/NN_rust/addition.csv").unwrap();
}