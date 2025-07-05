use std::io::Write;
use rand::Rng;

#[derive(Debug, serde::Deserialize)]
struct Config {
  template: String, // template string
  params: std::collections::HashMap<String,Vec<String>>, // template parameters
}

fn main() -> Result<(), std::io::Error> {
  let re = regex::Regex::new(r"\{\{[\w_-]+\}\}").unwrap(); // init regex
  let mut rng = rand::rng(); // init random number generator
  let mut buf = Vec::new(); // init output buffer
  let config: Config = serde_json::from_reader(std::io::stdin())?; // read config

  // find parameters in template
  let matches: Vec<_> = re.find_iter(&config.template).collect();
  if matches.len() > 0 {
    // loop over matches
    for (i, m) in matches.clone().into_iter().enumerate() {
      // write prefix
      let start = if i > 0 { matches[i - 1].end() } else { 0 }; // get prefix start
      let prefix = &config.template[start..m.start()]; // get prefix
      write!(&mut buf, "{prefix}")?; // write prefix

      // get parameter key from match, then get possible values
      let key = &config.template[m.start()+2 .. m.end()-2];
      let vals = match config.params.get(key) {
        Some(vals) => vals, // get values
        None => panic!("unknown key: {key}"), // unknown key
      };

      // write random value
      write!(&mut buf, "{}", &vals[rng.random_range(0..vals.len())])?;
    }

    // write suffix and newline
    let suffix = &config.template[matches[matches.len() - 1].end()..];
    writeln!(&mut buf, "{suffix}")?;
  } else {
    // no matches; write template and newline
    writeln!(&mut buf, "{}", config.template)?;
  }

  // write to stdout, return success
  let mut stdout = std::io::stdout().lock();
  stdout.write_all(&buf)?; // write to stdout
  stdout.flush()?; // flush stdout
  Ok(()) // return success
}
