use std::io::Write;
use rand::Rng;

#[derive(Debug, serde::Deserialize)]
struct Config {
  template: String, // template string
  params: std::collections::HashMap<String,Vec<String>>, // template parameters
}

fn main() -> Result<(), std::io::Error> {
  let mut rng = rand::rng(); // init random number generator
  let mut buf = Vec::new(); // init output buffer
  let config: Config = serde_json::from_reader(std::io::stdin())?; // read config

  let mut pos = 0usize; // position
  loop {
    // get prefix length
    let len = match &config.template[pos..].find("{{") {
      Some(len) => *len,
      None => {
        writeln!(&mut buf, "{}", &config.template[pos..])?; // write rest
        break; // exit loop
      },
    };

    // write prefix, advance position
    write!(&mut buf, "{}", &config.template[pos..(pos + len)])?; // write prefix
    pos += len + 2; // advance position

    // get key length
    let len = match &config.template[pos..].find("}}") {
      Some(len) => *len,
      None => panic!("unterminated key in template"),
    };

    // get parameter key, then get possible values
    let key = &config.template[pos..(pos + len)];
    let vals = match config.params.get(key) {
      Some(vals) => vals, // get values
      None => panic!("unknown key: {key}"), // unknown key
    };

    // write random value, advance position
    write!(&mut buf, "{}", &vals[rng.random_range(0..vals.len())])?;
    pos += len + 2; // advance
  }

  // write to stdout, return success
  let mut stdout = std::io::stdout().lock();
  stdout.write_all(&buf)?; // write to stdout
  stdout.flush()?; // flush stdout
  Ok(()) // return success
}
