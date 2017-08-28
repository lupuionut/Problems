/*
The number, 197, is called a circular prime because all rotations of the
digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71,
73, 79, and 97.

How many circular primes are there below one million?
*/

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

int* int_to_array(int);
static int length(int);
int* array_rotate(int*, int);
int multiplier(int);
bool is_prime(int);

int
main()
{
    int counter = 0;

    for (int i = 2; i < 1000000; i++)
    {
        int* array = int_to_array(i);
        int len = length(i);
        int* rotated = array_rotate(array, len);
        int circular = 1;

        for (int j = 0; j < len; j++)
        {
            if (is_prime(rotated[j]))
            {
                circular = circular & 1;
            }
            else
            {
                circular = circular & 0;
            }
        }

        if (circular == 1)
        {
            counter += 1;
        }

        free(array);
        free(rotated);
    }

    printf("total circular primes: %d\n", counter);

    return 1;
}

bool
is_prime(int n)
{
    if (n == 2)
        return true;

    int limit = (int) sqrt(n);

    for (int i = 2; i <= limit; i++)
    {
        if (n%i == 0)
        {
            return false;
        }
    }

    return true;
}

int*
array_rotate(int* array, int length)
{
    int* rotations = malloc(length * sizeof(int));

    for (int i = 0; i < length; i++)
    {
        rotations[i] = 0;
        int mult = length - 1;
        for (int j = i; j < i+length; j++)
        {
            rotations[i] += (array[j%length] * multiplier(mult));
            mult -= 1;
        }
    }
    return rotations;
}

int
multiplier(int length)
{
    if (length == 0)
    {
        return 1;
    }

    if (length == 1)
    {
        return 10;
    }

    if (length == 2)
    {
        return 100;
    }

    if (length > 2)
    {
        return 100*multiplier(length - 2);
    }
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
