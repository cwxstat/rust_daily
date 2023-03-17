# Raw Notes

Daily progress notes, in no particular order.

## 2023-03-04

Write a Rust program each day, for 100 days.




```rust,editable

fn add(a:&mut i32, b: i32) -> i32 {
    *a += b;
    *a
}

fn main() {
    let mut x = 5;
    println!("{}\n", add(&mut x, 2));
    let mut y = add(&mut x, 20);
    println!("{}\n", add(&mut y, 2));    
}  
   ```

It's also possible to add Go code. But, this
code won't execute.


```go,editable
package main

import "fmt"

func main() {
    fmt.Println("Hello, playground")
}
```


# From and Into

```rust,editable
#[derive(Debug)]
struct Meter(u32);

#[derive(Debug)]
struct Feet(u32);

// Implementing the From trait for converting Feet into Meter
impl From<Feet> for Meter {
    fn from(feet: Feet) -> Self {
        Meter((feet.0 as f32 * 0.3048) as u32)
    }
}

// Implementing the Into trait for converting Meter into Feet
impl Into<Feet> for Meter {
    fn into(self) -> Feet {
        Feet((self.0 as f32 * 3.28084) as u32)
    }
}

fn main() {
    // Using the From trait
    let feet = Feet(100);
    let meter: Meter = Meter::from(feet);
    println!("100 feet is equal to {:?} meters", meter);

    // Using the Into trait
    let meter = Meter(30);
    let feet: Feet = meter.into();
    println!("30 meters is equal to {:?} feet", feet);
}
```




