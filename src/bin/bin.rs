use std::error::Error;
use ufc_ai::demo::gradient::*;

fn main() -> Result<(), Box<dyn Error>> {
    demo_3()?;
    Ok(())
}
