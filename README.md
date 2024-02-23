# Static Closed-Addressing Dictionary based on HashTable with multiple K:V types on Rust #

## About this project ##

This is part of my pure interest in Algorithms course. I've decided to create a dictionary with multiple types of keys and values on Rust. I did it, but encountered some problems, which couldn't resolve.
So this implementation is not very good example to use in your projects. This is not convenient and *possibly* broken.
This is Closed-Addressing dictionary, so the buckets are Linked Lists.

## Dictionary methods and types ##

### Types ###

The dictionary can hold key and value pairs.
Key can be:

* String
* Int32
* Boolean
* Tuple (Key, Key)

Value is dynamic type, so it can be almost everything. You can add new types for a key if you want.

### Methods ###

Dictionary has the next methods:

* insert - inserts pair in dict
* remove - removes pair from dict
* clear - clears the dict
* items - returns a vector with all dict elements
* length - returns current length of dict
* look_up - returns a value from pair

## Known Issues ##
File murmurhash.rs might be not working (I think), it should work as hash method and dictionary should be able to be created with this type of hashing.
