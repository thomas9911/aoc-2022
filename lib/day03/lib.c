// did work before now crashes

// #include <string.h>
// #include "clist.h"
// #define GROUP_SIZE 3

// int char_to_score(char c)
// {
//     if (c > 96)
//     {
//         return c - 96;
//     }
//     else
//     {
//         return (c - 64) + 26;
//     }
// }

// char determine_overlap(char *left, char *right)
// {
//     for (int i = 0; i < strlen(left); i++)
//     {
//         if (strchr(right, *(left + i)) != NULL)
//         {
//             // found
//             return *(left + i);
//         }
//     }

//     return '?';
// }

// CList* find_overlapping(char *left, char *right)
// {
//     size_t n = sizeof(char);
//     CList *list = CList_init(n);
//     char found_char;
//     char* next;

//     for (int i = 0; i < strlen(left); i++)
//     {
//         next = right;
//         for (int j = 0; j < strlen(right); j++) {
//             next = strchr(next, *(left + i));
//             if (next == NULL) {
//                 break;
//             } else {
//                 found_char = *(left + i);
//                 list->add(list, &found_char);
//                 next++;
//             }
//         }


//         // while ((right = strchr(right, *(left + i))) != NULL)
//         // {
//         //     // found_char = *(left + i);
//         //     // list->add(list, &found_char);
//         //     // ++right; // Increment result, otherwise we'll find target at the same location
//         // }

//         // for (int j = 0; j < 1; j++) {
//         //     right = strchr(right, *(left + i));
//         //     if (right == NULL) {
//         //         printf("null pointer");
//         //     }
//         //     // right++;
//         // }
//     }

//     return list;
// }

// long day03a(const char *text)
// {
//     const char seperator[2] = "\n";
//     char *line;
//     int split_of_point;
//     int sum = 0;

//     line = strtok(text, seperator);

//     while (line != NULL)
//     {
//         split_of_point = strlen(line) / 2;
//         char *line_first = malloc(split_of_point * sizeof(char));

//         strcpy(line_first, line);
//         line_first[split_of_point] = '\0';

//         char *line_second = line + split_of_point;
//         // printf("%s\n", line_first);
//         // printf("%s\n", line_second);

//         char x = determine_overlap(line_first, line_second);
//         // printf("%c -> %i\n\n", x, char_to_score(x));
//         sum += char_to_score(x);

//         free(line_first);
//         line = strtok(NULL, seperator);
//     }

//     return sum;
// }

// long day03b(const char *text)
// {
//     const char seperator[2] = "\n";
//     char *line;
//     char *previous_line;
//     char item[2];
//     CList *overlap;
//     int split_of_point;
//     int sum = 0;
//     int line_number = 0;

//     line = strtok(text, seperator);

//     while (line != NULL)
//     {
//         if (line_number % GROUP_SIZE) {
//             previous_line = line;
//         } else {
//             // // printf("%s => %s\n", previous_line, line);
//             // overlap = find_overlapping(previous_line, line);
//             // // printf("%c\n", overlap)
//             // // overlap->print(overlap, 0, 10000, "char");
//             // *previous_line = '\0';
//             // for (int i = 0; i < overlap->count(overlap); i++) {
//             //     item[0] = overlap->at(overlap, i);
//             //     item[1] = '\0';
//             //     // printf("%s\n", item);

//             //     strcat(previous_line, item);
//             // }
//         }

//         printf("%s => %s\n", previous_line, line);
//         // sum += char_to_score(x);

//         line = strtok(NULL, seperator);
//         line_number++;
//     }

//     return sum;
// }
