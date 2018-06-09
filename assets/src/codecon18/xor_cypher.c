#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_LENGTH 256

char* decode(char* input, char key) {
  char* output = malloc(MAX_LENGTH + 1);
  int i;
  for(i = 0; input[i*2] != '\0' ; i ++) {
    char hexdigit[3] = { input[2*i], input[2*i+1], '\0'};
    long int character = strtol(hexdigit, NULL, 16);
    output[i] = character ^ key;
  }
  output[i] = '\0';
  return output;
}

long scoreWord(char* word){
   // printf("\nWord: %s", word);
    int allAlphanumeric = -1;
    long score = 0;
    char* p = word;
    while(*p){
      allAlphanumeric &= isalnum(*(p++));
    }
    if(allAlphanumeric) {
      score += 5;
      if(isupper(*word) && islower(*(word + strlen(word) - 1))){
        score += 20;
      }
    }
    return score;
}

long evaluateCredibility(char* msg) {
  int len = strlen(msg);
  if(len <= 0){ return -1; }
  // printf("\nEval: %s", msg);
  char* msgCopy = malloc(2 * len);
  strcpy(msgCopy, msg);
  long score = 0;
  char splitOnChars[] = " \t\n\v\f\r-";
  char* word = strtok (msgCopy, splitOnChars);
  do {
    score += scoreWord(word);
  } while(word = strtok (NULL, splitOnChars));
  // printf("\n Score: %d", (int)score);
  free(msgCopy);
  return score;
}

void solve(char* input){
  long maxScore = -1;
  char* bestGuess = NULL;
  int i;
  for(i = 0; i < 128; i++) {
    // printf("\nSTART KEY %d", i);
    char* output = decode(input, i);
    long score = evaluateCredibility(output);
    if(score > maxScore) {
      maxScore = score;
      if(bestGuess != NULL){
        free(bestGuess);
      }
      bestGuess = output;
    }
    else {
      free(output);
    }
    // printf("\nKEY %d DONE\n", i);

  }
  //printf("Input = %s\nOutput = %s\n\n", input, bestGuess);
  printf("%s", bestGuess);
  free(bestGuess);
}

int main() {
    char* input = calloc(1, MAX_LENGTH+1); // "023f6c386b3f39222820326b3f246b392428206b2a6b392332262e6b3f246b392428206b2a6b392332262e6b3f232a3f6c386b39222c233f6b24256b3f22262e";
    gets(input);
    solve(input);
    return 0;
}
