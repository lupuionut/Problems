/*
The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

Find the sum of all numbers, less than one million, which are palindromic in
base 10 and base 2.

(Please note that the palindromic number, in either base, may not include
leading zeros.)
*/

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

int *int_to_array(int, int);
char* int_to_binary(int, int);
int length(int);

int
main()
{
    int sum = 0;

    for (int n = 1; n < 1000000; n++)
    {
        // verify decimal
        int max = (int) log2(n) + 1;
        int len = length(n);
        int *array = int_to_array(n, len);

        int start = 0;
        int end = len - 1;
        bool palindrome = true;

        while (len > 1 && start != end && start < end)
        {
            if (array[start] != array[end])
            {
                palindrome = false;
                break;
            }

            start += 1;
            end -= 1;
        }
        free(array);

        // verify binary
        start = 0;
        end = max - 1;
        int stop = (start + end) / 2;
        char* bin = int_to_binary(n, max);

        while (start <= stop)
        {
            if (bin[start] != bin[end])
            {
                palindrome = false;
                break;
            }

            start += 1;
            end -= 1;
        }
        free(bin);

        if (palindrome == true)
        {
            sum += n;
        }
    }

    printf("sum of all palindromic numbers until 1 million is %d\n", sum);
    return 1;
}

char*
int_to_binary(int n, int size)
{
    int powers[20] = {1,2,4,8,16,32,64,128,256,512,1024,2048,4096,
                        8192,16384,32768,65536,131072,262144,524288};
    char* binary = malloc(size*sizeof(char));

    for (int i = 0; i < size; i++)
    {
        int position = size - i - 1;
        if ((n & powers[i]) == powers[i])
        {
            binary[position] = '1';
        }
        else
        {
            binary[position] = '0';
        }
    }

    return binary;
}

int*
int_to_array(int n, int size)
{
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

    if (n >= 100)
    {
        return 2 + length(n/100);
    }
}
