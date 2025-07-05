use std::io::Write;

#[derive(Debug, serde::Deserialize)]
pub struct Template {
  string: String, // template string
  params: std::collections::HashMap<String,Vec<String>>, // template parameters
}

impl Template {
  pub fn expand(self, rng: &mut impl rand::Rng) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new(); // init output buffer
    let mut pos = 0usize; // position

    loop {
      // get prefix length
      let len = match &self.string[pos..].find("{{") {
        Some(len) => *len,
        None => {
          writeln!(&mut buf, "{}", &self.string[pos..])?; // write rest
          break; // exit loop
        },
      };

      // write prefix, advance position
      write!(&mut buf, "{}", &self.string[pos..(pos + len)])?; // write prefix
      pos += len + 2; // advance position

      // get key length
      let len = match &self.string[pos..].find("}}") {
        Some(len) => *len,
        None => return Err(std::io::Error::other("unterminated key in template")),
      };

      // get key, get possible values, then pick random value
      let key = &self.string[pos..(pos + len)]; // get key
      let val = match self.params.get(key) {
        Some(vals) => &vals[rng.random_range(..vals.len())], // get random value
        None => return Err(std::io::Error::other(format!("unknown key: {key}"))),
      };

      // write value, advance position
      write!(&mut buf, "{val}")?;
      pos += len + 2; // advance position
    }

    Ok(buf) // return success
  }
}
