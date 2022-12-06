#include <stdlib.h>

#define START_OF_PACKET_SIZE 4
#define START_OF_MESSAGE_SIZE 14

void print_last_four(char *array)
{
    printf("%c, %c, %c, %c\n", array[0], array[1], array[2], array[3]);
}

void add_and_rotate_left(char *array, char item, size_t size)
{
    for (int i = 1; i < size; i++)
    {
        array[i - 1] = array[i];
    }
    array[size - 1] = item;
}

int last_characters_unique(char *array, size_t size)
{
    char found_chars[26] = {0};
    char ch;

    for (int i = 0; i < size; i++)
    {
        ch = array[i];
        if (ch == 0)
        {
            // array is partially empty
            return 0;
        }
        found_chars[ch - 97]++;
    }

    for (int i = 0; i < 26; i++)
    {
        if (found_chars[i] > 1)
        {
            // found duplicates
            return 0;
        }
    }

    return 1;
}

long run(const char *text, size_t size)
{
    char buffer[64] = {0};
    char ch;

    for (int i = 0; i < strlen(text); i++)
    {
        ch = *(text + i);
        if (ch == '\n')
        {
            // not found
            return -1;
        }

        add_and_rotate_left(buffer, ch, size);

        // print_last_four(buffer);
        if (last_characters_unique(buffer, size) == 1)
        {
            return i + 1;
        }
    }

    return -1;
}

long day06a(const char *text)
{
    return run(text, START_OF_PACKET_SIZE);
}

long day06b(const char *text)
{
    return run(text, START_OF_MESSAGE_SIZE);
}
