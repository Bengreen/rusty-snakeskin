# Actions to test rusty snakeskin

* [x] Create a python library
* [x] Create a rust program
* [x] Call elements of the python library from rust
* [x] Call async functions of the python library
* [ ] Call the Rust API from python
    * [ ] Create a API definition of the Rust API in python
    * [ ] Send API object from Rust into python when constructing a python object
    * [ ] Call methods on that object to hit the rust APIs


# Problems

How can i load the rust program (binary -- probably static) and have it accessible from python whilst sharing the same library object code as the rust binary.

Suggestion create a single shared object library that is available to both python and rust.


