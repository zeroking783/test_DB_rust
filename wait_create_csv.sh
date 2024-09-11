#!/bin/sh

# Ожидание файла в /var/lib/DB_test, создаваемого create_csv
while [ ! -f /var/lib/DB_test/some_output_file.csv ]; do
  echo "Waiting for create_csv to finish..."
  sleep 5
done

echo "File detected, proceeding with send_data"
