#include <stdlib.h>

#define NOOP "noop"
#define ADDX "addx"

#define SCREEN_X 40
#define SCREEN_Y 6

void tick_cycle_part_a(int *state_x, int *cycles, int *signal_strength_recorder)
{
    (*cycles)++;
    if (*cycles % 40 == 20)
    {
        *signal_strength_recorder += *cycles * *state_x;
    }
}

void tick_cycle_part_b(int *state_x, int *cycles, unsigned char *output)
{
    int current_cycle = (*cycles) % SCREEN_X;
    if (current_cycle == 0 && *cycles != 0)
    {
        strcat(output, "\n");
    }
    if ((current_cycle - 1 == *state_x) || (current_cycle == *state_x) || (current_cycle + 1 == *state_x))
    {
        strcat(output, "#");
    }
    else
    {
        strcat(output, ".");
    }

    (*cycles)++;
}

long day10a(const char *text)
{
    const char seperator[2] = "\n";
    char *line;

    int arg1 = 0;

    int state_x = 1;
    int cycles = 0;
    int signal_strength;
    int signal_strength_sum = 0;

    line = strtok(text, seperator);

    while (line != NULL)
    {
        if (strcmp(line, NOOP) == 0)
        {
            tick_cycle_part_a(&state_x, &cycles, &signal_strength_sum);
        }
        else if (strncmp(line, ADDX, strlen(ADDX)) == 0)
        {
            tick_cycle_part_a(&state_x, &cycles, &signal_strength_sum);
            tick_cycle_part_a(&state_x, &cycles, &signal_strength_sum);
            arg1 = atoi(line + strlen(ADDX));
            state_x += arg1;
        };

        line = strtok(NULL, seperator);
    }

    return signal_strength_sum;
}

long day10b(const char *text, unsigned char *output)
{
    const char seperator[2] = "\n";
    char *line;

    int arg1 = 0;

    int state_x = 1;
    int cycles = 0;

    line = strtok(text, seperator);

    while (line != NULL)
    {
        if (strcmp(line, NOOP) == 0)
        {
            tick_cycle_part_b(&state_x, &cycles, output);
        }
        else if (strncmp(line, ADDX, strlen(ADDX)) == 0)
        {
            tick_cycle_part_b(&state_x, &cycles, output);
            tick_cycle_part_b(&state_x, &cycles, output);
            arg1 = atoi(line + strlen(ADDX));
            state_x += arg1;
        };

        line = strtok(NULL, seperator);
    }

    return 1;
}
