#include <stdio.h>
#include <string.h>
#include "clist.h"
#include <stdint.h>

CList *process_data(const char *text)
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

int32_t day0a(const char *text)
{
    CList *list = process_data(text);

    size_t *previous;
    previous = list->at(list, 0);
    size_t *current;
    int32_t counter = 0;
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

size_t window(CList *list, size_t i) {
    return *((size_t *)list->at(list, i)) + *((size_t *)list->at(list, i + 1)) + *((size_t *)list->at(list, i + 2));
}

int32_t day0b(const char *text)
{
    CList *list = process_data(text);

    size_t previous_sum = window(list, 0);
    size_t current_sum = 0;
    int32_t counter = 0;

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
