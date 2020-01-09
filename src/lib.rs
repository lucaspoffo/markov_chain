use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fs;

use std::error::Error;
use std::fmt;

use rand::{Rng, thread_rng};

pub struct Chain {
  map: HashMap<String, HashMap<String, usize>>
}

impl Chain {
  pub fn new() -> Chain {
    let map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    return Chain { map }
  }

  pub fn feed_file(&mut self, filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    for line in contents.lines() {
      if !line.is_empty() {
        self.feed(line.split_whitespace().map(String::from).collect());
      }
    }
    Ok(())
  }

  fn feed(&mut self, words: Vec<String>) {
    let mut len = words.len();
    if len < 2 { return; }
    len -= 1;
    for i in 0..len {
      let word = words.get(i).unwrap();
      let next_word = words.get(i + 1).unwrap();

      match self.map.get_mut(word) {
        Some(word_map) => word_map.add(next_word.clone()),
        None => {
          let mut word_map: HashMap<String, usize> = HashMap::new();
          word_map.add(next_word.clone());
          self.map.insert(word.clone(), word_map);
        }
      }
    }
  }

  pub fn generate(&self) -> String {
    let mut rng = thread_rng();
    let cap = rng.gen_range(1, self.map.len());
    let mut it = self.map.keys().skip(cap - 1);
    
    let mut count = 0;
    let mut result: Vec<String> = Vec::new();
    if let Some(initial_word) = it.next() {
      let mut next_word = initial_word.clone();
      while self.map.contains_key(&next_word) && count < 23 {
        match self.map.get(&next_word) {
          Some(word_map) => {
            next_word = word_map.next();
            result.push(next_word.clone());
          },
          None => unreachable!(),
        }
        count += 1;
      }
    } else {
      return String::from("");
    }

    return result.join(" ");
  }
}

trait States {
  fn add(&mut self, token: String);
  fn next(&self) -> String;
}

impl States for HashMap<String, usize> {
  fn add(&mut self, token: String) {
    match self.entry(token) {
      Occupied(mut e) => *e.get_mut() += 1,
      Vacant(e) => { e.insert(1); },
    }
  }

  fn next(&self) -> String {
    let mut sum = 0;
    for &value in self.values() {
      sum += value;
    }
    let mut rng = thread_rng();
    let cap = rng.gen_range(0, sum);

    sum = 0;
    for (key, &value) in self.iter() {
      sum += value;
      if sum > cap {
        return key.clone()
      }
    }
    unreachable!("The random number generator failed.")
  }
}

impl fmt::Display for Chain {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Chain: [\n")?;
    for (key, map) in self.map.iter() {
      write!(f, "\t{}:\n", key);
      for (word, value) in map.iter() {
        write!(f, "\t\t{}: {}\n", word, value)?;
      }
    }
    write!(f, "]")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn feed() {
    let mut chain = Chain::new();
    chain.feed("I love cats".split_whitespace().map(String::from).collect());

    let mut i_hash = HashMap::new();
    i_hash.insert(String::from("love"), 1);
    let mut love_hash = HashMap::new();
    love_hash.insert(String::from("cats"), 1);
    
    let mut result = HashMap::new();
    result.insert(String::from("i"), i_hash);
    result.insert(String::from("love"), love_hash);

    assert_eq!(chain.map, result);
  }
}
