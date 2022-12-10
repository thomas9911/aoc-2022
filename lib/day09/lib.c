#include <stdlib.h>
#include <stdint.h>
#include "hashmap.h"

#define UP 'U'
#define RIGHT 'R'
#define DOWN 'D'
#define LEFT 'L'

struct Location
{
    char *position;
    int x;
    int y;
    int count;
};

int location_compare(const void *a, const void *b, void *udata)
{
    const struct Location *ua = a;
    const struct Location *ub = b;
    return strcmp(ua->position, ub->position);
}

uint64_t location_hash(const void *item, uint64_t seed0, uint64_t seed1)
{
    const struct Location *location = item;
    return hashmap_sip(location->position, strlen(location->position), seed0, seed1);
}

void direction_to_coordinates(char ch, int coordinates[2])
{
    switch (ch)
    {
    case UP:
        coordinates[0] = 0;
        coordinates[1] = -1;
        break;
    case DOWN:
        coordinates[0] = 0;
        coordinates[1] = 1;
        break;

    case LEFT:
        coordinates[0] = -1;
        coordinates[1] = 0;
        break;

    case RIGHT:
        coordinates[0] = 1;
        coordinates[1] = 0;
        break;

    default:
        coordinates[0] = 0;
        coordinates[1] = 0;
        break;
    }
}

int distance_between(int tail_position[2], int head_position[2])
{
    int x_diff = abs(head_position[0] - tail_position[0]);
    int y_diff = abs(head_position[1] - tail_position[1]);
    return max(x_diff, y_diff);
}

int clamp_to_one(int x)
{
    if (x == 0)
    {
        return 0;
    }
    if (x >= 1)
    {
        return 1;
    }
    if (x < 0)
    {
        return -1;
    }
}

void pull_tail_forward(int tail_position[2], int head_position[2], int direction_coordinations[2])
{
    if (distance_between(tail_position, head_position) > 1)
    {
        int x_diff = clamp_to_one(head_position[0] - tail_position[0]);
        int y_diff = clamp_to_one(head_position[1] - tail_position[1]);

        tail_position[0] += x_diff;
        tail_position[1] += y_diff;
    }
}

void print_trail(struct hashmap *visited_path)
{
    int mid_point = 16;
    int grid[32][32] = {0};
    size_t iter = 0;
    void *item;

    while (hashmap_iter(visited_path, &iter, &item))
    {
        const struct Location *location = item;
        // printf("= (%i, %i)\n", location->x, location->y);
        grid[(location->x) + mid_point][(location->y) + mid_point] = 1;
    }

    for (size_t i = 0; i < 32; i++)
    {
        for (size_t j = 0; j < 32; j++)
        {
            if (grid[j][i] == 0)
            {
                printf(".");
            }
            else
            {
                printf("#");
            }
        }
        printf("\n");
    }
}

long day09a(const char *text)
{
    const char seperator[2] = "\n";
    char *line;
    int amount;
    char direction;
    struct hashmap *visited_path = hashmap_new(sizeof(struct Location), 0, 0, 0, location_hash, location_compare, NULL, NULL);
    int head_position[2] = {0};
    int tail_position[2] = {0};

    int direction_coordinations[2] = {0};

    line = strtok(text, seperator);

    while (line != NULL)
    {
        direction = *line;
        amount = atoi(line + 2);
        direction_to_coordinates(direction, direction_coordinations);
        for (int i = 0; i < amount; i++)
        {
            head_position[0] += direction_coordinations[0];
            head_position[1] += direction_coordinations[1];

            pull_tail_forward(tail_position, head_position, direction_coordinations);
            char *string_in_heap = (char *)malloc((15 + 1) * sizeof(char));
            sprintf(string_in_heap, "%i,%i", tail_position[0], tail_position[1]);
            hashmap_set(visited_path, &(struct Location){.position = string_in_heap, .x = tail_position[0], .y = tail_position[1]});
        }

        line = strtok(NULL, seperator);
    }

    size_t iter = 0;
    void *item;
    int visited_fields = 0;

    while (hashmap_iter(visited_path, &iter, &item))
    {
        const struct Location *location = item;
        // printf(" => (%i, %i)\n", location->x, location->y);
        visited_fields++;
    }

    hashmap_free(visited_path);

    return visited_fields;
}

long day09b(const char *text)
{
    const char seperator[2] = "\n";
    char *line;
    int amount;
    char direction;
    struct hashmap *visited_path = hashmap_new(sizeof(struct Location), 0, 0, 0, location_hash, location_compare, NULL, NULL);
    int head_position[10][2] = {0};
    int tail_position[10][2] = {0};

    int direction_coordinations[2] = {0};

    line = strtok(text, seperator);

    while (line != NULL)
    {
        direction = *line;
        amount = atoi(line + 2);
        direction_to_coordinates(direction, direction_coordinations);
        for (int i = 0; i < amount; i++)
        {
            head_position[0][0] += direction_coordinations[0];
            head_position[0][1] += direction_coordinations[1];

            for (int j = 0; j < 10; j++)
            {
                // printf("=> (%i, %i)\n", head_position[j][0], head_position[j][1]);
                pull_tail_forward(tail_position[j], head_position[j], direction_coordinations);
                // printf("  => (%i, %i)\n", tail_position[j][0], tail_position[j][1]);
                if (j != 9)
                {
                    head_position[j + 1][0] = tail_position[j][0];
                    head_position[j + 1][1] = tail_position[j][1];
                }
            }

            // printf("\n");

            char *string_in_heap = (char *)malloc((15 + 1) * sizeof(char));
            sprintf(string_in_heap, "%i,%i", tail_position[9][0], tail_position[9][1]);
            hashmap_set(visited_path, &(struct Location){.position = string_in_heap, .x = tail_position[9][0], .y = tail_position[9][1]});
            // printf("=> (%i, %i)\n", tail_position[9][0], tail_position[9][1]);
        }

        line = strtok(NULL, seperator);
    }

    size_t iter = 0;
    void *item;
    int visited_fields = 0;

    while (hashmap_iter(visited_path, &iter, &item))
    {
        const struct Location *location = item;
        // printf("= (%i, %i)\n", location->x, location->y);
        visited_fields++;
    }

    // print_trail(visited_path);

    hashmap_free(visited_path);

    return visited_fields;
}
