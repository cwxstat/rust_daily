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
    println!("{}\n",add(&mut x, 2));
    let mut y = add(&mut x, 20);
    println!("{}\n",add(&mut y, 2));    
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






