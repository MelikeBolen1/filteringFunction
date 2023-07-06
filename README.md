# filteringFunction
This repository contains a simple Rust program that demonstrates how to implement a custom filtering function in Rust. 

# Custom Filtering Function in Rust

The custom filtering function allows you to filter elements from a collection based on a specific condition.

## Implementation Details

The program consists of the following components:

- The `FilterCondition` struct: Represents the filter condition with a single field of the desired type.
- The `is_match` method: Implemented on the `FilterCondition` struct, it checks whether an item matches the filter condition.
- The `custom_filter` function: Takes a collection and a reference to a `FilterCondition` object, and returns a new collection containing only the elements that match the filter condition.
- The `main` function: Creates a collection and a `FilterCondition` object, calls the `custom_filter` function, and prints the filtered result to the console.

## Usage

To run the program locally, follow these steps:

1. Clone this repository: `git clone https://github.com/your-username/custom-filtering-rust.git`
2. Navigate to the project directory: `cd custom-filtering-rust`
3. Build and run the program using Cargo: `cargo run`

You should see the filtered result printed to the console.

Feel free to modify the collection and filter condition in the `main` function to test different scenarios.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request on the GitHub repository.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
