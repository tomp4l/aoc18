package aoc18
package day18

import java.lang.module.ModuleDescriptor.Opens
import cats.Show
import cats.effect.IO
import cats.syntax.all.*
import cats.data.State

enum Forest:
  case Open
  case Tree
  case Lumberyard

given Show[Forest] = (f) =>
  f match
    case Forest.Open => "."
    case Forest.Tree => "|"
    case Forest.Lumberyard => "#"

object Program extends Some2dDay with PureDay:
  type I = Forest
  // Members declared in aoc18.Some2dDay
  def parseItem(i: String): IO[I] =
    IO(i match
      case "." => Forest.Open
      case "|" => Forest.Tree
      case "#" => Forest.Lumberyard
    )

  val passMinute = State
    .get[A]
    .flatMap(state =>
      state.keys.toList.traverse(p =>
        val surrounding = (p.neighbours ++ p.diagonals).flatMap(state.get(_))
        state.get(p) match
          case Some(Forest.Open) if surrounding.count(_ == Forest.Tree) >= 3 =>
            State.modify[A](_.updated(p, Forest.Tree))
          case Some(Forest.Tree)
              if surrounding.count(_ == Forest.Lumberyard) >= 3 =>
            State.modify[A](_.updated(p, Forest.Lumberyard))
          case Some(Forest.Lumberyard)
              if surrounding.count(_ == Forest.Lumberyard) < 1 || surrounding
                .count(_ == Forest.Tree) < 1 =>
            State.modify[A](_.updated(p, Forest.Open))
          case _ => State.empty[A, Unit]
      )
    )

  def score(s: A) =
    val trees = s.count(_._2 == Forest.Tree)
    val lumberyards = s.count(_._2 == Forest.Lumberyard)
    (trees * lumberyards)

  def part1(input: A): String =
    val endState = (1 to 10).toList.traverse(_ => passMinute).runS(input).value
    score(endState).toString

  def part2(input: A): String =
    val magic = 420

    val endState =
      (1 to (magic + 100)).toList
        .traverse(_ => passMinute.get.map(score))
        .runA(input)
        .value
        .zipWithIndex

    def detectCycle(list: List[(Int, Int)]): (Int, Int) =
      val cycleDetected = list.tail.find(_._1 == list.head._1)
      cycleDetected match
        case None => detectCycle(list.tail)
        case Some(_, i) => (list.head._2, i - list.head._2)

    val cycle = detectCycle(endState.drop(magic))
    val cycleSize = cycle._2

    assert(
      cycle._1
        .to(cycle._1 + cycleSize)
        .map(v => detectCycle(endState.drop(v)))
        .forall(_._2 == cycleSize)
    )

    val n = (1000000000 - 1) % cycleSize
    val x = (magic % cycleSize)
    val y = magic + cycleSize + n - x
    endState(y)._1.toString
  end part2

end Program
