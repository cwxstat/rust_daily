Object.assign(window.search, {"doc_urls":["chapter_1.html#chapter-1","chapter_1.html#a-more-advanced-example","chapter_2.html#chapter-2","chapter_3.html#chapter-3","raw_notes.html#raw-notes","raw_notes.html#2023-03-04"],"index":{"documentStore":{"docInfo":{"0":{"body":36,"breadcrumbs":4,"title":2},"1":{"body":37,"breadcrumbs":5,"title":3},"2":{"body":3,"breadcrumbs":4,"title":2},"3":{"body":0,"breadcrumbs":4,"title":2},"4":{"body":5,"breadcrumbs":4,"title":2},"5":{"body":46,"breadcrumbs":5,"title":3}},"docs":{"0":{"body":"This book was created with mdBook It's very simple to create a book with rust, with interactive code snippets. Everything here is created in the folder /theBook mdbook build\nrm -rf ../docs\nmv book ../docs And, that's it! Here is the first example of interactive code. fn main() { // Some code println!(\"Hello, world!\");\n}","breadcrumbs":"Chapter 1 » Chapter 1","id":"0","title":"Chapter 1"},"1":{"body":"macro_rules! test { // Arguments don't need to be separated by a comma. // Any template can be used! ($left:expr; and $right:expr) => { println!(\"{:?} and {:?} is {:?}\", stringify!($left), stringify!($right), $left && $right) }; // ^ each arm must end with a semicolon. ($left:expr; or $right:expr) => { println!(\"{:?} or {:?} is {:?}\", stringify!($left), stringify!($right), $left || $right) };\n} fn main() { test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32); test!(true; or false);\n}","breadcrumbs":"Chapter 1 » A more advanced example","id":"1","title":"A more advanced example"},"2":{"body":"This is an example of chapter 2.","breadcrumbs":"Chapter 2 » Chapter 2","id":"2","title":"Chapter 2"},"3":{"body":"","breadcrumbs":"Chapter 3 » Chapter 3","id":"3","title":"Chapter 3"},"4":{"body":"Daily progress notes, in no particular order.","breadcrumbs":"Raw Notes » Raw Notes","id":"4","title":"Raw Notes"},"5":{"body":"Write a Rust program each day, for 100 days. fn add(a:&mut i32, b: i32) -> i32 { *a += b; *a\n} fn main() { let mut x = 5; println!(\"{}\\n\",add(&mut x, 2)); let mut y = add(&mut x, 20); println!(\"{}\\n\",add(&mut y, 2)); } It's also possible to add Go code. But, this code won't execute. package main import \"fmt\" func main() { fmt.Println(\"Hello, playground\")\n}","breadcrumbs":"Raw Notes » 2023-03-04","id":"5","title":"2023-03-04"}},"length":6,"save":true},"fields":["title","body","breadcrumbs"],"index":{"body":{"root":{"0":{"3":{"df":1,"docs":{"5":{"tf":1.0}}},"4":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}},"1":{"0":{"0":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}},"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}},"2":{"0":{"2":{"3":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}},"df":1,"docs":{"5":{"tf":1.0}}},"df":3,"docs":{"1":{"tf":1.0},"2":{"tf":1.4142135623730951},"5":{"tf":1.4142135623730951}},"i":{"3":{"2":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"df":0,"docs":{}}},"3":{"df":1,"docs":{"3":{"tf":1.0}}},"4":{"df":0,"docs":{},"i":{"3":{"2":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}},"5":{"df":1,"docs":{"5":{"tf":1.0}}},"a":{"d":{"d":{"(":{"&":{"df":0,"docs":{},"m":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"a":{":":{"&":{"df":0,"docs":{},"m":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{},"v":{"a":{"df":0,"docs":{},"n":{"c":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"m":{"df":1,"docs":{"1":{"tf":1.0}}}}},"b":{"df":1,"docs":{"5":{"tf":1.4142135623730951}},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":3,"docs":{"0":{"tf":1.0},"2":{"tf":1.4142135623730951},"3":{"tf":1.0}}}}}}},"df":0,"docs":{}},"o":{"d":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.7320508075688772},"5":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"m":{"a":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}}},"r":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}},"df":0,"docs":{}}}},"d":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"4":{"tf":1.0}}}}},"y":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"o":{"c":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"e":{"a":{"c":{"df":0,"docs":{},"h":{"df":2,"docs":{"1":{"tf":1.0},"5":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"y":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"x":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":3,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"2":{"tf":1.0}}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}},"df":0,"docs":{}}}},"f":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"s":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"r":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"m":{"df":0,"docs":{},"t":{".":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":0,"docs":{},"n":{"(":{"\"":{"df":0,"docs":{},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}},"df":1,"docs":{"5":{"tf":1.0}}}},"n":{"df":3,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"5":{"tf":1.4142135623730951}}},"o":{"df":0,"docs":{},"l":{"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"u":{"df":0,"docs":{},"n":{"c":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}}}},"g":{"df":0,"docs":{},"o":{"df":1,"docs":{"5":{"tf":1.0}}}},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}}},"i":{"3":{"2":{"df":1,"docs":{"5":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}}},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"a":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"t":{"'":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}},"df":0,"docs":{}}},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"t":{":":{"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"m":{"a":{"c":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"_":{"df":0,"docs":{},"r":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":3,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"5":{"tf":1.7320508075688772}}}}},"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}}},"df":0,"docs":{}},"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}},"v":{"df":1,"docs":{"0":{"tf":1.0}}}},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"4":{"tf":1.4142135623730951}}}}}},"o":{"df":0,"docs":{},"r":{"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"4":{"tf":1.0}}}}},"df":0,"docs":{}}},"p":{"a":{"c":{"df":0,"docs":{},"k":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"4":{"tf":1.0}}}},"df":0,"docs":{}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"y":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}}}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":0,"docs":{},"n":{"!":{"(":{"\"":{"df":0,"docs":{},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"{":{"df":0,"docs":{},"}":{"\\":{"df":0,"docs":{},"n":{"\"":{",":{"a":{"d":{"d":{"(":{"&":{"df":0,"docs":{},"m":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"4":{"tf":1.0}}}}}}}}}},"r":{"a":{"df":0,"docs":{},"w":{"df":1,"docs":{"4":{"tf":1.0}}}},"df":0,"docs":{},"f":{"df":1,"docs":{"0":{"tf":1.0}}},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{":":{"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"m":{"df":1,"docs":{"0":{"tf":1.0}}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}}}}},"s":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"df":0,"docs":{}}},"p":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"y":{"!":{"(":{"$":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"s":{"df":0,"docs":{},"t":{"!":{"(":{"1":{"df":0,"docs":{},"i":{"3":{"2":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"u":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":1,"docs":{"1":{"tf":1.0}}}}},"h":{"a":{"df":0,"docs":{},"t":{"'":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}}}},"u":{"df":0,"docs":{},"s":{"df":1,"docs":{"1":{"tf":1.0}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"5":{"tf":1.0}}}}}}},"x":{"df":1,"docs":{"5":{"tf":1.7320508075688772}}},"y":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}},"breadcrumbs":{"root":{"0":{"3":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"4":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"1":{"0":{"0":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}},"df":2,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.4142135623730951}}},"2":{"0":{"2":{"3":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"df":1,"docs":{"5":{"tf":1.0}}},"df":3,"docs":{"1":{"tf":1.0},"2":{"tf":2.0},"5":{"tf":1.4142135623730951}},"i":{"3":{"2":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"df":0,"docs":{}}},"3":{"df":1,"docs":{"3":{"tf":1.7320508075688772}}},"4":{"df":0,"docs":{},"i":{"3":{"2":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}},"5":{"df":1,"docs":{"5":{"tf":1.0}}},"a":{"d":{"d":{"(":{"&":{"df":0,"docs":{},"m":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"a":{":":{"&":{"df":0,"docs":{},"m":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{},"v":{"a":{"df":0,"docs":{},"n":{"c":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"m":{"df":1,"docs":{"1":{"tf":1.0}}}}},"b":{"df":1,"docs":{"5":{"tf":1.4142135623730951}},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":4,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.0},"2":{"tf":2.0},"3":{"tf":1.7320508075688772}}}}}}},"df":0,"docs":{}},"o":{"d":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.7320508075688772},"5":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"m":{"a":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}}},"r":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}},"df":0,"docs":{}}}},"d":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"4":{"tf":1.0}}}}},"y":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"o":{"c":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"e":{"a":{"c":{"df":0,"docs":{},"h":{"df":2,"docs":{"1":{"tf":1.0},"5":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"y":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"x":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":3,"docs":{"0":{"tf":1.0},"1":{"tf":1.4142135623730951},"2":{"tf":1.0}}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}},"df":0,"docs":{}}}},"f":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"s":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"r":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"m":{"df":0,"docs":{},"t":{".":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":0,"docs":{},"n":{"(":{"\"":{"df":0,"docs":{},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}},"df":1,"docs":{"5":{"tf":1.0}}}},"n":{"df":3,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"5":{"tf":1.4142135623730951}}},"o":{"df":0,"docs":{},"l":{"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"u":{"df":0,"docs":{},"n":{"c":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}}}},"g":{"df":0,"docs":{},"o":{"df":1,"docs":{"5":{"tf":1.0}}}},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}}},"i":{"3":{"2":{"df":1,"docs":{"5":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}}},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"a":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"t":{"'":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}},"df":0,"docs":{}}},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"t":{":":{"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"m":{"a":{"c":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"_":{"df":0,"docs":{},"r":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":3,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"5":{"tf":1.7320508075688772}}}}},"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}}},"df":0,"docs":{}},"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}},"v":{"df":1,"docs":{"0":{"tf":1.0}}}},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":2,"docs":{"4":{"tf":2.0},"5":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"r":{"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"4":{"tf":1.0}}}}},"df":0,"docs":{}}},"p":{"a":{"c":{"df":0,"docs":{},"k":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"4":{"tf":1.0}}}},"df":0,"docs":{}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"y":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}}}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":0,"docs":{},"n":{"!":{"(":{"\"":{"df":0,"docs":{},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"{":{"df":0,"docs":{},"}":{"\\":{"df":0,"docs":{},"n":{"\"":{",":{"a":{"d":{"d":{"(":{"&":{"df":0,"docs":{},"m":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"4":{"tf":1.0}}}}}}}}}},"r":{"a":{"df":0,"docs":{},"w":{"df":2,"docs":{"4":{"tf":1.7320508075688772},"5":{"tf":1.0}}}},"df":0,"docs":{},"f":{"df":1,"docs":{"0":{"tf":1.0}}},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{":":{"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"m":{"df":1,"docs":{"0":{"tf":1.0}}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}}}}},"s":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"df":0,"docs":{}}},"p":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"y":{"!":{"(":{"$":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"s":{"df":0,"docs":{},"t":{"!":{"(":{"1":{"df":0,"docs":{},"i":{"3":{"2":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"u":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":1,"docs":{"1":{"tf":1.0}}}}},"h":{"a":{"df":0,"docs":{},"t":{"'":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}}}},"u":{"df":0,"docs":{},"s":{"df":1,"docs":{"1":{"tf":1.0}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"5":{"tf":1.0}}}}}}},"x":{"df":1,"docs":{"5":{"tf":1.7320508075688772}}},"y":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}},"title":{"root":{"0":{"3":{"df":1,"docs":{"5":{"tf":1.0}}},"4":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}},"1":{"df":1,"docs":{"0":{"tf":1.0}}},"2":{"0":{"2":{"3":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"2":{"tf":1.0}}},"3":{"df":1,"docs":{"3":{"tf":1.0}}},"a":{"d":{"df":0,"docs":{},"v":{"a":{"df":0,"docs":{},"n":{"c":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":3,"docs":{"0":{"tf":1.0},"2":{"tf":1.0},"3":{"tf":1.0}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}}},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"4":{"tf":1.0}}}}}},"r":{"a":{"df":0,"docs":{},"w":{"df":1,"docs":{"4":{"tf":1.0}}}},"df":0,"docs":{}}}}},"lang":"English","pipeline":["trimmer","stopWordFilter","stemmer"],"ref":"id","version":"0.9.5"},"results_options":{"limit_results":30,"teaser_word_count":30},"search_options":{"bool":"OR","expand":true,"fields":{"body":{"boost":1},"breadcrumbs":{"boost":1},"title":{"boost":2}}}});