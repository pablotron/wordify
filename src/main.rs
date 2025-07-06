use std::io::Write;
mod madlib;

fn main() -> Result<(), std::io::Error> {
  let t: madlib::Template = serde_json::from_reader(std::io::stdin())?; // read template
  let mut buf = t.expand(&mut rand::rng())?; // expand template
  if buf.is_empty() || buf[buf.len() - 1] != b'\n' {
    writeln!(&mut buf)? // append newline
  }

  std::io::stdout().lock().write_all(&buf)?; // write buffer
  Ok(()) // return success
}
