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

    printf("\%i",total);
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
