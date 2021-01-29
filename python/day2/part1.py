f = open("password-list.txt", "r")
freadlines = f.readlines()
array = []
for i in freadlines:
    array.append(i.strip("\n").split(" "))

count = 0
for line in array:
    count_of_occur = line[2].count(line[1][0])
    max_occurence = int(line[0].split("-")[1])
    min_occurence = int(line[0].split("-")[0])
    if count_of_occur >= min_occurence and count_of_occur <= max_occurence:
        count += 1

print(f"Total count is: {count}")