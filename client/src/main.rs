use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()>{
    println!("Start on Client");

    Ok(())
}
