# Rust-Py-FastEmbed

`rust-py-fastembed` is a Rust library that provides Python bindings for processing arrays and dictionaries. It uses the [`pyo3`](https://pyo3.rs/) library to enable seamless interaction between Rust and Python, allowing you to manipulate data structures efficiently.

This library leverages **Rust's type safety** and **high performance** to ensure robust and fast data processing, making it an excellent choice for applications that require both safety and speed.

## Features

- **Process Integer Arrays**: Multiply all integers in an array by 2, with error handling for negative numbers.
- **Process String Arrays**: Append a suffix to each string in an array, with error handling for empty strings.
- **Process Dictionaries**: Increment integer keys and append a suffix to string values, with error handling for negative keys.

## Why Rust?

Rust is known for its **type safety** and **memory safety**, which eliminate entire classes of bugs at compile time. Additionally, Rust's **zero-cost abstractions** ensure that you get the best possible performance without sacrificing safety. By using Rust, this library guarantees:

- **Compile-time type checking**: Prevents type-related errors.
- **Memory safety without garbage collection**: Ensures efficient memory usage.
- **High performance**: Ideal for computationally intensive tasks.

## Installation

To use this library, you need to have Rust and Python installed on your system. Add the following to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.18", features = ["extension-module"] }
```

## Python Module

The library exposes the following Python functions:

1. `process_int_array(input: List[int]) -> List[int]`
   - Multiplies each integer in the input array by 2.
   - Returns an error if any number is negative.

2. `process_string_array(input: List[str]) -> List[str]`
   - Appends " - manipulated" to each string in the input array.
   - Returns an error if any string is empty.

3. `process_dict(input: Dict[int, str]) -> Dict[int, str]`
   - Increments each integer key by 1 and appends " - processed" to each string value.
   - Returns an error if any key is negative.

## Usage

### Python Example

After building the library as a Python module, you can use it as follows:

```python
import fast_manipulation

# Process an integer array
result = fast_manipulation.process_int_array([1, 2, 3])
print(result)  # Output: [2, 4, 6]

# Process a string array
result = fast_manipulation.process_string_array(["hello", "world"])
print(result)  # Output: ["hello - manipulated", "world - manipulated"]

# Process a dictionary
result = fast_manipulation.process_dict({1: "one", 2: "two"})
print(result)  # Output: {2: "one - processed", 3: "two - processed"}
```

### Rust Example

You can also use the library directly in Rust, benefiting from Rust's **type safety** and **performance**:

```rust
use rust_py_fastembed::process_int_array;

fn main() {
    let result = process_int_array(vec![1, 2, 3]).unwrap();
    println!("{:?}", result); // Output: [2, 4, 6]
}
```

## Testing

The library includes unit tests for all functions. To run the tests, use:

```bash
cargo test
```

## Building the Python Module

To build the Python module, use the following command:

```bash
maturin develop
```

This will compile the Rust code and make the Python module available for local testing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.# Rust-Py-FastEmbed

`rust-py-fastembed` is a Rust library that provides Python bindings for processing arrays and dictionaries. It uses the [`pyo3`](https://pyo3.rs/) library to enable seamless interaction between Rust and Python, allowing you to manipulate data structures efficiently.

## Features

- **Process Integer Arrays**: Multiply all integers in an array by 2, with error handling for negative numbers.
- **Process String Arrays**: Append a suffix to each string in an array, with error handling for empty strings.
- **Process Dictionaries**: Increment integer keys and append a suffix to string values, with error handling for negative keys.

## Installation

To use this library, you need to have Rust and Python installed on your system. Add the following to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.18", features = ["extension-module"] }
```

## Python Module

The library exposes the following Python functions:

1. `process_int_array(input: List[int]) -> List[int]`
   - Multiplies each integer in the input array by 2.
   - Returns an error if any number is negative.

2. `process_string_array(input: List[str]) -> List[str]`
   - Appends " - manipulated" to each string in the input array.
   - Returns an error if any string is empty.

3. `process_dict(input: Dict[int, str]) -> Dict[int, str]`
   - Increments each integer key by 1 and appends " - processed" to each string value.
   - Returns an error if any key is negative.

## Usage

### Python Example

After building the library as a Python module, you can use it as follows:

```python
import fast_manipulation

# Process an integer array
result = fast_manipulation.process_int_array([1, 2, 3])
print(result)  # Output: [2, 4, 6]

# Process a string array
result = fast_manipulation.process_string_array(["hello", "world"])
print(result)  # Output: ["hello - manipulated", "world - manipulated"]

# Process a dictionary
result = fast_manipulation.process_dict({1: "one", 2: "two"})
print(result)  # Output: {2: "one - processed", 3: "two - processed"}
```

### Rust Example

You can also use the library directly in Rust:

```rust
use rust_py_fastembed::process_int_array;

fn main() {
    let result = process_int_array(vec![1, 2, 3]).unwrap();
    println!("{:?}", result); // Output: [2, 4, 6]
}
```

## Testing

The library includes unit tests for all functions. To run the tests, use:

```bash
cargo test
```

## Building the Python Module

To build the Python module, use the following command:

```bash
maturin develop
```

This will compile the Rust code and make the Python module available for local testing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.