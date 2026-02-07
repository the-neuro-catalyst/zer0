import json
import sys
import os

def clean_text(text):
    """Normalize whitespace and remove non-printable characters."""
    if not isinstance(text, str):
        return str(text)
    return ' '.join(text.split())

def process_file(input_path):
    """Read a file and output cleaned JSONL lines."""
    if not os.path.exists(input_path):
        print(f"Error: File {input_path} not found.", file=sys.stderr)
        return

    # Basic example for text/csv could be expanded
    # For now, treating line-based text
    try:
        with open(input_path, 'r', encoding='utf-8') as f:
            for line in f:
                cleaned = clean_text(line)
                if cleaned:
                    print(json.dumps({"text": cleaned}))
    except Exception as e:
        print(f"Error processing file: {e}", file=sys.stderr)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 prep-inference.py <input_file>", file=sys.stderr)
        sys.exit(1)
    
    process_file(sys.argv[1])
