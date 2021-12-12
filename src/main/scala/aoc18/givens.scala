package aoc18

import cats.Show
import cats.syntax.all.*
import scala.annotation.tailrec

given showIntTupleMap[A: Show]: Show[Map[(Int, Int), A]] with
  def show(m: Map[(Int, Int), A]): String =
    if m.isEmpty then ""
    else
      @tailrec
      def loop(
          remaining: List[((Int, Int), String)],
          acc: List[List[String]],
          minX: Int
      ): List[List[String]] =
        remaining match
          case Nil => acc.map(_.map(_.show))
          case ((_, x), i) :: rest =>
            val (c, n) =
              if (x == minX) then (List.empty, acc)
              else (acc.head, acc.tail)
            loop(rest, (c :+ i) :: n, minX)

      val maxX = m.maxBy(_._1._1)._1._1
      val maxY = m.maxBy(_._1._2)._1._2
      val minX = m.minBy(_._1._1)._1._1
      val minY = m.minBy(_._1._2)._1._2

      val allPoints = for
        x <- minX to maxX
        y <- minY to maxY
      yield (x, y)

      val filledMap = allPoints.foldLeft(m.map((c, i) => c -> i.show))((m, p) =>
        m.updatedWith(p) {
          case None => Some(".")
          case i => i
        }
      )

      val groups = loop(
        filledMap.toList
          .map { case ((x, y), a) => ((y, x), a.show) }
          .sortBy(_._1),
        List.empty,
        minX
      ).reverse

      val columnWidths = groups
        .flatMap(_.zipWithIndex)
        .groupMapReduce(_._2)(_._1.length)(math.max)
        .toList
        .sortBy(_._1)
        .map(_._2)

      val paddedGroups =
        groups.map(_.zip(columnWidths).map((s, i) => " " * (i - s.length) + s))

      paddedGroups
        .map(_.mkString)
        .mkString("\n")
  end show
end showIntTupleMap
