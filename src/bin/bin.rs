use std::error::Error;
use std::io::stdin;
use ufc_ai::demo::clustering;
use ufc_ai::demo::genetic;
use ufc_ai::demo::puzzle;
use ufc_ai::demo::regression;

const DEMOS: &[(&str, fn() -> Result<(), Box<dyn Error>>)] = &[
    ("puzzle demo", || {
        puzzle::demo();
        Ok(())
    }),
    ("genetic demo", || {
        genetic::demo();
        Ok(())
    }),
    ("regression demo 1 SGD", regression::demo_1),
    ("regression demo 2 SGD", regression::demo_2),
    ("regression demo 2 LS", regression::demo_2_ls),
    ("regression demo 3 SGD", regression::demo_3),
    ("regression demo 3 LS", regression::demo_3_ls),
    ("k means demo", clustering::k_means_demo),
];

fn main() -> Result<(), Box<dyn Error>> {
    for (i, (name, _)) in DEMOS.iter().enumerate() {
        println!("{}: {}", i, name)
    }
    println!("{}: {}", DEMOS.len(), "exit");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    match buffer.trim().parse::<usize>() {
        Ok(n) if n < DEMOS.len() => {
            if n < DEMOS.len() {
                DEMOS[n].1()?
            }
        }
        Ok(n) if n == DEMOS.len() => {
            println!("bye");
            return Ok(());
        }
        _ => println!("unknown option"),
    }

    Ok(())
}
