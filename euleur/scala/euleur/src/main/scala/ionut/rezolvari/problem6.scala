/*
The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 552 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

*/
package me.ionut.rezolvari

object Problema6 {

	def rezolvare() : Int = {
		val sum_of_squares = (1 to 100).map(x => x*x).sum
		val suma = (1 to 100).sum
		val square_of_sum = suma * suma
		square_of_sum - sum_of_squares
	}


}
