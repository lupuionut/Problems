/*
  Euler discovered the remarkable quadratic formula: n2+n+41

  It turns out that the formula will produce 40 primes for the consecutive integer values 0≤n≤39
  . However, when n=40,402+40+41=40(40+1)+41 is divisible by 41, and certainly when n=41,412+41+41

  is clearly divisible by 41.

  The incredible formula n2−79n+1601
  was discovered, which produces 80 primes for the consecutive values 0≤n≤79

  . The product of the coefficients, −79 and 1601, is −126479.

  Considering quadratics of the form:

      n2+an+b

  , where |a|<1000 and |b|≤1000

  where |n|
  is the modulus/absolute value of n
  e.g. |11|=11 and |−4|=4

  Find the product of the coefficients, a
  and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n=0.
*/

package me.ionut.rezolvari

object Problema27 {

  def rezolvare() = {
    all_polynoms(1000).foldLeft((0,0)){(acc,x) => if (number_primes(x) > acc._2) (x._1 * x._2, number_primes(x)) else acc}._1
  }

  /*
   * get all tuples (b,c) for polynomials with discriminant -163 
   * all polynomials with discriminant (ax^2+bx+c, b^2-4ac == 0) equal with -163
   * http://people.ucalgary.ca/~ramollin/PPQ.pdf
   */
  def all_polynoms(limit: Int): List[(Int,Int)] = {
    {for (b <- (-limit to limit); c <- (-limit to limit); if ((b*b) - 4*c) == -163) yield (b,c)}.toList
  }


  def number_primes(x: (Int,Int)): Int = {
    Stream.from(0).takeWhile(n => is_prime(n*n + x._1 * n + x._2)).length
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
