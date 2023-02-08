/*
Surprisingly there are only three numbers that can be written as the sum
of fourth powers of their digits:
    1634 = 14 + 64 + 34 + 44
    8208 = 84 + 24 + 04 + 84
    9474 = 94 + 44 + 74 + 44
As 1 = 14 is not a sum it is not included.
The sum of these numbers is 1634 + 8208 + 9474 = 19316.
Find the sum of all the numbers that can be written as the sum of fifth powers
of their digits
*/

#include <stdio.h>
#include <math.h>

int sum_power(int, int);

void main()
{
    int i, sum = 0;
    int max = 6 * (int) pow(9,5);

    for (i = 2; i<max; i++)
    {
        if (sum_power(i, 5) == i)
        {
            sum += i;
        }
    }

    printf("sum is %d \n", sum);
}

int sum_power(int a, int n)
{
    int digit, sigma = 0;

    while (a != 0)
    {
        digit = a%10;
        sigma += (int) pow(digit,n);
        a = a/10;
    }
    return sigma;
}
