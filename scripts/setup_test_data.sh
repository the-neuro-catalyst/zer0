#!/bin/bash

# ZERO Test Data Generation Suite
# Generates a comprehensive set of sanitized and PII-laden data for system validation.

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
cat <<EOF > complex_data.json
{
  "simulation": {
    "id": "sim-89201",
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
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
cat <<EOF > deploy.yaml
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
cat <<EOF > settings.toml
title = "Application Configuration"
[server]
host = "0.0.0.0"
port = 8080
[database]
enabled = true
EOF

# XML
echo " - data.xml"
cat <<EOF > data.xml
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <item type="hardware" stock="50">Server Blade</item>
   <item type="software" stock="unlimited">License Key</item>
</catalog>
EOF

# Logs
echo " - app.log"
for i in {1..50}; do
    echo "$(date '+%Y-%m-%d %H:%M:%S') [INFO] [Thread-$((i%5))] Processing request ID #$((1000+i))" >> app.log
done

# ==========================================
# 2. PII SUITE (Sensitive Data for Redaction Testing)
# ==========================================
echo "[Section] Generating PII Suite..."

# CSV with PII
echo " - staff_pii.csv"
echo "id,name,email,ssn,phone,address,credit_card,license_number,passport_number" > staff_pii.csv
for i in {1..50}; do
    echo "$i,User_$i,user_$i@example.com,$((100+i%90))$((10+i%90))$((1000+i%9000)),555-$((100+i%900))-$((1000+i%9000)),$((1000+i)) Main St, Anytown, CA,4111111111111111,DL$((100000+i)),PP$((1000000+i))" >> staff_pii.csv
done
 
# JSON with PII
echo " - complex_data_pii.json"
cat <<EOF > complex_data_pii.json
{
  "users": [
    {
      "id": 1,
      "name": "John Doe",
      "email": "john.doe@example.com",
      "ssn": "123-45-6789",
      "credit_card": "4111111111111111"
    },
    {
      "id": 2,
      "name": "Jane Smith",
      "email": "jane.smith@example.com",
      "ssn": "987-65-4321",
      "credit_card": "5500000000000004"
    }
  ]
}
EOF

# Text with PII
echo " - notes_pii.txt"
cat <<EOF > notes_pii.txt
User Audit Record:
John Doe (john.doe@example.com) | SSN: 123-45-6789 | CC: 4111111111111111
Jane Smith (jane.smith@example.com) | SSN: 987-65-4321 | CC: 5500000000000004
EOF

# ==========================================
# 3. BINARIES & ARCHIVES
# ==========================================
echo "[Section] Generating Binaries and Archives..."

# Image
echo "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" | base64 -d > pixel.gif

# Compressed
cp staff.csv to_compress.txt
gzip -c to_compress.txt > data.gz
if command -v zip &> /dev/null; then
    zip -q data_pii.zip staff_pii.csv
fi
rm to_compress.txt

# SQLite
echo " - system.db"
if command -v sqlite3 &> /dev/null; then
    sqlite3 system.db "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT); INSERT INTO users (name, email) VALUES ('Admin', 'admin@neurocatalyst.zero');"
elif command -v python3 &> /dev/null; then
    python3 -c "import sqlite3; conn = sqlite3.connect('system.db'); c = conn.cursor(); c.execute('CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)'); c.execute(\"INSERT INTO users (name, email) VALUES ('Admin', 'admin@neurocatalyst.zero')\"); conn.commit(); conn.close()"
fi

# SQLite with PII
echo " - system_pii.db"
if command -v sqlite3 &> /dev/null; then
    sqlite3 system_pii.db "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT, ssn TEXT, credit_card TEXT); INSERT INTO users (name, email, ssn, credit_card) VALUES ('John Doe', 'john.doe@example.com', '123-45-6789', '4111111111111111');"
elif command -v python3 &> /dev/null; then
    python3 -c "import sqlite3; conn = sqlite3.connect('system_pii.db'); c = conn.cursor(); c.execute('CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT, ssn TEXT, credit_card TEXT)'); c.execute(\"INSERT INTO users (name, email, ssn, credit_card) VALUES ('John Doe', 'john.doe@example.com', '123-45-6789', '4111111111111111')\"); conn.commit(); conn.close()"
fi

echo "Test Data Suite Ready in ./test_data"
