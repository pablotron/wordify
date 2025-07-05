use std::io::Write;
mod madlib;

fn main() -> Result<(), std::io::Error> {
  let t: madlib::Template = serde_json::from_reader(std::io::stdin())?; // read template
  let buf = t.expand(&mut rand::rng())?; // expand template

  let mut stdout = std::io::stdout().lock();
  stdout.write_all(&buf)?; // write to stdout

  Ok(()) // return success
}
