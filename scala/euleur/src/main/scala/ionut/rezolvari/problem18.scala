/*
 *Find the maximum total from top to bottom of the triangle below:
 */

package me.ionut.rezolvari

object Problema18 {

  def rezolvare() = {
    val l = Array (
              List(75),
              List(95,64),
              List(17,47,82),
              List(18,35,87,10),
              List(20,4,82,47,65),
              List(19,1,23,75,3,34),
              List(88,2,77,73,7,63,67),
              List(99,65,4,28,6,16,70,92),
              List(41,41,26,56,83,40,80,70,33),
              List(41,48,72,33,47,32,37,16,94,29),
              List(53,71,44,65,25,43,91,52,97,51,14),
              List(70,11,33,28,77,73,17,78,39,68,17,57),
              List(91,71,52,38,17,14,91,43,58,50,27,29,48),
              List(63,66,4,68,89,53,67,30,73,16,69,87,40,31),
              List(4,62,98,27,23,9,70,98,73,93,38,53,60,4,23)
            )
    parse(l)
  }


  def parse(a: Array[List[Int]]): List[Int] = {
    if (a.length == 2)
      add_lists(a(0), zip(a(1)).map(pair_max))
    else {
      val ll = zip(a(a.length-1)).map(pair_max)
      val ab = a.take(a.length - 2) ++ Array(add_lists(ll, a(a.length-2)))
      parse(ab)
    }
  }

  def zip(line: List[Int]): List[(Int,Int)] = {
    if (line.length == 2)
      List((line.head, line.tail.head))
    else
      List((line.head, line.tail.head)) ++ zip(line.drop(1))
  }

  def pair_max(p: (Int, Int)): Int = math.max(p._1, p._2)

  def add_lists(l1: List[Int], l2: List[Int]): List[Int] = {
    if (l1.length == 1)
      List(l1.head + l2.head)
    else
      List(l1.head + l2.head) ++ add_lists(l1.tail, l2.tail)
  }
}
