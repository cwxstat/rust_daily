# Go and in Rust
So Go can do this... can Rust do this?

```go
s := fmt.Sprintf("example: {}\n", "text")
```

```rust,editable
fn main() {
    let s = format!("example: {}\n", "text");
    println!("{}", s);
}
```

## Maps

### Go Sample

```go
package main

import "fmt"

func main() {
	m := map[string]string{
		"key1": "value1",
		"key2": "value2",
	}

	fmt.Println(m)
}

```

### Rust Sample

```rust,editable
use std::collections::HashMap;

fn main() {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("key1".to_string(), "value1".to_string());
    m.insert("key2".to_string(), "value2".to_string());

    println!("{:?}", m);
}
```

## Append

### Go Sample


```go
package main

import "fmt"

func main() {
	s := []string{}
	s = append(s,"one")
	s = append(s,"two")

	fmt.Println(s)
}

```
### Rust Sample
```rust,editable
fn main() {
    let mut s: Vec<String> = Vec::new();
    s.push("one".to_string());
    s.push("two".to_string());

    println!("{:?}", s);
}

```

## Structs and Recievers
### Go Sample

```go
package main

import "fmt"

type K struct {
	name string
	age  float64
}

type S struct {
	Name string
	b    []K
}

func (s *S) Get(name string) float64 {
	for _, v := range s.b {
		if v.name == name {
			return v.age
		}

	}
	return -1
}

func (s *S) Put(name string, age float64) {
	if s.b == nil {
		s.b = make([]K, 0)

	}
	s.b = append(s.b, K{name, age})

}

func main() {
	s := &S{"Bob", nil}
	s.Put("Bob", 12)
	s.Put("Alice", 13)
	fmt.Println("bob's age: ", s.Get("Bob"))

	fmt.Println(s)

}

```


### Rust Sample

```rust,editable
#[derive(Debug)]
struct K {
    name: String,
    age: f64,
}

#[derive(Debug)]
struct S {
    name: String,
    b: Vec<K>,
}

impl S {
    fn new(name: &str) -> Self {
        S {
            name: name.to_string(),
            b: Vec::new(),
        }
    }

    fn get(&self, name: &str) -> f64 {
        for k in &self.b {
            if k.name == name {
                return k.age;
            }
        }
        -1.0
    }

    fn put(&mut self, name: &str, age: f64) {
        self.b.push(K {
            name: name.to_string(),
            age,
        });
    }
}

fn main() {
    let mut s = S::new("Bob");
    s.put("Bob", 12.0);
    s.put("Alice", 13.0);
    println!("bob's age: {:?}", s.get("Bob"));

    println!("{:?}", s);
}


```

What about Go's defer?

```rust,editable
n main() {
    defer(|| {
        println!("This message always gets printed last!");
    });

    panic!("Oops, something went wrong!");
}

fn defer<F: FnOnce() + 'static>(f: F) {
    let deferrer = Defer { f: Some(f) };
    std::mem::forget(deferrer);
}

struct Defer<F: FnOnce() + 'static> {
    f: Option<F>,
}

impl<F: FnOnce() + 'static> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        }
    }
}


```