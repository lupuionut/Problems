/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
Which starting number, under one million, produces the longest chain?
NOTE: Once the chain starts the terms are allowed to go above one million.

*/

package me.ionut.rezolvari

object Problema14 {

  def rezolvare() = {
    def from(n: Long, c: Int = 0): Int = if (n == 1) c + 1 else from(if (n % 2 == 0) n / 2 else 3 * n + 1, c + 1)
    val r = Stream.from(0).slice(1, 1000000).map(n => (n, from(n))).reduceLeft((a, b) => if (a._2 > b._2) a else b)._1  
    r 
  }
        
       
}
