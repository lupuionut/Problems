package me.ionut.rezolvari

object Problema1 {

	def rezolvare() : Int = {
		val numbers = 1 to 999
		numbers.filter(x => divides_by(x)(3) || divides_by(x)(5)).reduce(_ + _)
	}

	def divides_by(x: Int)(d: Int) : Boolean = {
		if (x % d == 0)
			true
		else
			false
	}
}
