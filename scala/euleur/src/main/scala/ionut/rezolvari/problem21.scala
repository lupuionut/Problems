/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/

package me.ionut.rezolvari

object Problema21 {

  def rezolvare() = {
    (1 to 10000).toList.filter(n => {val b= d(n).sum; d(b).sum == n && b != n}).sum
  }


  def d(n: Int): List[Int] = {
    if (n == 1)
      List(1)
    else
      (1 to n).toList.filter(x => n % x == 0 && n != x)
  }
}
