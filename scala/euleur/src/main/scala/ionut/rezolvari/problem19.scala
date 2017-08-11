/*
You are given the following information, but you may prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*/

package me.ionut.rezolvari

object Problema19 {

  def rezolvare() = {
    check_points.filter(x => x % 7 == 0 && x > 365).length
  }

  def check_points = {
    (1901 to 2000).scanLeft((0,1900))((acc,year) => (acc._1+year_length(year-1), year)).map(x => year_points(x._2, x._1)).flatten
  }

  def is_leap_year(year: Int): Boolean = {
    if (year%4 == 0) {
      if (year%100 == 0 && year%400 == 0) {
        true
      } else if (year%100 == 0 && year%400 != 0) {
        false
      } else {
        true
      }
    } else false
  }

  def year_length(year: Int): Int = if (is_leap_year(year)) 366 else 365

  def year_points(year: Int, start: Int): List[Int] = {
    if (is_leap_year(year)) {
      List(start+1,start+32,start+61,start+92,start+122,start+153,start+183,start+214,start+245,start+275,start+306,start+336)
    } else {
      List(start+1,start+32,start+60,start+91,start+121,start+152,start+182,start+213,start+244,start+274,start+305,start+335)
    }
  }

}
