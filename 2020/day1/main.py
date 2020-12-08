#!/usr/bin/python3
import sys

def puzzle1(c):
    for i1 in c:
        for i2 in c:
            if i1+i2 == 2020:
                print(f"{i1} * {i2} = {i1*i2} ")

def puzzle2(c):
    for i1 in c:
        for i2 in c:
            for i3 in c:
                if i3+i1+i2 == 2020:
                    print(f"{i1} * {i2} * {i3} = {i3*i1*i2} ")



def main():
    name = sys.argv[1]
    print(f"reading file {name}")
    
    content = []
    for it in open(name):
        content.append(int(it))

    puzzle1(content)
    puzzle2(content)


if __name__ == "__main__":
    main()


# ________________________________________________________
# Executed in   51,64 secs   fish           external
   # usr time   50,69 secs    2,02 millis   50,68 secs
   # sys time    0,49 secs   77,37 millis    0,42 secs

