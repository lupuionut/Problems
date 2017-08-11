/*

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

*/

package me.ionut.rezolvari

object Problema9 {

    def rezolvare() = {
      val r = for { b <- 1 to 1000; a <- 1 to b; c = 1000 - a -b; if (a<b && b<c && a + b + c == 1000 && (a*a)+(b*b) == (c*c))} yield (a*b*c)
      r.head
      //triplets()
    }


    /*
     * A doua metoda de rezolvare folosind https://en.wikipedia.org/wiki/Formulas_for_generating_Pythagorean_triples#Dickson.27s_method
     *
     * r^2 = 2st
     * (r^2)/2 = st -> s si t sunt divizori ai lui (r^2)/2
     * pt fiecare numar se genereaza (r^2)/2
     * se gasesc divizorii lui (r^2)/2 => s si t
     * se creeaza perechile (r+s, r+t, r+s+t) = (x,y,z)
     * se verifica care corespunde cu regula (r+s)^2 + (r+t)^2 = (r+s+t)^2
     * si x + y + z == 1000
     */
    def triplets() = {
      val t = (2 to 500).map(r => (r, r*r/2))
                .map(n => divisors(n._2,2).map(x => (n._1 + (n._2/x), n._1 + x, n._1 + (n._2/x) + x)))
                .flatten
                .filter(p => p._1*p._1 + p._2*p._2 == p._3*p._3 && p._1 + p._2 + p._3 == 1000)
                .head
      t._1 * t._2 * t._3
    }

    def divisors(n: Int, d:Int): List[Int] = {
      if (n <= d)
        List(1,n)
      else {
        if (n%d == 0)
          List(d) ++ divisors(n, d+1)
        else
          divisors(n, d+1)
      }
    }
}


