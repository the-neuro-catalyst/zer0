
import csv
import os
import random
import datetime

def generate_test_data(num_rows=100, output_dir="extensions/zero-mcp-gemini/test_data"):
    """
    Generates a CSV file with test data.
    """
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    output_file = os.path.join(output_dir, "test_data.csv")
    
    headers = ["id", "name", "description", "value", "timestamp"]
    
    with open(output_file, 'w', newline='') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(headers)
        
        for i in range(1, num_rows + 1):
            row_id = i
            name = f"Item_{i:03d}"
            description = f"Description for item {i}"
            value = round(random.uniform(10.0, 1000.0), 2)
            timestamp = datetime.datetime.now().isoformat()
            writer.writerow([row_id, name, description, value, timestamp])
            
    print(f"Generated {num_rows} rows of test data to {output_file}")

if __name__ == "__main__":
    generate_test_data()
