import json
import argparse

def process_spaces(spaces, parent_class="top-level"):
    result = []
    for space in spaces:
        # Base data for both functions and impls
        base = {
            "class": parent_class,
            "method": space.get("name"),
            "loc": space["metrics"].get("loc", {}).get("lloc", 0),
            "comments": space["metrics"].get("loc", {}).get("cloc", 0),
            "cyclomatic_sum": space["metrics"].get("cyclomatic", {}).get("sum", 0),
        }
        
        # If it's a top-level function, append directly to result
        if space["kind"] == "function":
            result.append(base)
        
        # If it's an implementation ("impl"), recursively process subspaces
        elif space["kind"] == "impl":
            result.extend(process_spaces(space.get("spaces", []), parent_class=space.get("name")))
    
    return result

def main():
    # Set up CLI argument parsing
    parser = argparse.ArgumentParser(description="Process JSON data from a file.")
    parser.add_argument('filename', help="Path to the JSON file")
    
    args = parser.parse_args()
    
    # Read the JSON file
    with open(args.filename, "r") as file:
        data = json.load(file)
    
    # Process the spaces
    processed_data = process_spaces(data.get("spaces", []))
    
    # Print the result
    for item in processed_data:
        print(json.dumps(item, indent=2))

if __name__ == "__main__":
    main()
