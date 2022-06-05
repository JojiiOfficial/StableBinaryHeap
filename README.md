# StableBinaryHeap
Wrapper around rusts BinaryHeap but preserves insertion order for equal items

# Limitations
You can only push `usize::MAX + 1` times (32bit: 4294967296 or 64bit: 18446744073709551616 times)
