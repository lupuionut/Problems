/*
The fraction 49/98 is a curious fraction, as an inexperienced mathematician
in attempting to simplify it may incorrectly believe that 49/98 = 4/8,
which is correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction,
less than one in value, and containing two digits in the numerator and denominator.

If the product of these four fractions is given in its lowest common terms,
find the value of the denominator.
*/

#include <stdio.h>
#include <stdbool.h>

bool is_curious(int, int);
int gcd(int, int);

int
main()
{
    int product_numarator = 1;
    int product_numitor = 1;

    for (int i = 10; i < 100; i++)
    {
        for (int j = 10; j < 100; j++)
        {
            if (is_curious(i, j))
            {
                product_numarator *= i;
                product_numitor *= j;
            }
        }
    }

    printf("product is %d\n",
            product_numitor/gcd(product_numarator, product_numitor));

    return 1;
}

bool
is_curious(int numarator, int numitor)
{
    int curious_numarator = numarator/10;
    int right_numarator = numarator%10;
    int curious_numitor = numitor%10;
    int left_numitor = numitor/10;

    if (curious_numarator == 0 || curious_numitor == 0)
    {
        return false;
    }

    if (numarator%11 == 0 || numitor%11 == 0)
    {
        return false;
    }

    if (numarator == numitor)
    {
        return false;
    }

    if ((curious_numarator == curious_numitor || right_numarator == left_numitor)
        && (curious_numarator * numitor == curious_numitor * numarator))
    {
        return true;
    }

    return false;
}

int
gcd(int a, int b)
{
    if (b == 0)
    {
        return a;
    }
    else
    {
        return gcd(b, a%b);
    }
}
