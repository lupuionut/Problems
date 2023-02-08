/*

Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?

*/

package me.ionut.rezolvari
import io.Source._

object Problema22 {

  def rezolvare() = {
    val continut = readInput("./src/main/scala/ionut/rezolvari/problem22.txt").map( _.replace("\"","")).sorted
    continut.map( w => wordValue(w)*(continut.indexOf(w)+1) ).sum
  }

  def readInput(s: String):Array[String]  = {
    fromFile(s).mkString.split(",")
  }

  def wordValue(s: String): Int = {
    s.map(c => c.toInt - 64).sum
  }
  
  def getCurrentDirectory = new java.io.File(".").getCanonicalPath
}
