/*
We shall say that an n-digit number is pandigital if it makes use of all the
digits 1 to n exactly once; for example, the 5-digit number, 15234,
is 1 through 5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254,
containing multiplicand, multiplier, and product is 1 through 9 pandigital.
Find the sum of all products whose multiplicand/multiplier/product identity
can be written as a 1 through 9 pandigital.

HINT: Some products can be obtained in more than one way so be sure
to only include it once in your sum.
*/
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <math.h>

bool unusual_pandigital(int, int, int);
int *member_digits(int n);
int length (int n);
bool array_contains(int *array, int n);
int array_sum(int *array);

#define LIMIT 2000
#define LENGTH 100

int
main()
{
    int *products = malloc(LENGTH*sizeof(int));
    int start = 2;
    int k = 0;

    for (int i = start; i < LIMIT; i++)
    {
        for (int j = start; j < LIMIT; j ++)
        {
            int product = i*j;
            if (unusual_pandigital(i, j, product)
                    && !array_contains(products,product))
            {
                products[k] = product;
                k++;
            }
        }
    }

    printf("sum of all products is %d\n", array_sum(products));
    return 1;
}

int
array_sum(int *array)
{
    int sum = 0;
    for (int i = 0; i <= LENGTH; i++)
    {
        sum += array[i];
    }

    return sum;
}

bool
array_contains(int *array, int n)
{
    for (int i = 0; i <= LENGTH; i++)
    {
        if (array[i] == n)
        {
            return true;
        }
    }
    return false;
}

bool
unusual_pandigital(int m, int n, int p)
{
    if (length(p) == 9)
    {
        return false;
    }

    if (m%10 == 0 || n%10 == 0 || p%10 == 0)
    {
        return false;
    }

    int xs[9] = { 0 };

    int* d_m = member_digits(m);
    int* d_n = member_digits(n);
    int* d_p = member_digits(p);

    for (int i = 0; i < length(m); i++)
    {
        if (d_m[i] == 0)
            return false;

        if (xs[d_m[i]-1] == d_m[i])
        {
            return false;
        }
        else
        {
            xs[d_m[i]-1] = d_m[i];
        }
    }

    for (int i = 0; i < length(n); i++)
    {
        if (d_n[i] == 0)
            return false;

        if (xs[d_n[i]-1] == d_n[i])
        {
            return false;
        }
        else
        {
            xs[d_n[i]-1] = d_n[i];
        }
    }

    for (int i = 0; i < length(p); i++)
    {
        if (d_p[i] == 0)
            return false;

        if (xs[d_p[i]-1] == d_p[i])
        {
            return false;
        }
        else
        {
            xs[d_p[i]-1] = d_p[i];
        }
    }

    for (int i = 0; i < 9; i++)
    {
        if (xs[i] == 0)
            return false;
    }

    return true;
}

int*
member_digits(int n)
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

int
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

    if (n >= 100 && n < 1000)
    {
        return 3;
    }

    if (n >= 1000 && n < 10000)
    {
        return 4;
    }

    if (n >= 10000 && n <100000)
    {
        return 5;
    }

    if (n >= 1000000 && n < 10000000)
    {
        return 6;
    }

    if (n >= 10000000 && n < 100000000)
    {
        return 7;
    }

    if (n > 987654321)
    {
        return 9;
    }

    return 9;
}
