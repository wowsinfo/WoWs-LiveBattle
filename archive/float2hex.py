"""
Converts a floating point number to a hex string.
"""

import struct

def float2hex(f):
    # convert to little endian and all captial
    return struct.pack('<f', f).hex().upper()

if __name__ == '__main__':
    number = input('Enter a floating point number: ')
    print(float2hex(float(number)))
