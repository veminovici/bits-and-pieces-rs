# Understand Cow

Based on the ['6 things you can do with the Cow in Rust'](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55).

### Cow Structure

```rust
pub enum Cow<'a, B> 
where
    B: 'a + ToOwned + ?Sized, 
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

### A function rarely modifying the data
This scenario is implemented by *test_1*.

### A struct optionally owning the data
This scenario is implemented by *test_2*.

### A clone on write struct
This scenario is implemented by *test_3*.

### Keep your own type inside it
This scenario is implemented by *test_4*.