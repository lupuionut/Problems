/*
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
*/

package me.ionut.rezolvari
import scala.collection.SeqView

object Problema23 {

  def rezolvare() = {
    val xs = (1 to 28123).filter(is_abundant(_)).toVector
    val abundant_sums = xs.foldLeft(Map(0 -> true)) { (acc, x) => acc ++ generate_sums(x,xs,28123) }
    (1 to 28123).filter(!abundant_sums.contains(_)).sum
  }


  def is_abundant(n: Int): Boolean = {
      if (divisors(n).sum > n)
        true
      else
        false
  }

  def divisors(n: Int): Vector[Int] = {
    def divisors_int(n: Int, d: Int, acc: Vector[Int]): Vector[Int] = {
      if (n <= d)
        acc
      else {
        if (n%d == 0)
          divisors_int(n, d+1, acc ++ Vector(d))
        else
          divisors_int(n, d+1, acc)
      }
    }
    divisors_int(n, 2, Vector(1))
  }


  /*
   * generate a map with all possible sums until 28123 between 1 abundent number and all the abundant numbers
   */
  def generate_sums(x:Int, xs: Vector[Int], max: Int): Map[Int, Boolean] = {
    def i_generate_sums(x: Int, xs: Vector[Int], acc: Map[Int, Boolean], max: Int): Map[Int, Boolean] = {
      if (xs.length == 0) {
        acc
      } else {
        if ((x + xs.head) <= max) {
          i_generate_sums(x, xs.tail, acc ++ Map(x+xs.head -> true), max)
        } else {
          acc
        }
      }
    }
    i_generate_sums(x,xs,Map(0 -> true), max)
  }


}
