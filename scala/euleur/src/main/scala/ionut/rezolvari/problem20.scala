/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

*/

package me.ionut.rezolvari

object Problema20 {

  def rezolvare() = {
    factorial(100).toString.map(_.asDigit).sum
  }

  def factorial(n: Int): BigInt = {
    def inner_factorial(acc: BigInt, n: Int): BigInt = {
      if (n == 1)
        acc
      else
        inner_factorial(acc * n, n-1)
    }
    inner_factorial(1, n)
  }
}
