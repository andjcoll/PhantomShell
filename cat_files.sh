find src -name "*.rs" -type f -print0 | while IFS= read -r -d '' file; do
    echo "// ==> FILE: $file <=="
    cat "$file"
    echo ""
done 
