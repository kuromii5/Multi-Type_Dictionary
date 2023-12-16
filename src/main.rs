mod murmurhash;
use std::any::{self, Any};
use std::collections::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Result};
use std::rc::Rc;
use std::fmt::Display;

#[derive(Debug)]
enum HashType {
    Default,
    Murmur,
    
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Key {
    Int(i32),
    String(&'static str),
    Bool(bool),
    Tuple((Rc<Key>, Rc<Key>)),
}

#[derive(Debug)]
struct HashTable {
    buckets: Vec<LinkedList<(Key, Rc<dyn Any>)>>,
    size: usize,
    hashtype: HashType,
}

impl HashTable
{
    fn new(size: usize, hashtype: HashType) -> Self {
        HashTable {
            buckets: vec![LinkedList::new(); size],
            size: size,
            hashtype: hashtype,
        }
    }

    fn hash(&self, key: &Key) -> Result<usize> {
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

    fn insert(&mut self, key: Key, value: Rc<dyn Any>) {
        let hash = self.hash(&key).unwrap();

        for (bucket_key, bucket_value) in &mut self.buckets[hash] {
            if key == *bucket_key {
                *bucket_value = value;
                return;
            }
        }

        self.buckets[hash].push_front((key, value));
    }

    fn look_up(&self, key: &Key) -> Option<&Rc<dyn Any>> {
        let hash = self.hash(key).unwrap();

        for (bucket_key, bucket_value) in &self.buckets[hash] {
            if *key == *bucket_key {
                return Some(bucket_value);
            }
        }

        None
    }

    fn remove(&mut self, key: &Key) {
        let hash = self.hash(key).unwrap();
        let mut new_list: LinkedList<(Key, Rc<dyn Any + 'static>)> = LinkedList::new();
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

    fn items(&self) -> Vec<&(Key, Rc<dyn Any + 'static>)> {
        let mut vec: Vec<&(Key, Rc<dyn Any + 'static>)> = Vec::new();
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
    let mut t: HashTable = HashTable::new(5, HashType::Default);
    t.insert(Key::Int(56), Rc::new(128));
    t.insert(Key::String("Easy"), Rc::new("string"));

    println!("{:?}", t);

    t.remove(&Key::Int(56));
    t.insert(
        Key::Tuple(
            (Rc::new(Key::Tuple((Rc::new(Key::Bool(true)), Rc::new(Key::String("amogus"))))),
             Rc::new(Key::Int(68))
            )
        ),
        Rc::new(56.23)
    );

    println!("{:?}", t);
}
