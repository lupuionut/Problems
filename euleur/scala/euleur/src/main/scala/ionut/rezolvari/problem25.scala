/*
The Fibonacci sequence is defined by the recurrence relation:

    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.

Hence the first 12 terms will be:

    F1 = 1
    F2 = 1
    F3 = 2
    F4 = 3
    F5 = 5
    F6 = 8
    F7 = 13
    F8 = 21
    F9 = 34
    F10 = 55
    F11 = 89
    F12 = 144

The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
*/

package me.ionut.rezolvari
import scala.math.BigInt

object Problema25 {

  def rezolvare() = {
    //varianta mea -> fibonacci().takeWhile(_.toString.length < 1000).length
    fibs.takeWhile(_.toString.length < 1000).length
  }

  def fibonacci(): List[BigInt] = {
    def fibonacci_i(n: BigInt, max:BigInt, acc: List[BigInt]): List[BigInt] = {
      if (n == 1 || n == 2) {
        fibonacci_i(n+1,max,acc++List(BigInt(1)))
      } else if (n >= max) {
        acc
      } else {
        val x = acc.last + acc.init.last
        fibonacci_i(n+1,max,acc ++ List(x))
      }
    }
    fibonacci_i(1, 5000, List())
  }

  lazy val fibs: Stream[BigInt] = BigInt(0) #:: BigInt(1) #:: fibs.zip(fibs.tail).map { n => n._1 + n._2 }
}
