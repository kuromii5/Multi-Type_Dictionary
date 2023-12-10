mod murmurhash;
use std::any::{self, Any};
use std::collections::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Result};
use std::rc::Rc;

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
    K: Hash + Clone + PartialEq,
    V: Clone + PartialEq,
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

    fn insert(&mut self, key: K, value: V) -> () {
        let hash = self.hash(&key).unwrap();

        for (bucket_key, bucket_value) in &mut self.buckets[hash] {
            if key == *bucket_key {
                *bucket_value = value;
                return;
            }
        }

        self.buckets[hash].push_front((key, value));
    }

    fn look_up(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key).unwrap();

        for (bucket_key, bucket_value) in &self.buckets[hash] {
            if *key == *bucket_key {
                return Some(bucket_value);
            }
        }

        None
    }

    fn remove(&mut self, key: &K) {
        let hash = self.hash(key).unwrap();
        let mut new_list: LinkedList<(K, V)> = LinkedList::new();
        while let Some((existing_key, existing_value)) = self.buckets[hash].pop_front() {
            if *key == existing_key {
                continue;
            }

            new_list.push_back((existing_key, existing_value))
        }

        self.buckets[hash] = new_list;
    }

    fn clear(&mut self) {
        for list in &mut self.buckets {
            list.clear();
        }
    }

    fn items(&self) -> Vec<&(K, V)> {
        let mut vec: Vec<&(K, V)> = Vec::new();
        for list in &self.buckets {
            for item in list {
                vec.push(item);
            }
        }

        vec
    }

    fn length(&self) -> usize {
        let mut count: usize = 0;
        for bucket in &self.buckets {
            count += bucket.len();
        }
        count
    }
}

fn main() {
    let mut t: HashTable<String, String> = HashTable::new(5, HashType::Default);
    t.insert(String::from("key1"), String::from("42"));
    t.insert(String::from("key2"), "99".into());
    t.insert(String::from("key1"), String::from("48"));
    t.insert(String::from("key3"), String::from("48"));
    t.insert(String::from("key4"), String::from("48"));
    t.insert(String::from("key5"), String::from("48"));
    t.insert(String::from("key6"), String::from("48"));
    println!("{:?}", t);
    println!("{}", t.length());
    println!("{:?}", t.look_up(&"key2".to_string()));

    t.remove(&"key2".to_string());

    println!("{:?}", t);
    println!("{:?}", t.items());

    t.clear();

    println!("{:?}", t);
    println!("{:?}", t.items());
}
