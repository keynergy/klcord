#!/bin/python3
import os

for n in os.listdir("./layouts"):
    print(n)
    f = open("./layouts/"+n, 'r')
    lines=f.readlines()

    name = lines[0].strip()
    author = lines[4].strip()

    rows = lines[1:4]

    matrix = ""

    print(name, author)
    y = 0
    for row in rows:
        row = row.strip().split()
        row = '", "'.join(row)
        start = ""
        end = ","
        if y == 0:
            start = "["
        if y == 2:
            end = "]"
        row = start + '["' + row + '"]' + end
        y += 1
        matrix += row + "\n"

    print(matrix)

    out = '''name = "%s"
author = "%s"
link = ""
year = 2022

[formats.standard]
matrix = %s
map = {}
home_row = 1''' %(name, author, matrix)

    newfile = open("newlayouts/" + n + ".toml", "a")
    newfile.write(out)
    newfile.close()

    f.close()
