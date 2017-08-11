/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
cel mai mare multiplu comun = lcm

lcm = (a x b) / gcd(a,b)
 
*/
package me.ionut.rezolvari

object Problema5 {

	def rezolvare() : Int = {
		(1 to 19).foldLeft(1){ (x, acc) => lcm(x, acc) }
	}

	def lcm(a: Int, b: Int) : Int = {
		(a * b) / gcd(a,b) 
	}

	def gcd(a:Int, b:Int) : Int = {
		if (b == 0) {
			a
		} else {
			gcd(b, a%b)
		}
	}
}
