#!/bin/bash
# Script to optimize GLB models for better performance

echo "Model optimization recommendations:"
echo "=================================="

echo ""
echo "Large models found (>10MB):"
find assets/models -name "*.glb" -size +10M -exec ls -lh {} +

echo ""
echo "To optimize these models, you can:"
echo "1. Install gltf-transform: npm install -g @gltf-transform/cli"
echo "2. Run optimization commands:"

find assets/models -name "*.glb" -size +10M | while read file; do
    backup_file="${file%.glb}_original.glb"
    echo "   # Backup and optimize $file"
    echo "   cp \"$file\" \"$backup_file\""
    echo "   gltf-transform optimize \"$file\" \"$file\""
done

echo ""
echo "3. Alternative manual optimizations:"
echo "   - Reduce texture sizes from 4K to 1K or 2K"
echo "   - Use texture compression (KTX2/Basis)"
echo "   - Simplify geometry (reduce polygon count)"
echo "   - Remove unnecessary animations or materials"

echo ""
echo "Current asset sizes:"
du -sh assets/models/* | sort -hr