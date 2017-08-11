/*
    A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
        1/2	= 	0.5
        1/3	= 	0.(3)
        1/4	= 	0.25
        1/5	= 	0.2
        1/6	= 	0.1(6)
        1/7	= 	0.(142857)
        1/8	= 	0.125
        1/9	= 	0.(1)
        1/10	= 	0.1 
    Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
    Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
*/
package me.ionut.rezolvari

object Problema26 {

  def rezolvare() = {
    (2 to 1000).foldLeft((0,0)){(acc, x) => val r = imparte(1,x,List()); if (r > acc._2) (x,r) else acc}
  }


  /*
   * n - deimpartit
   * i - impartitor
   * acc - accumulator
   * functia imparte n la i, ia restul, 
   * verifica daca exista in acumulator, 
   *    daca exista inseamna ca ciclul se reia si atunci function returneaza lungimea acumulatorului
   *    daca nu exista, il aduga in acumulator si continua impartitul
   */
  def imparte(n:Int, i:Int, acc:List[Int]): Int = {
    val d = n%i
    if (d == 0 || acc.contains(d))
    {
      acc.length;
    }
    else
    {
      imparte(d*10, i, acc ++ List(d))
    }
  }
}
