#include <day0/lib.c>

/* function returning the max between two numbers */
int32_t max(int32_t num1, int32_t num2)
{

    /* local variable declaration */
    int32_t result;

    if (num1 > num2)
        result = num1;
    else
        result = num2;

    return result;
}

int32_t bar_function(int32_t x)
{
    return x;
}
