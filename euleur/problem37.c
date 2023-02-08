/*
The number 3797 has an interesting property. Being prime itself,
it is possible to continuously remove digits from left to right,
and remain prime at each stage: 3797, 797, 97, and 7.
Similarly we can work from right to left: 3797, 379, 37, and 3.

Find the sum of the only eleven primes that are both truncatable from
left to right and right to left.

NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
*/

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>

int length(int n);
int* possible_numbers(int n, int length);
bool is_prime(int n);

int
main(int argc, char *argv[])
{
    int counter = 0;
    int sum = 0;
    int n = 10;

    while (counter < 11)
    {
        int len = length(n);
        int* elem = possible_numbers(n, len);
        bool all_primes = true;

        for (int i = 0; i < 2*len - 1; i++)
        {
            if (!is_prime(elem[i]))
            {
                all_primes = false;
                break;
            }
        }

        if (all_primes)
        {
            counter += 1;
            sum += n;
        }
        n += 1;
        free(elem);
    }

    printf("sum is %d\n", sum);
    return 0;
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

    if (n >= 100)
    {
        return 2 + length(n/100);
    }
}


int*
possible_numbers(int n, int length)
{
    int possibles = 2*length - 1;
    int* xs = malloc(possibles*sizeof(int));
    int i = 0, divizor;
    int k = i;

    for (i = 0; i < length; i++)
    {
        int d = i%length;
        divizor = (int) pow(10,d);
        xs[k] = n/divizor;
        if (d != 0)
        {
            k += 1;
            xs[k] = n%divizor;
        }
        k += 1;
    }

    return xs;
}

bool
is_prime(int n)
{
    if (n == 1)
    {
        return false;
    }

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
