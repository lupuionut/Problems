/*
145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the factorial of
their digits.

Note: as 1! = 1 and 2! = 2 are not sums they are not included.
*/

#include <stdio.h>
#include <stdlib.h>

int* int_to_array(int);
static int length(int);
static int factorial(int);

int
main()
{
    int total = 0;

    // upper bound is 7 digits x 9!  = 2540160
    // every larger 7 digit number has a sum of factorials less than itself
    for (int i = 3; i < 2540160; i++)
    {
        int *digits = int_to_array(i);
        int sum = 0;

        for (int j = 0; j < length(i); j++)
        {
            sum += factorial(digits[j]);
        }

        if (sum == i)
        {
            total += sum;
        }
        free(digits);
    }

    printf("sum is %d", total);

    return 1;
}

int*
int_to_array(int n)
{
    int size = length(n);
    int *digits = malloc(size * sizeof(int));
    int i = size - 1;

    while (n != 0)
    {
        digits[i] = n%10;
        n = n/10;
        i--;
    }

    return digits;
}

static int
length (int n)
{
    if (n < 10)
    {
        return 1;
    }

    if (n >= 10 && n < 100)
    {
        return 2;
    }

    if (n >= 100)
    {
        return 2 + length(n/100);
    }
}


static int
factorial(int n)
{
    int factorial = 1;

    for (int i = 2; i <= n; i++)
    {
        factorial *= i;
    }

    return factorial;
}
