package me.ionut.rezolvari

object Problema2 {

	def rezolvare() : Int = {
		val (x, y, z) = gen_fibonacci(0, 1, 0)
		z
	}

	def gen_fibonacci(prev: Int, current: Int, acc: Int) : (Int, Int, Int) = {
		if (current > 4000000) {
			(prev, current, acc)
		}
		else {
			val y = { if (is_even(current)) acc + current else acc }
			gen_fibonacci(current, current + prev, y)
		}
	}

	def is_even(n : Int) : Boolean = (n | 1) != n 
}
