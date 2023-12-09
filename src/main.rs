mod murmurhash;
use std::collections::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Result};

#[derive(Debug)]
enum HashType {
    Default,
    Murmur,
    
}

#[derive(Debug)]
struct HashTable<K, V> {
    buckets: Vec<LinkedList<(K, V)>>,
    size: usize,
    hashtype: HashType,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Clone,
    V: Clone,
{
    fn new(size: usize, hashtype: HashType) -> Self {
        HashTable {
            buckets: vec![LinkedList::new(); size],
            size: size,
            hashtype: hashtype,
        }
    }

    fn hash(&self, key: &K) -> Result<usize> {
        match self.hashtype {
            HashType::Default => {
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                Ok(hasher.finish() as usize % self.size)
            }
            HashType::Murmur => match murmurhash::murmur(&mut key.clone(), 727) {
                Ok(h) => Ok((h % self.size as u128) as usize),
                Err(e) => Err(e),
            },
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let hash = self.hash(&key).unwrap();
        self.buckets[hash].push_front((key, value));
    }

    fn look_up(&self, key: &K) -> Option<&V> {
        None
    }

    fn remove(&mut self, key: &K) {

    }

    fn clear(&mut self) {

    }

    fn items(&mut self) {

    }

    fn length(&mut self) {
        
    }
}

fn main() {
    let mut t: HashTable<String, String> = HashTable::new(5, HashType::Murmur);
    t.insert(String::from("key1"), String::from("42"));
    t.insert(String::from("key2"), "99".into());
    println!("{:?}", t);

}
