use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::error::Error;
use std::fmt;
use std::fs;

use rand::{Rng, thread_rng};

pub struct Chain {
  map: HashMap<Vec<String>, HashMap<String, usize>>,
  frase_start: HashMap<Vec<String>, usize>,
  order: usize
}

impl Chain {
  pub fn new(order: usize) -> Chain {
    assert!(order > 0);
    let map: HashMap<Vec<String>, HashMap<String, usize>> = HashMap::new();
    let frase_start: HashMap<Vec<String>, usize> = HashMap::new();
    Chain { map, frase_start, order }
  }

  pub fn feed_file(&mut self, filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    contents.lines()
      .filter(|line| !line.is_empty())
      .for_each(|line| self.feed(line.split_whitespace().map(String::from).collect()));
    Ok(())
  }

  fn feed(&mut self, words: Vec<String>) {
    let mut len = words.len();
    if len < self.order { return; }
    len -= self.order;

    for i in 0..len {
      let word_chain = &words[i..(i + self.order)];
      if i == 0 {
        self.frase_start.add(word_chain.to_vec());
      }

      let next_word = words.get(i + self.order).unwrap();

      match self.map.get_mut(word_chain) {
        Some(word_map) => word_map.add(next_word.clone()),
        None => {
          let mut word_map: HashMap<String, usize> = HashMap::new();
          word_map.add(next_word.clone());
          self.map.insert(word_chain.to_vec(), word_map);
        }
      }
    }
  }

  pub fn generate(&self) -> String {
    let mut count = self.order;
    let mut result: Vec<String> = Vec::new();
    let initial_words = self.frase_start.next();
   
    let mut next_word_chain = &initial_words[..];
    result.append(&mut next_word_chain.clone().to_vec());
    while self.map.contains_key(next_word_chain) && count < 1000 { 
      match self.map.get(next_word_chain) {
        Some(word_map) => {
          result.push(word_map.next().clone());
          let start = result.len() - self.order;
          next_word_chain = &result[start..];
        },
        None => unreachable!(),
      }
      count += 1;
    }
    return fix_ponctuation(result.join(" "));
  }

  pub fn print_frase_start(&self) {
    for (words, count) in &self.frase_start {
      println!("{:?}: {}", words, count);
    }
  }
}

fn fix_ponctuation(s: String) -> String {
  s.replace(" .", ".").replace(" !", "!").replace(" ?", "?").replace(" ,", ",")
}

trait States<T> {
  fn add(&mut self, token: T);
  fn next(&self) -> T;
}

impl<T> States<T> for HashMap<T, usize> where T: Hash + Eq + Clone {
  fn add(&mut self, token: T) {
    match self.entry(token) {
      Occupied(mut e) => *e.get_mut() += 1,
      Vacant(e) => { e.insert(1); },
    }
  }

  fn next(&self) -> T {
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
      write!(f, "\t{:?}:\n", key);
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
    let mut chain = Chain::new(2);
    chain.feed("I love cats".split_whitespace().map(String::from).collect());

    let mut hash = HashMap::new();
    hash.insert(String::from("cats"), 1);
    
    let mut result = HashMap::new();
    result.insert(vec!["I".to_string(), "love".to_string()], hash);

    assert_eq!(result, chain.map);
  }

  #[test]
  fn generate() {
    let mut chain = Chain::new(2);
    chain.feed("I love cats".split_whitespace().map(String::from).collect());
    chain.feed("I hate cats".split_whitespace().map(String::from).collect());

    assert!(vec!["I love cats", "cats", "I hate cats"].contains(&&chain.generate()[..]));
  }
}
