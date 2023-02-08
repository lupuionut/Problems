/*
 *lattice path = (p1, p2) destination = coeficient binomial (p1+p2)/p2 = (p1+p2)!/(p1+p2-p2)! * p2!
  Combinari de n luate cate k = n!/((n-k)! * k!)
 */

package me.ionut.rezolvari

object Problema15 {

  def rezolvare() = {
    val bc = (21 to 40).map(_.toDouble).product / (1 to 20).map(_.toDouble).product
    bc.toLong 
  }

}
