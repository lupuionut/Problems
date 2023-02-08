/*
    2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
    What is the sum of the digits of the number 2^1000?
 */

package me.ionut.rezolvari
import scala.math.BigInt


object Problema16 {

  def rezolvare() = {
    val a = BigInt(1)
    (a << 1000).toString.map(_.asDigit).sum
  }

}
