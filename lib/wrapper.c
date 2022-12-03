#include <day0/lib.c>
#include <day01/lib.c>
#include <day02/lib.c>
#include <day03/lib.c>

/* function returning the max between two numbers */
long max_impl(long num1, long num2)
{

    /* local variable declaration */
    long result;

    if (num1 > num2)
        result = num1;
    else
        result = num2;

    return result;
}

long bar_function(long x)
{
    return x;
}
