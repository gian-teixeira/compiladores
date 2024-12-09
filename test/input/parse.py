from sys import argv
import json

result = []

tokens = json.load(open(argv[1]))
for token in tokens:
    for t,v in token['token_type'].items():
        result.append([(t,v), '\t'])

with open(argv[2]) as output:
    for i,line in enumerate(output):
        line = line.split()
        result[i].append([(line[5],line[3])])

for r in result:
    print(r)
