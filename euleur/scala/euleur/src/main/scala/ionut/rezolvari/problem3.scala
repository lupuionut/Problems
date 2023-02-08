package me.ionut.rezolvari

object Problema3 {

	def rezolvare() : Int = {
		factors(600851475143L).last.toInt
	}


	def factors(n: Long): List[Long] = (2 to math.sqrt(n).toInt).find(n % _ == 0).fold(List(n))(i => i.toLong :: factors(n / i))

}
