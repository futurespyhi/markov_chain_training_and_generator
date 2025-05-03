# 🔗 Markov Chain Training and Generator

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-green.svg?style=for-the-badge)

A lightweight and efficient Markov chain implementation in Rust for sequence prediction and generation. This project provides a simple yet powerful interface for training Markov chains on any type of sequential data.

## ✨ Features

- 🚀 **Generic Implementation**: Works with any type that implements `Eq + Hash + Clone`
- 🎯 **Simple API**: Easy-to-use methods for training and prediction
- 🔄 **Deterministic & Random**: Supports both deterministic and random selection for equal probabilities
- ⚡ **Error Handling**: Robust error handling for edge cases
- 🧪 **Well Tested**: Comprehensive unit tests included

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
markov_chain_training_and_generator = "0.1.0"
rand = "0.8.5"
```

## 🚀 Quick Start

```rust
use markov_chain_training_and_generator::Chain;

fn main() {
    // Create a new chain
    let mut chain = Chain::new();
    
    // Train with a sequence
    let sequence = vec!["hello", "world", "hello", "rust"];
    chain.train(&sequence).unwrap();
    
    // Predict the most likely next token
    if let Some(next) = chain.most_likely_after("hello") {
        println!("Most likely after 'hello': {}", next);
    }
}
```

## 🛠️ API Documentation

### Creating a Chain

```rust
let mut chain = Chain::<String>::new();
```

### Training the Chain

```rust
// Train with a sequence of tokens
let result = chain.train(&["apple", "banana", "cherry", "apple", "banana"]);

// Handle potential errors
match result {
    Ok(_) => println!("Training successful!"),
    Err(ChainError::EmptySequence) => println!("Cannot train with empty sequence"),
}
```

### Predicting Next Token

```rust
// Get the most likely token after a given token
if let Some(next) = chain.most_likely_after("apple") {
    println!("Next token: {}", next);
}
```

## 🏗️ Project Structure

```
markov_chain_training_and_generator/
├── src/
│   └── lib.rs          # Core implementation
├── Cargo.toml          # Package manifest
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
└── .gitignore         # Git ignore rules
```

## 🔍 Implementation Details

### Data Structure

The `Chain` struct uses a nested HashMap architecture:
- **Outer HashMap**: Maps tokens to their successor information
- **Inner HashMap**: Maps successor tokens to their occurrence counts
- **Edge Weights**: Occurrence counts serve as edge weights for probability calculation

### Core Methods

#### `train`
- Processes sequences of tokens
- Updates transition counts between consecutive tokens
- Returns `Result<&mut Self, ChainError>` for method chaining
- Handles empty sequences gracefully

#### `most_likely_after`
- Finds the most probable successor for a given token
- Handles ties by random selection
- Returns `Option<T>` for safe handling of missing tokens

## 🧪 Testing

Run the tests with:

```bash
cargo test
```

### Test Coverage

- ✅ Empty sequence handling
- ✅ Training with various data types
- ✅ Prediction accuracy
- ✅ Random selection for equal probabilities
- ✅ Edge case handling

## 📈 Example Use Cases

### Text Generation
```rust
let mut chain = Chain::new();
let words = vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
chain.train(&words).unwrap();
```

### Number Sequences
```rust
let mut chain = Chain::new();
let numbers = vec![1, 1, 2, 3, 5, 8, 13, 21];
chain.train(&numbers).unwrap();
```

### Custom Types
```rust
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    name: String,
    value: i32,
}

let mut chain = Chain::new();
// Train with custom states...
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Inspired by the mathematical concept of Markov chains
- Thanks to the Rust community for excellent documentation and tools

## 📮 Contact

GitHub: [@your-username](https://github.com/futurespyhi)

Project Link: [https://github.com/your-username/markov_chain_training_and_generator](https://github.com/futurespyhi/markov_chain_training_and_generator.git)

---

⭐️ If you find this project useful, please consider giving it a star!
