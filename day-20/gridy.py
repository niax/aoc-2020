import sys

for y, line in enumerate(sys.stdin):
    line = line.strip()
    o = ''
    for x, c in enumerate(line):
        if x % 10 == 0:
            o += ' '
        o += c
    if y % 10 == 0:
        print ""
    print o
