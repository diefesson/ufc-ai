use std::error::Error;
use ufc_ai::demo::gradient::demo;

fn main() -> Result<(), Box<dyn Error>> {
    demo()?;
    Ok(())
}
