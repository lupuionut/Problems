/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

*/
package me.ionut.rezolvari

object Problema7 {

	def rezolvare() : Int = {
		Stream.from(1).filter(is_prime(_)).take(10001).last
	}


	def is_prime(n:Int) : Boolean = {
		if (n == 2) return true 

		if ((n <= 1) || ((n | 1) != n)) {
			return false
		} 

		val sq = math.pow(n, 0.5).ceil.toInt
		val divs = for { i <- 2 to sq; r = (n%i == 0)} yield r
			
		if (divs.contains(true)) {
			false
		} else {
			true
		}
	}

}
