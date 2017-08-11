/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
package me.ionut.rezolvari

object Problema4 {

	def rezolvare() : Int = {
		val choices = for { i <- 999 to 1 by -1; z <- 999 to 1 by -1; if is_palindrome(i*z) } yield (i*z)
		choices.max
	}

	def is_palindrome(n : Int) = {
		n.toString.reverse == n.toString
	} 
}
