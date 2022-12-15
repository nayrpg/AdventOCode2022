import os
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
    
main()
