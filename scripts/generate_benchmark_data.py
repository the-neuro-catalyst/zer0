import os
import json
import csv
import random
import string
import sys

sys.setrecursionlimit(5000)

BENCHMARK_DIR = "test_data/benchmark"
NORMAL_DIR = os.path.join(BENCHMARK_DIR, "normal")
PROBLEMATIC_DIR = os.path.join(BENCHMARK_DIR, "problematic")

os.makedirs(NORMAL_DIR, exist_ok=True)
os.makedirs(PROBLEMATIC_DIR, exist_ok=True)

def generate_random_string(length=10):
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))

def create_normal_data():
    print("Generating Normal Data...")
    
    # 1. Large Clean JSON
    print("- large_clean.json")
    data = []
    for i in range(10000):
        data.append({
            "id": i,
            "name": generate_random_string(20),
            "email": f"user{i}@example.com",
            "active": i % 2 == 0,
            "score": random.uniform(0, 100)
        })
    with open(os.path.join(NORMAL_DIR, "large_clean.json"), "w") as f:
        json.dump(data, f, indent=2)

    # 2. Clean CSV
    print("- clean_data.csv")
    with open(os.path.join(NORMAL_DIR, "clean_data.csv"), "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["id", "name", "role", "salary"])
        for i in range(10000):
            writer.writerow([
                i, 
                generate_random_string(15), 
                random.choice(["Dev", "Manager", "QA", "Designer"]),
                random.randint(50000, 150000)
            ])

    # 3. Nested Valid JSON
    print("- nested_valid.json")
    nested = {"level": 0, "child": None}
    current = nested
    for i in range(1, 50): # Reasonable nesting
        current["child"] = {"level": i, "child": None}
        current = current["child"]
    with open(os.path.join(NORMAL_DIR, "nested_valid.json"), "w") as f:
        json.dump(nested, f, indent=2)

def create_problematic_data():
    print("Generating Problematic Data...")

    # 1. Malformed JSON (Syntax Error)
    print("- malformed.json")
    with open(os.path.join(PROBLEMATIC_DIR, "malformed.json"), "w") as f:
        f.write('[{"id": 1, "name": "Valid"}, {"id": 2, "name": "Missing Brace"') # Intentionally broken

    # 2. Schema Mismatch CSV
    print("- schema_mismatch.csv")
    with open(os.path.join(PROBLEMATIC_DIR, "schema_mismatch.csv"), "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["id", "name", "email"]) # 3 columns
        writer.writerow(["1", "Alice", "alice@example.com"])
        writer.writerow(["2", "Bob"]) # Missing column
        writer.writerow(["3", "Charlie", "charlie@example.com", "Extra Data"]) # Extra column

    # 3. Type Error JSON (if schema enforcement is on)
    print("- type_error.json")
    data = [
        {"id": 1, "age": 25},
        {"id": 2, "age": "twenty-five"}, # String instead of int
        {"id": 3, "age": 30}
    ]
    with open(os.path.join(PROBLEMATIC_DIR, "type_error.json"), "w") as f:
        json.dump(data, f, indent=2)

    # 4. Huge Field JSON (Buffer Overflow / Memory Stress)
    print("- huge_field.json")
    huge_string = "A" * (1024 * 1024 * 50) # 50MB string
    data = {"id": 1, "payload": huge_string}
    with open(os.path.join(PROBLEMATIC_DIR, "huge_field.json"), "w") as f:
        json.dump(data, f)

    # 5. Deep Nesting (Stack Overflow)
    print("- deep_nesting.json")
    deep = {"level": 0, "child": None}
    current = deep
    for i in range(1, 2000): # Extreme nesting
        current["child"] = {"level": i, "child": None}
        current = current["child"]
    with open(os.path.join(PROBLEMATIC_DIR, "deep_nesting.json"), "w") as f:
        json.dump(deep, f)

if __name__ == "__main__":
    create_normal_data()
    create_problematic_data()
    print("Done.")
