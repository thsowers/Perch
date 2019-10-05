cd cli && cargo build --release     # Compile CLI
cd ../data && rm -rf db \           # Remove old index if it exists
 ../cli/target/release/perch-cli -i # Build index