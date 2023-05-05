# FAQs

#### Q.

Why Hashmap is used for TODO list instead of list of objects?

#### A.

Because it's easier to read the data.

---

#### Q.

Why Mutex is used for TODO list?

#### A.

Because Rust doesn't know at compile time which thread will call the code. If we don't use Mutex, Rust will complain that the code is not thread-safe.
