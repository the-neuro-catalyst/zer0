#!/bin/bash

echo "🧹 Cleaning up old test data..."
rm -rf test_data/
mkdir -p test_data
cd test_data

echo "Generating ZERO Test Infrastructure..."

# ==========================================
# 1. STANDARD SUITE (Sanitized)
# ==========================================
echo "[Section] Generating Standard Suite..."

# CSV
echo " - staff.csv"
echo "id,name,username,department,role,salary,status,hire_date" > staff.csv
for i in {1..100}; do
    echo "$i,User_$i,user_handle_$i,Dept_$((i%5)),Role_$((i%3)),$((50000 + i*100)),Active,2023-01-$(( (i%28)+1 ))" >> staff.csv
done

# JSON
echo " - complex_data.json"
cat > complex_data.json << 'EOF'
{
  "simulation": {
    "id": "sim-89201",
    "timestamp": "2024-02-07T12:00:00Z",
    "active": true,
    "parameters": { "threshold": 0.85, "timeout_ms": 5000, "retry_policy": "exponential_backoff" }
  },
  "metrics": {
    "cpu_usage": [12.5, 15.2, 45.1, 10.0, 11.2],
    "memory_usage": [1024, 1048, 1056, 1100, 1024]
  },
  "users": [
    { "id": 1, "access": ["read", "write"], "preferences": { "theme": "dark" } },
    { "id": 2, "access": ["read"], "preferences": { "theme": "light" } }
  ]
}
EOF

# YAML
echo " - deploy.yaml"
cat > deploy.yaml << 'EOF'
application:
  name: production-service
  version: 2.5.1
  replicas: 5
  environment:
    - name: NODE_ENV
      value: production
EOF

# TOML
echo " - settings.toml"
cat > settings.toml << 'EOF'
title = "Application Configuration"
[server]
host = "0.0.0.0"
port = 8080
[database]
enabled = true
EOF

# XML
echo " - data.xml"
cat > data.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <item type="hardware" stock="50">Server Blade</item>
   <item type="software" stock="unlimited">License Key</item>
</catalog>
EOF

# Logs
echo " - app.log"
for i in {1..50}; do
    echo "2024-02-07 12:00:$(printf "%02d" $((i%60))) [INFO] [Thread-$((i%5))] Processing request ID #$((1000+i))" >> app.log
done

# ==========================================
# 2. PII SUITE (Sensitive Data for Redaction Testing)
# ==========================================
echo "[Section] Generating PII Suite..."

# CSV with PII
echo " - staff_pii.csv"
cat > staff_pii.csv << 'EOF'
id,name,email,ssn,phone,address,credit_card
1,John Doe,john.doe@example.com,123-45-6789,555-0101,123 Main St,4532-1234-5678-9010
2,Jane Smith,jane.smith@example.com,987-65-4321,555-0102,456 Oak Ave,5425-9876-5432-1098
3,Bob Johnson,bob.j@example.com,456-78-9012,555-0103,789 Pine Rd,3734-4567-8901-2345
4,Alice Williams,alice.w@example.com,234-56-7890,555-0104,321 Elm St,6011-5678-9012-3456
5,Charlie Brown,charlie.b@example.com,678-90-1234,555-0105,654 Maple Dr,5105-6789-0123-4567
EOF

# JSON with PII
echo " - complex_data_pii.json"
cat > complex_data_pii.json << 'EOF'
{
  "users": [
    {
      "id": 1,
      "name": "John Doe",
      "email": "john.doe@example.com",
      "ssn": "123-45-6789",
      "phone": "555-0101",
      "salary": 75000
    },
    {
      "id": 2,
      "name": "Jane Smith",
      "email": "jane.smith@example.com",
      "ssn": "987-65-4321",
      "phone": "555-0102",
      "salary": 82000
    }
  ]
}
EOF

# Text with PII
echo " - notes_pii.txt"
cat > notes_pii.txt << 'EOF'
John Doe (SSN: 123-45-6789) called at 555-0101 regarding account access.
Jane Smith (SSN: 987-65-4321) reported issue from IP 192.168.1.100.
Credit card 4532-1234-5678-9010 needs verification.
Patient records: Name John, DOB 01/15/1985, MRN 123456
Account holder: Alice Williams, License #DL-2023-456789
EOF

# ==========================================
# 3. BINARIES AND ARCHIVES
# ==========================================
echo "[Section] Generating Binaries and Archives..."

