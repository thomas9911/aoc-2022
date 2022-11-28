#include <stdio.h>
#include <string.h>
#include "clist.h"
#include <stdlib.h>

CList *process_data_day_1(const char *text)
{
    size_t n = sizeof(size_t);
    CList *list = CList_init(n);
    void *obj = NULL;
    char str_integer[6];
    str_integer[0] = '\0';
    size_t integer;
    size_t text_size = strlen(text);

    for (size_t i = 0; i <= text_size; i++)
    {
        if (i == text_size || text[i] == '\n')
        {
            obj = &integer;
            sscanf(str_integer, "%zu", obj);
            // printf("%zu !!\n", *((size_t*)obj));
            list->add(list, obj);
            str_integer[0] = '\0';
        }
        else
            strncat(str_integer, &text[i], 1);
    };

    return list;
}

long day0a(const char *text)
{
    CList *list = process_data_day_1(text);

    size_t *previous;
    previous = list->at(list, 0);
    size_t *current;
    long counter = 0;
    for (size_t i = 1; i < list->count(list); i++)
    {
        current = list->at(list, i);
        if (*current > *previous)
            counter++;
        previous = current;
    }

    list->free(list);

    return counter;
}

size_t window(CList *list, size_t i)
{
    return *((size_t *)list->at(list, i)) + *((size_t *)list->at(list, i + 1)) + *((size_t *)list->at(list, i + 2));
}

long day0b(const char *text)
{
    CList *list = process_data_day_1(text);

    size_t previous_sum = window(list, 0);
    size_t current_sum = 0;
    long counter = 0;

    for (size_t i = 1; i < (list->count(list) - 2); i++)
    {
        current_sum = window(list, i);
        // printf("%zu -> %zu\n", previous_sum, current_sum);
        if (current_sum > previous_sum)
            counter++;
        previous_sum = current_sum;
    }

    list->free(list);

    return counter;
}

#define FORWARD 'f'
#define UP 'u'
#define DOWN 'd'
#define ERROR_DIR 'e'

struct Command
{
    char direction;
    int amount;
};

CList *process_data_day_2(const char *text)
{
    size_t n = sizeof(size_t);
    CList *list = CList_init(n);
    const char seperator[2] = "\n";
    char *line;

    line = strtok(text, seperator);

    char direction;
    int amount;
    struct Command command;

    while (line != NULL)
    {
        // printf(" %s\n", line);
        if (strstr(line, "forward") != NULL)
        {
            direction = FORWARD;
        }
        else if (strstr(line, "up") != NULL)
        {
            direction = UP;
        }
        else if (strstr(line, "down") != NULL)
        {
            direction = DOWN;
        }
        else
        {
            // error
            direction = ERROR_DIR;
        }

        amount = atoi((strstr(line, " ") + 1));
        command.amount = amount;
        command.direction = direction;
        list->add(list, &command);

        line = strtok(NULL, seperator);
    }

    return list;
}

long day0c(const char *text)
{
    CList *list = process_data_day_2(text);
    int depth = 0;
    int position = 0;
    for (int i = 0; i < list->count(list); i++)
    {
        struct Command *obj = list->at(list, i);
        // printf("%c: %i\n", obj->direction, obj->amount);
        switch (obj->direction)
        {
        case FORWARD:
            position += obj->amount;
            break;

        case UP:
            depth -= obj->amount;
            break;

        case DOWN:
            depth += obj->amount;
            break;

        default:
            break;
        }
    }

    return depth * position;
}

long day0d(const char *text)
{
    CList *list = process_data_day_2(text);
    int depth = 0;
    int position = 0;
    int aim = 0;
    for (int i = 0; i < list->count(list); i++)
    {
        struct Command *obj = list->at(list, i);
        // printf("%c: %i\n", obj->direction, obj->amount);
        switch (obj->direction)
        {
        case FORWARD:
            position += obj->amount;
            depth += aim * (obj->amount);
            break;

        case UP:
            aim -= obj->amount;
            break;

        case DOWN:
            aim += obj->amount;
            break;

        default:
            break;
        }
    }

    return depth * position;
}
