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
          write!(&mut buf, "{}", &self.string[pos..])?; // write rest
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

#[cfg(test)]
mod tests {
  use crate::madlib::Template;

  #[derive(Debug, serde::Deserialize)]
  struct ExpandTest {
    name: String, // test name
    val: Template, // test template
    exp: String, // expected value
  }

  #[test]
  fn test_template_expand() {
    let mut rng = rand::rng();

    // tests expected to pass
    let pass_tests = r#"[{
      "name": "empty",
      "val": {
        "string": "",
        "params": { "foo": ["bar"] }
      },
      "exp": ""
    }, {
      "name": "no params",
      "val": {
        "string": "foobar",
        "params": { "foo": ["bar"] }
      },
      "exp": "foobar"
    }, {
      "name": "suffix",
      "val": {
        "string": "{{foo}}bar",
        "params": { "foo": ["bar"] }
      },
      "exp": "barbar"
    }, {
      "name": "prefix",
      "val": {
        "string": "bar{{foo}}",
        "params": { "foo": ["bar"] }
      },
      "exp": "barbar"
    }, {
      "name": "prefix and suffix",
      "val": {
        "string": "bar{{foo}}bar",
        "params": { "foo": ["bar"] }
      },
      "exp": "barbarbar"
    }, {
      "name": "multiple parameters",
      "val": {
        "string": "{{greet}} {{name}}",
        "params": { "greet": ["hi"], "name": ["paul"] }
      },
      "exp": "hi paul"
    }]"#;

    // run pass tests
    for test in serde_json::from_str::<Vec<ExpandTest>>(pass_tests).unwrap() {
      match test.val.expand(&mut rng) {
        Ok(got) => assert_eq!(String::try_from(got).unwrap(), test.exp),
        Err(err) => panic!("test \"{}\" failed: {}", test.name, err),
      };
    }

    // tests expected to fail
    // note: exp is expected error string
    let fail_tests = r#"[{
      "name": "missing key",
      "val": {
        "string": "{{foo}}",
        "params": {}
      },
      "exp": "unknown key: foo"
    }, {
      "name": "unterminated key",
      "val": {
        "string": "{{foo",
        "params": { "foo": ["bar"] }
      },
      "exp": "unterminated key in template"
    }]"#;

    // run fail tests
    for test in serde_json::from_str::<Vec<ExpandTest>>(fail_tests).unwrap() {
      match test.val.expand(&mut rng) {
        Ok(got) => panic!("test \"{}\": got \"{}\", exp error", test.name, String::try_from(got).unwrap()),
        Err(err) => assert_eq!(err.to_string(), test.exp),
      };
    }
  }
}