# SQLite database (CLEAN VERSION - no recreates)
echo " - system.db"
rm -f system.db  # Remove if exists
sqlite3 system.db << 'EOF'
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS logs (
  id INTEGER PRIMARY KEY,
  user_id INTEGER,
  action TEXT,
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id)
);

INSERT INTO users (name, email) VALUES 
  ('John Doe', 'john@example.com'),
  ('Jane Smith', 'jane@example.com'),
  ('Bob Johnson', 'bob@example.com');

INSERT INTO logs (user_id, action) VALUES
  (1, 'login'),
  (2, 'create_resource'),
  (3, 'delete_resource');
EOF

# SQLite database with PII (CLEAN VERSION)
echo " - system_pii.db"
rm -f system_pii.db  # Remove if exists
sqlite3 system_pii.db << 'EOF'
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT UNIQUE,
  ssn TEXT,
  credit_card TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS audit_logs (
  id INTEGER PRIMARY KEY,
  user_id INTEGER,
  action TEXT,
  ip_address TEXT,
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id)
);

INSERT INTO users (name, email, ssn, credit_card) VALUES
  ('John Doe', 'john@example.com', '123-45-6789', '4532-1234-5678-9010'),
  ('Jane Smith', 'jane@example.com', '987-65-4321', '5425-9876-5432-1098'),
  ('Bob Johnson', 'bob@example.com', '456-78-9012', '3734-4567-8901-2345');

INSERT INTO audit_logs (user_id, action, ip_address) VALUES
  (1, 'login', '192.168.1.100'),
  (2, 'data_access', '192.168.1.101'),
  (3, 'export_data', '192.168.1.102');
EOF

# Create tar.gz archives
echo " - Creating archives..."
tar -czf standard_suite.tar.gz staff.csv complex_data.json deploy.yaml settings.toml data.xml app.log 2>/dev/null || true
tar -czf pii_suite.tar.gz staff_pii.csv complex_data_pii.json notes_pii.txt 2>/dev/null || true
tar -czf database_suite.tar.gz system.db system_pii.db 2>/dev/null || true

# ==========================================
# 4. BENCHMARK DATA
# ==========================================
echo "[Section] Generating Benchmark Data..."

# Create benchmark directory structure
mkdir -p benchmark/normal
mkdir -p benchmark/problematic

# Normal benchmark data
echo " - benchmark/normal/large_clean.json"
echo "[" > benchmark/normal/large_clean.json
for i in {1..1000}; do
    if [ $i -lt 1000 ]; then
        echo "  {\"id\": $i, \"name\": \"User_$i\", \"email\": \"user$i@example.com\", \"active\": true, \"score\": $((RANDOM % 100))}," >> benchmark/normal/large_clean.json
    else
        echo "  {\"id\": $i, \"name\": \"User_$i\", \"email\": \"user$i@example.com\", \"active\": true, \"score\": $((RANDOM % 100))}" >> benchmark/normal/large_clean.json
    fi
done
echo "]" >> benchmark/normal/large_clean.json

echo " - benchmark/normal/clean_data.csv"
echo "id,name,role,salary" > benchmark/normal/clean_data.csv
for i in {1..1000}; do
    role=$(echo "Dev Manager QA Designer" | tr ' ' '\n' | sort -R | head -1)
    salary=$((50000 + RANDOM % 100000))
    echo "$i,User_$i,$role,$salary" >> benchmark/normal/clean_data.csv
done

# Problematic benchmark data
echo " - benchmark/problematic/malformed.json"
echo '[{"id": 1, "name": "Valid"}, {"id": 2, "name": "Missing Brace"' > benchmark/problematic/malformed.json

echo " - benchmark/problematic/schema_mismatch.csv"
cat > benchmark/problematic/schema_mismatch.csv << 'EOF'
id,name,email
1,Alice,alice@example.com
2,Bob
3,Charlie,charlie@example.com,ExtraData
EOF

# ==========================================
# SUMMARY
# ==========================================
cd ..
echo ""
echo "✅ Test Data Suite Ready in ./test_data"
echo ""
echo "📋 Generated Files:"
ls -lh test_data/ | tail -n +2 | awk '{print "  - " $9 " (" $5 ")"}'
echo ""
echo "📊 Test Data Structure:"
find test_data -type f -o -type d | head -20 | sed 's/^/  /'
echo ""
echo "✨ Ready for CI/CD testing!"