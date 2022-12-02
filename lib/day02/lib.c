#include <stdio.h>
#include "clist.h"

#define ROCK 'r'
#define PAPER 'p'
#define SCISSORS 's'

struct Hand
{
    char opponent;
    char you;
};

char opponent(char played)
{
    switch (played)
    {
    case 'A':
        return ROCK;
        break;
    case 'B':
        return PAPER;
        break;
    case 'C':
        return SCISSORS;
        break;
    default:
        break;
    }
}

char you(char played, char opponent_play)
{
    switch (played)
    {
    case 'X':
        return ROCK;
        break;
    case 'Y':
        return PAPER;
        break;
    case 'Z':
        return SCISSORS;
        break;
    default:
        break;
    }
}

char match_outcome(char expected, char opponent_play)
{
    switch (expected)
    {
    case 'X':
        // loose
        switch (opponent_play)
        {
        case ROCK:
            return SCISSORS;
            break;
        case PAPER:
            return ROCK;
            break;
        case SCISSORS:
            return PAPER;
            break;
        default:
            break;
        }
        break;
    case 'Y':
        // draw
        return opponent_play;
        break;
    case 'Z':
        // win
        switch (opponent_play)
        {
        case ROCK:
            return PAPER;
            break;
        case PAPER:
            return SCISSORS;
            break;
        case SCISSORS:
            return ROCK;
            break;
        default:
            break;
        }
        break;
    default:
        break;
    }
}

struct Hand hand_from_line(char *line, char you_parser(char, char))
{
    struct Hand hand;

    hand.opponent = opponent(line[0]);
    hand.you = you_parser(line[2], hand.opponent);
    return hand;
}

int hand_to_score(struct Hand hand)
{
    char p1 = hand.opponent;
    char p2 = hand.you;

    int base_score = 0;
    switch (p2)
    {
    case ROCK:
        base_score = 1;
        break;
    case PAPER:
        base_score = 2;
        break;
    case SCISSORS:
        base_score = 3;
        break;
    default:
        break;
    }

    int outcome_score = 0;
    if ((p1 == ROCK && p2 == SCISSORS) || (p1 == PAPER && p2 == ROCK) || (p1 == SCISSORS && p2 == PAPER))
    {
        outcome_score = 0;
    }
    else if ((p1 == ROCK && p2 == ROCK) || (p1 == PAPER && p2 == PAPER) || (p1 == SCISSORS && p2 == SCISSORS))
    {
        outcome_score = 3;
    }
    else if ((p1 == ROCK && p2 == PAPER) || (p1 == PAPER && p2 == SCISSORS) || (p1 == SCISSORS && p2 == ROCK))
    {
        outcome_score = 6;
    }
    else
    {
        outcome_score = 0;
    }

    return base_score + outcome_score;
}

CList *parse(const char *text, char you_parser(char, char))
{

    const char seperator[2] = "\n";
    char *line;

    line = strtok(text, seperator);

    struct Hand hand;

    size_t n = sizeof(hand);
    CList *list = CList_init(n);

    while (line != NULL)
    {
        hand = hand_from_line(line, you_parser);
        list->add(list, &hand);
        line = strtok(NULL, seperator);
    }

    return list;
}

long day02a(const char *text)
{
    CList *list = parse(text, &you);
    long sum = 0;
    for (int i = 0; i < list->count(list); i++)
    {
        struct Hand *obj = list->at(list, i);
        sum += hand_to_score(*obj);
    }

    return sum;
}

long day02b(const char *text)
{
    CList *list = parse(text, &match_outcome);
    long sum = 0;
    for (int i = 0; i < list->count(list); i++)
    {
        struct Hand *obj = list->at(list, i);
        sum += hand_to_score(*obj);
    }
    return sum;
}
