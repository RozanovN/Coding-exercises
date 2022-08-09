// Reverse bits of a given 32 bits unsigned integer.
uint32_t reverseBits(uint32_t n) {
    uint32_t result = n;
    const uint32_t one = 1;
    for (int i = 32; i > 16; i--) {
        if ((n & one<<(i-1)) > 0 && (n & one<<(32-i)) < 1) {
            result = result - (one<<(i-1)) + (one<<(32-i));
        } else if ((n & one<<(i-1)) < 1 && (n & one<<(32-i)) > 0) {
            result = result + (one<<(i-1)) - (one<<(32-i));
        }
    }
    return result;
}
