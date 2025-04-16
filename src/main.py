import rust_py_fastembed

# Example 1: Process an integer array
data = [1, 2, 3, 4, 5]
print("Original integer array:", data)
processed_data = rust_py_fastembed.process_int_array(data)
print("Processed integer array:", processed_data)

# Example 2: Process a string array
string_data = ["hello", "world"]
print("\nOriginal string array:", string_data)
processed_string_data = rust_py_fastembed.process_string_array(string_data)
print("Processed string array:", processed_string_data)

# Example 3: Process a dictionary
dict_data = {1: "one", 2: "two"}
print("\nOriginal dictionary:", dict_data)
processed_dict_data = rust_py_fastembed.process_dict(dict_data)
print("Processed dictionary:", processed_dict_data)
