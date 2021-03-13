#!/usr/bin/env python3

import re
import sys

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('filename is not specified', file=sys.stderr)
        exit()

    with open(sys.argv[1], mode='r') as f:
        src = f.read()
        src = re.sub('//.*\n', '\n', src)
        src = re.sub('/\\*.*\\*/', '', src, flags=re.DOTALL)
        src = re.sub('\n\\s*\n', '\n', src)

    print(src)
