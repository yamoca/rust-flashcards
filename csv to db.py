import csv
import sqlite3

# Replace 'your_input.csv' with the actual filename
csv_filename = 'sampledata.csv'
db_filename = 'output_database.sqlite'

# Connect to the SQLite database
conn = sqlite3.connect(db_filename)
cursor = conn.cursor()

# Read the CSV file and create a table
with open(csv_filename, 'r') as csv_file:
    csv_reader = csv.reader(csv_file)
    header = next(csv_reader)  # Assuming the first row is the header
    columns = ', '.join(header)
    cursor.execute(f'CREATE TABLE IF NOT EXISTS data ({columns})')

    # Insert data into the table
    for row in csv_reader:
        placeholders = ', '.join(['?' for _ in row])
        cursor.execute(f'INSERT INTO data VALUES ({placeholders})', row)

# Commit changes and close the connection
conn.commit()
conn.close()

print(f'Data from {csv_filename} has been successfully imported into {db_filename}.')
