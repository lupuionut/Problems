/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.
*/

package me.ionut.rezolvari

object Problema10 {

	def rezolvare() : Long = {
		(1 to 2000000).filter(is_prime(_)).map(_.toLong).sum
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
