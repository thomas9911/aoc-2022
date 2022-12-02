#include <stdlib.h>
#define AMOUNT_OF_MAXIMUM 3

long day01a(const char *text)
{
    const char seperator = '\n';
    char *line;

    line = strchr(text, seperator);

    int sum = 0;
    int max = 0;

    while (line != NULL)
    {
        *line = '\0'; /* overwrite first separator, creating two strings. */
        int amount = atoi(text);
        text = line + 1; // skip \0
        line = strchr(text, seperator);
        if (amount == 0)
        {
            // new elf
            if (sum > max)
            {
                max = sum;
            };
            sum = 0;
        }
        else
        {
            // current elf
            sum += amount;
        };
    }

    return max;
}

int lowest_calories_index(int *maximums)
{
    int lowest_amount = INT_MAX;
    int lowest_index = AMOUNT_OF_MAXIMUM;
    for (int i = 0; i < AMOUNT_OF_MAXIMUM; i++)
    {
        if (maximums[i] < lowest_amount)
        {
            lowest_index = i;
            lowest_amount = maximums[i];
        }
    }

    return lowest_index;
}

int *update_maximums(int amount, int *maximums)
{
    int lowest_index = lowest_calories_index(maximums);

    maximums[lowest_index] = amount;
}

long day01b(const char *text)
{
    const char seperator = '\n';
    char *line;

    line = strchr(text, seperator);

    int sum = 0;
    int maximums[3] = {0, 0, 0};

    while (line != NULL)
    {
        *line = '\0'; // overwrite first separator, creating two strings.
        int amount = atoi(text);
        text = line + 1; // skip \0
        line = strchr(text, seperator);
        if (amount == 0)
        {
            // new elf
            int lowest_index = lowest_calories_index(maximums);
            if (sum > maximums[lowest_index])
            {
                update_maximums(sum, &maximums);
            };
            sum = 0;
        }
        else
        {
            // current elf
            sum += amount;
        };
    }

    // check last line
    if (sum != 0)
    {
        int lowest_index = lowest_calories_index(maximums);
        if (sum > maximums[lowest_index])
        {
            update_maximums(sum, &maximums);
        };
    }

    sum = 0;
    for (int i = 0; i < AMOUNT_OF_MAXIMUM; i++)
    {
        sum += maximums[i];
    };

    return sum;
}
