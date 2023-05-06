import math
f = open("./english_bigrams.txt")
string = f.read()
string = string.split("\n")
bigrams = list(map(lambda line: line.split(' '),string))
total_of_bigrams = 0
for b in bigrams:
    total_of_bigrams = total_of_bigrams + int(b[1])


bigrams = [(b[0], int(b[1])/total_of_bigrams) for b in bigrams]

dict = {}
for l1 in range(97,123):
    for l2 in range(97,123):
        dict[chr(l1)+chr(l2)] = 0


for bigram in bigrams:
    dict[bigram[0].lower()] = math.log(bigram[1])
min_val = min(dict.values())
max_val = max(dict.values())

dict = {k: round(1000000*((v-min_val)/(max_val-min_val))) for k,v in dict.items()}
max_val = max(dict , key=dict.get)
min_val = min(dict , key=dict.get)

with open('./letter_frequency_eng1.txt', 'w') as f:
    for k,v in dict.items():
        f.write(f'{k} {v}\n')


    