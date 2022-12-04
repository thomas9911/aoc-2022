// inclusive range

#include <stdlib.h>
#include <string.h>

struct IncRange
{
    int from;
    int till;
};

void print_inc_range(struct IncRange range)
{
    printf("from: %i, till: %i\n", range.from, range.till);
}

struct Either
{
    struct IncRange left;
    struct IncRange right;
};

void print_either(struct Either either)
{
    print_inc_range(either.left);
    print_inc_range(either.right);
    printf("\n");
}

void print_buffer(char *buffer)
{
    for (int i = 0; i < 64; i++)
    {
        if (buffer[i] == 0)
        {
            printf(".");
        }
        else
        {
            printf("%c", buffer[i]);
        }
    }

    printf("\n");
}

void clear_buffer(char *buffer)
{
    memset(buffer, 0, 64);
}

struct Either parse_line(const char *text)
{
    char ch;
    int number;
    char number_data[64] = {'\0'};
    int at = 0;
    int left_or_right = 1; // left = 1 right = 2

    struct Either result;
    struct IncRange assignment;

    for (int i = 0; i < strlen(text) + 1; i++)
    {
        ch = *(text + i);
        if (ch == '-')
        {
            number = atoi(number_data);
            clear_buffer(number_data);
            at = 0;

            assignment.from = number;

            continue;
        }

        if (ch == ',' || i == strlen(text))
        {
            number = atoi(number_data);
            clear_buffer(number_data);
            at = 0;

            assignment.till = number;

            if (left_or_right == 1)
            {
                result.left = assignment;
            }
            else
            {
                result.right = assignment;
            }

            left_or_right = 2;
            continue;
        }

        number_data[at] = ch;
        at++;
    }

    return result;
}

int determine_fully_contains(struct Either result)
{
    if (result.left.from <= result.right.from && result.left.till >= result.right.till)
    {
        return 1;
    }

    if (result.right.from <= result.left.from && result.right.till >= result.left.till)
    {
        return 1;
    }

    return 0;
}

int determine_overlap(struct Either result)
{
    if (result.left.till >= result.right.from && result.left.from <= result.right.from)
    {
        return 1;
    }

    if (result.right.till >= result.left.from && result.right.from <= result.left.from)
    {
        return 1;
    }

    return 0;
}

long calculate(const char *text, int score_function(struct Either))
{
    const char seperator[2] = "\n";
    char *line;
    struct Either assignment;
    int overlapped = 0;

    line = strtok(text, seperator);

    while (line != NULL)
    {
        assignment = parse_line(line);

        if (score_function(assignment) == 1)
        {
            overlapped += 1;
        }

        line = strtok(NULL, seperator);
    }

    return overlapped;
}

long day04a(const char *text)
{
    return calculate(text, determine_fully_contains);
}

long day04b(const char *text)
{
    return calculate(text, determine_overlap);
}
