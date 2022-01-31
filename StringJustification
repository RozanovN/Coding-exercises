#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>

void clearTheString(char string[]) {
    int i;

    for (i = 0; i < strlen(string) - 1; i++) {
        string[i] = ' ';
    }
}

void addWhiteSpaceToTheBeginningOfALine(char string[], int *currentLineLength, int *lineIndex) {
    string[*lineIndex] = ' ';
    *currentLineLength = *currentLineLength + 1;
    *lineIndex = *lineIndex + 1;
}

void addTokenToTheLine(char *token, char line[], int *lineIndex, int *currentLineLength, int *wordsInLine) {
    for (int i = 0; i < strlen(token); i++) {
        line[*lineIndex] = token[i];
        *lineIndex = *lineIndex + 1;
        *currentLineLength = *currentLineLength + 1;
    }
    ++*wordsInLine;
}

void justifyOneWordLine(char singleLine[], const int *lineLength, int *lineIndex, int *currentLineLength) {
    char copyOfSingleLine[*lineLength];
    strcpy(copyOfSingleLine, singleLine);
    *lineIndex = 0;
    int i = 0;

    singleLine[0] = ' ';
    ++*currentLineLength;
    for (i = 1; i < strlen(singleLine); i++) {
        singleLine[i] = copyOfSingleLine[*lineIndex];
        ++*lineIndex;
    }
    int numOfSpaces = 0;
    int spacesToAdd = (*lineLength - *currentLineLength) / 2;
    while (numOfSpaces < spacesToAdd) {
        strcpy(copyOfSingleLine, singleLine);
        *lineIndex = 0;
        i = 0;
        while (i < *currentLineLength){
            if (isspace(copyOfSingleLine[*lineIndex]) && *currentLineLength != *lineLength &&
            !isspace(copyOfSingleLine[*lineIndex + 1])) {
                singleLine[i] = ' ';
                singleLine[++i] = ' ';
                *currentLineLength += 1;
                ++*lineIndex;
                ++i;
                ++numOfSpaces;
            }else {
                singleLine[i] = copyOfSingleLine[*lineIndex];
                ++i;
                ++*lineIndex;
            }
        }
    }
}

void justifyLine(char singleLine[], int *currentLineLength, const int *lineLength, int *lineIndex) {
    while (*currentLineLength != *lineLength) {
        char copyOfSingleLine[*lineLength];
        strcpy(copyOfSingleLine, singleLine);
        *lineIndex = 0;
        int i = 0;
        while (i < *currentLineLength){
            if (isspace(copyOfSingleLine[*lineIndex]) && *currentLineLength != *lineLength
            && !isspace(copyOfSingleLine[*lineIndex + 1])) {
                singleLine[i] = ' ';
                singleLine[++i] = ' ';
                *currentLineLength += 1;
                ++*lineIndex;
                ++i;
            }else {
                singleLine[i] = copyOfSingleLine[*lineIndex];
                ++i;
                ++*lineIndex;
            }
        }
    }
}

void resetTheLine(char Line[], char *token, int *currentLineLength, int *lineIndex, const int *lineLength,
                  int *wordsInALine){
    *currentLineLength = 0;
    *lineIndex = 0;
    for (int i = 0; i < *lineLength; i++) {
        if (i < strlen(token)) {
            Line[i] = token[i];
        } else {
            Line[i] = ' ';
        }
    }
    *wordsInALine = 0;
}

void formatAndPrintParagraph(char *paragraph, int lineLength) {
    char copyOfParagraph[strlen(paragraph)];
    strcpy(copyOfParagraph, paragraph);
    char* token = strtok(copyOfParagraph, " ");
    int lineIndex = 0;
    int currentLineLength = 0;
    char singleLine[lineLength];
    int wordsInALine = 0;

    clearTheString(singleLine);
    while (token != NULL) {
        if (strlen(token) > lineLength) {
            perror("One of the words in your paragraph is bigger than the length of the line");
            exit(EXIT_FAILURE);
        }
        if (lineIndex > 0 && currentLineLength + 1 + strlen(token) <= lineLength) {
            addWhiteSpaceToTheBeginningOfALine(singleLine, &currentLineLength, &lineIndex);
        }
        if (currentLineLength + strlen(token) <= lineLength) {
            addTokenToTheLine(token, singleLine, &lineIndex, &currentLineLength,
                              &wordsInALine);
            token = strtok(NULL, " ");
        }else {
            if (wordsInALine == 1) {
                justifyOneWordLine(singleLine, &lineLength, &lineIndex,
                                   &currentLineLength);
            }
            else if (currentLineLength != lineLength) {
                justifyLine(singleLine, &currentLineLength,&lineLength,&lineIndex);
            }
            printf("%s\n", singleLine);
            resetTheLine(singleLine, token, &currentLineLength,
                         &lineIndex, &lineLength, &wordsInALine);
        }
    }
    if (wordsInALine == 1) {
        justifyOneWordLine(singleLine, &lineLength, &lineIndex, &currentLineLength);
    }
    else if (currentLineLength != lineLength) {
        justifyLine(singleLine, &currentLineLength, &lineLength, &lineIndex);
    }
    printf("%s\n", singleLine);
}

int main() {
    return 0;
}
