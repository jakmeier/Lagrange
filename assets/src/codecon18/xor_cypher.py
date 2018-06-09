import sys

def decode(msg, key):
  return ''.join([
      decode_digit(key, msg[i:i+2])
      for i in range(0, len(msg), 2)
    ]);
   
def decode_digit(key, doubleHexDigit):
  return chr(key ^ int(doubleHexDigit, 16));
  
'''
def my_decode(msg, key): return ''.join([chr(key ^ int(msg[i:i+2], 16)) for i in range(0, len(msg), 2)])
'''  

def evaluate_credibility(str):
  tokens = str.split()
  words = [ word for token in tokens for word in token.split("-") ]
  return sum(score_words(words))
  
def score_words(words):
   for w in words:
      if w.isalnum(): 
        if len(w) > 1 and w[0].isupper() and w[-1].islower():
          yield 25
        else:
          yield 5
          
def solve(input):
  messages = [decode(input, key) for key in range(128)]
  return max(messages, key=evaluate_credibility)
  
data = sys.stdin.read().splitlines()
for line in data:
    print (solve(line))
    
''' 
    print([(str(evaluate_credibility(msg)), msg) for msg in messages])
'''

