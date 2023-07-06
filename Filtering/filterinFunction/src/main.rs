struct FilterCondition {
    threshold: i32,
}

impl FilterCondition {
    // Method that checks if the value matches the filtering condition
    fn is_match(&self, value: i32) -> bool {
        value > self.threshold
    }
}

fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_items = Vec::new();

    for &item in collection {
        // Check the filtering condition and add the matching items to the filtered items list
        if condition.is_match(item) {
            filtered_items.push(item);
        }
    }

    filtered_items
}

fn main() {
    // Collection to be filtered
    let collection = vec![10, 20, 30, 40, 50];

    // Filter condition represented by the `FilterCondition` object
    let filter_condition = FilterCondition { threshold: 25 };

    // Perform filtering on the collection using the `custom_filter` function
    let filtered_result = custom_filter(&collection, &filter_condition);

    // Print the filtered result
    println!("Filtered Result: {:?}", filtered_result);
}
