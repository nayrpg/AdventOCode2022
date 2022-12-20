import os
import sys
def main():
    max_cals = 0
    with open("input") as file_in:
        total = 0
        for line in file_in:
            #print("HERE {}").format(total)
            if line.strip() == "":
                  max_cals = max(max_cals,total)
                  total = 0 
            else:
                total += int(line)
    print(max_cals)
def main2():
    top_cals = 0
    second_cals = 0
    third_cals = 0
    with open("input") as file_in:
        total = 0
        for line in file_in:
            #print("HERE {}").format(total)
            if line.strip() == "":
                if total > third_cals:
                    third_cals = total
                if third_cals > second_cals:
                    tmp = second_cals
                    second_cals = third_cals
                    third_cals = tmp
                if second_cals > top_cals:
                    tmp = top_cals
                    top_cals = second_cals
                    second_cals = tmp
                total = 0 
            else:
                total += int(line)
    print("First: {}".format(top_cals))
    print("Second: {}".format(second_cals))
    print("Third: {}".format(third_cals))
    print("Total: {}".format(top_cals+second_cals+third_cals))
part = int(sys.argv[1])
if part == 1:
    main()
elif part == 2:
    main2()
