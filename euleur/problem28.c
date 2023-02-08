/*
Starting with the number 1 and moving to the right in a clockwise direction
a 5 by 5 spiral is formed as follows:

21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed
in the same way?
*/


#include <stdio.h>
int * cycle(int start, int k);

void main()
{
    int *current;
    int i,step, total = 1;
    int start = 1;
    int lambda = 2;

    for (step = 0; step < 500; step++)
    {
        current = cycle(start,lambda);
        for (i = 0; i < 4; i++)
        {
            total = total + current[i];
        }
        start = current[3];
        lambda = lambda + 2;
    }

    printf("%i",total);
}

int * cycle(int start, int k)
{
    static int row[4];
    row[0] = start + k;
    row[1] = row[0] + k;
    row[2] = row[1] + k;
    row[3] = row[2] + k;
    return row;
}
