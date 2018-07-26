# autoneverimpl

A Rust macro for automatically generating an implementation of your trait for the never type (`!`).

Each method's implementation is just a simple `unreachable!()`.

## Example

```rust
#[feature(never_type)]

#[autoneverimpl]
trait MyAwesomeTrait {
	fn f();
}

fn main() {
	<! as MyAwesomeTrait>::f();
}
```
