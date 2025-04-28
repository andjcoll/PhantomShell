import json
import argparse
import math


def process_spaces(spaces, parent_class="top-level"):
    result = []
    for space in spaces:
        # Base data for both functions and impls
        base = {
            "class": parent_class,
            "method": space.get("name"),
            "n1": space["metrics"].get("halstead", {}).get("n1", 0),
            "n2": space["metrics"].get("halstead", {}).get("n2", 0),
            "N1": space["metrics"].get("halstead", {}).get("N1", 0),
            "N2": space["metrics"].get("halstead", {}).get("N2", 0),
            "N": space["metrics"]
            .get("halstead", {})
            .get("estimated_program_length", 0),
        }

        # If it's a top-level function, append directly to result
        if space["kind"] == "function":
            result.append(base)

        # If it's an implementation ("impl"), recursively process subspaces
        elif space["kind"] == "impl":
            result.extend(
                process_spaces(space.get("spaces", []), parent_class=space.get("name"))
            )

    return result


def aggregate_class_data(method_data):
    class_data = {}

    # Aggregate n1, n2, N1, N2 for each class
    for item in method_data:
        class_name = item["class"]
        n1, n2, N1, N2 = item["n1"], item["n2"], item["N1"], item["N2"]

        if class_name not in class_data:
            class_data[class_name] = {
                "total_n1": 0,
                "total_n2": 0,
                "total_N1": 0,
                "total_N2": 0,
            }

        class_data[class_name]["total_n1"] += n1
        class_data[class_name]["total_n2"] += n2
        class_data[class_name]["total_N1"] += N1
        class_data[class_name]["total_N2"] += N2

    return class_data


def calculate_fault_prediction(n1, n2, N1, N2):
    return ((N1 + N2) * math.log2(n1 + n2)) / 3000 if (n1 + n2) > 0 else 0


def main():
    # Set up CLI argument parsing
    parser = argparse.ArgumentParser(description="Process JSON data from a file.")
    parser.add_argument("filename", help="Path to the JSON file")

    args = parser.parse_args()

    # Read the JSON file
    with open(args.filename, "r") as file:
        data = json.load(file)

    # Process the spaces (methods)
    method_data = process_spaces(data.get("spaces", []))

    # Print the result for methods (table per method)
    for item in method_data:
        print(json.dumps(item, indent=2))

    # Aggregate the data by class and calculate fault prediction
    class_data = aggregate_class_data(method_data)

    # Print the result for classes with fault predictions
    for cls, data in class_data.items():
        fault_prediction = calculate_fault_prediction(
            data["total_n1"], data["total_n2"], data["total_N1"], data["total_N2"]
        )
        print(
            json.dumps(
                {
                    "class": cls,
                    "total_n1": data["total_n1"],
                    "total_n2": data["total_n2"],
                    "fault_prediction": f"{fault_prediction:.2f}",
                },
                indent=2,
            )
        )
        # print(f"  Total n1: {data['total_n1']}")
        # print(f"  Total n2: {data['total_n2']}")
        # print(f"  Fault Prediction: {fault_prediction:.2f}")
        # print()  # Blank line between classes
        #


if __name__ == "__main__":
    main()
