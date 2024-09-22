use std::hash::Hash;
use std::collections::HashMap;
use rand::seq::IteratorRandom;
#[derive(Debug,PartialEq)]
pub enum ChainError {
    EmptySequence,
}

#[derive(Debug,PartialEq)] 
pub struct Chain<T>
where
    T: Eq + Hash + Clone,
{
    map: HashMap<T, HashMap<T, usize>>,
}

impl<T> Chain<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Chain {
            map: HashMap::new(),
        }
    }

    pub fn train(&mut self, sequence: &[T]) -> Result<&mut Self, ChainError> {
        if sequence.is_empty() {
            return Err(ChainError::EmptySequence);
        }
        
        if sequence.len() == 1 {
            return Ok(self);
        }
    
        sequence.iter().zip(sequence.iter().skip(1)).for_each(|(current, next)| {
            self.map
                .entry(current.clone())
                .or_default()
                .entry(next.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
        
        Ok(self)
    }

    pub fn most_likely_after(&self, token: T) -> Option<T> {
        let mut rng = rand::thread_rng();
        self.map.get(&token).and_then(|next_map| {
            let max_count = match next_map.values().max() {
                Some(value) => value,
                None => return None,
            };
            let candidates: Vec<_> = next_map
                .iter()
                .filter(|&(_, &count)| count == *max_count)
                .collect();
    
                if candidates.len() == 1 {
                    Some(candidates[0].0.clone())
                } else {
                    candidates.into_iter()
                        .choose(&mut rng)
                        .map(|(next_token, _)| next_token.clone())
                }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_with_empty_sequence() {
        let mut chain1 = Chain::new();
        let mut chain2 = Chain::new();
        let result1 = chain1.train(&Vec::<&i32>::new());
        let result2 = chain2.train(&Vec::<&String>::new());
        assert_eq!(result1, Err(ChainError::EmptySequence));  
        assert_eq!(result2, Err(ChainError::EmptySequence));  
    }

    #[test]
    fn test_train_and_most_likely_after() {
        let mut chain1 = Chain::new();
        let mut chain2 = Chain::new();

        assert!(chain1.train(&["apple", "banana", "cherry", "apple", "banana", "cherry", "banana"]).is_ok(), "Training chain1 failed");
        assert!(chain2.train(&[1, 1, 3, 1, 1, 3]).is_ok(), "Training chain2 failed");
        
        assert_eq!(chain1.most_likely_after("apple"), Some("banana"));
        let result0 = chain1.most_likely_after("cherry");
        assert!(result0 == Some("apple") || result0 == Some("banana"));   
        assert_eq!(chain1.most_likely_after("watermelon"), None); 
        let result1 = chain2.most_likely_after(1);
        assert!(result1 == Some(1) || result1 == Some(3));  
        assert_eq!(chain2.most_likely_after(3), Some(1));  
        assert_eq!(chain2.most_likely_after(4), None);
    }
}
