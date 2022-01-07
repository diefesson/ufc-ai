use std::error::Error;
use std::io::stdin;
use ufc_ai::demo::genetic;
use ufc_ai::demo::gradient;
use ufc_ai::demo::puzzle;

const DEMOS: &[(&str, fn() -> Result<(), Box<dyn Error>>)] = &[
    ("puzzle demo", || {
        puzzle::demo();
        Ok(())
    }),
    ("genetic demo", || {
        genetic::demo();
        Ok(())
    }),
    ("regression demo 1 SGD", gradient::demo_1),
    ("regression demo 2 SGD", gradient::demo_2),
    ("regression demo 2 least squares", gradient::demo_2_ls),
    ("regression demo 3 SGD", gradient::demo_3),
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
