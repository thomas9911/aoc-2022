
#include <stdlib.h>
#define GRID_SIZE 128
typedef int GRID[GRID_SIZE][GRID_SIZE];

void print_grid(GRID grid, int size)
{
    for (int i = 0; i < size; i++)
    {
        for (int j = 0; j < size; j++)
        {
            printf("%i", grid[j][i]);
        }
        printf("\n");
    }
}

int load_grid(GRID grid, const char *text)
{
    const char seperator[2] = "\n";
    char *line;
    int field;
    int line_index = 0;
    int actual_size = GRID_SIZE;

    line = strtok(text, seperator);

    while (line != NULL)
    {
        actual_size = strlen(line);
        for (int i = 0; i < actual_size; i++)
        {
            field = (*(line + i)) - 48;
            grid[i][line_index] = field;
        }

        line = strtok(NULL, seperator);
        line_index++;
    }

    return actual_size;
}

int check_if_tree_visible(GRID grid, int pos_x, int pos_y, int size)
{
    int my_height = grid[pos_x][pos_y];
    int current_height;
    int visible = 0;
    int maximum_height = 0;

    for (int i = 0; i < pos_x; i++)
    {
        current_height = grid[i][pos_y];
        if (current_height > maximum_height)
        {
            maximum_height = current_height;
        }
    }
    if (maximum_height < my_height)
    {
        visible += 1;
    }
    maximum_height = 0;

    for (int i = 0; i < pos_y; i++)
    {
        current_height = grid[pos_x][i];
        if (current_height > maximum_height)
        {
            maximum_height = current_height;
        }
    }
    if (maximum_height < my_height)
    {
        visible += 1;
    }
    maximum_height = 0;

    for (int i = pos_x + 1; i < size; i++)
    {
        current_height = grid[i][pos_y];
        if (current_height > maximum_height)
        {
            maximum_height = current_height;
        }
    }
    if (maximum_height < my_height)
    {
        visible += 1;
    }
    maximum_height = 0;

    for (int i = pos_y + 1; i < size; i++)
    {
        current_height = grid[pos_x][i];
        if (current_height > maximum_height)
        {
            maximum_height = current_height;
        }
    }
    if (maximum_height < my_height)
    {
        visible += 1;
    }
    maximum_height = my_height;

    return visible;
}

int calculate_scenic_score(GRID grid, int pos_x, int pos_y, int size)
{
    int my_height = grid[pos_x][pos_y];
    int current_height;
    int visible = 0;
    int up = 0, down = 0, left = 0, right = 0;

    for (int i = pos_x - 1; i >= 0; i--)
    {
        current_height = grid[i][pos_y];

        if (current_height < my_height)
        {
            left += 1;
        }
        else if (current_height >= my_height)
        {
            left += 1;
            break;
        }
    }

    for (int i = pos_x + 1; i < size; i++)
    {
        current_height = grid[i][pos_y];

        if (current_height < my_height)
        {
            right += 1;
        }
        else if (current_height >= my_height)
        {
            right += 1;
            break;
        }
    }

    for (int i = pos_y - 1; i >= 0; i--)
    {
        current_height = grid[pos_x][i];

        if (current_height < my_height)
        {
            up += 1;
        }
        else if (current_height >= my_height)
        {
            up += 1;
            break;
        }
    }

    for (int i = pos_y + 1; i < size; i++)
    {
        current_height = grid[pos_x][i];

        if (current_height < my_height)
        {
            down += 1;
        }
        else if (current_height >= my_height)
        {
            down += 1;
            break;
        }
    }

    return left * right * down * up;
}

long day08a(const char *text)
{
    int grid[GRID_SIZE][GRID_SIZE] = {0};

    int actual_size = load_grid(grid, text);
    int amount_of_visible_trees = 0;

    // print_grid(grid, actual_size);

    // check visible
    for (int i = 1; i < actual_size - 1; i++)
    {
        for (int j = 1; j < actual_size - 1; j++)
        {
            int visible = check_if_tree_visible(grid, i, j, actual_size);
            if (visible != 0)
            {
                amount_of_visible_trees += 1;
            }
        }
    }

    // check outer ring
    amount_of_visible_trees += (2 * actual_size) + 2 * (actual_size - 2);

    return amount_of_visible_trees;
}

long day08b(const char *text)
{
    int grid[GRID_SIZE][GRID_SIZE] = {0};

    int actual_size = load_grid(grid, text);
    int score;
    int max_score = 0;

    for (int i = 1; i < actual_size - 1; i++)
    {
        for (int j = 1; j < actual_size - 1; j++)
        {
            score = calculate_scenic_score(grid, i, j, actual_size);
            if (score > max_score)
            {
                max_score = score;
            }
        }
    }

    return max_score;
}
