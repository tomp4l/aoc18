package aoc18
package day17

import cats.syntax.all.*
import cats.Show
import cats.effect.IO
import cats.data.State

given Show[Flow] = g =>
  g match
    case Flow.Clay => "#"
    case Flow.DownFlow => "|"
    case Flow.Still => "~"

enum Flow:
  case DownFlow
  case Still
  case Clay

object Program extends aoc18.PureDay:
  type FlowMap = Map[Point2d, Flow]
  type A = FlowMap
  def parse(input: List[String]): IO[A] =
    input
      .foldLeftM(Map.empty[Point2d, Flow]) { (m, l) =>
        l match
          case s"x=$x, y=$ymin..$ymax" =>
            (x.toIntIO, ymin.toIntIO, ymax.toIntIO).mapN((x, ymin, ymax) =>
              val newPoints =
                (ymin to ymax).map(y => Point2d(x, y) -> Flow.Clay)
              m ++ newPoints
            )
          case s"y=$y, x=$xmin..$xmax" =>
            (y.toIntIO, xmin.toIntIO, xmax.toIntIO).mapN((y, xmin, xmax) =>
              val newPoints =
                (xmin to xmax).map(x => Point2d(x, y) -> Flow.Clay)
              m ++ newPoints
            )
          case l => IO.raiseError(new Exception(s"unparsed: $l"))
      }
      .map(_ + (Point2d(500, 0) -> Flow.DownFlow))

  def simulateFlowDown(
      point: Point2d,
      maxY: Int
  ): State[FlowMap, Unit] =
    if point.y >= maxY then State.empty
    else
      State
        .get[FlowMap]
        .flatMap(state =>
          state.get(point.below) match
            case None =>
              State.modify[FlowMap](_.updated(point.below, Flow.DownFlow)) *>
                simulateFlowDown(point.below, maxY)
            case Some(Flow.Clay) =>
              simulateFlowSideways(point, maxY)
            case Some(Flow.Still) => simulateFlowSideways(point.below, maxY)
            case _ => State.empty[FlowMap, Unit]
        )

  def simulateFlowSideways(
      point: Point2d,
      maxY: Int
  ): State[FlowMap, Unit] =
    State
      .get[FlowMap]
      .flatMap(state =>
        def fillSidewaysTo(
            point: Point2d,
            next: Point2d => Point2d
        ): Point2d =
          val n = next(point)
          (state.get(point.below), state.get(n)) match
            case None -> _ => point
            case Some(Flow.DownFlow) -> _ => point
            case _ -> Some(Flow.Clay) => point
            case _ => fillSidewaysTo(n, next)

        val left = fillSidewaysTo(point, _.left)
        val right = fillSidewaysTo(point, _.right)
        val nextFlow = (state.get(left.below), state.get(right.below)) match
          case (None, None) =>
            simulateFlowDown(left, maxY) >> simulateFlowDown(right, maxY)
          case (None, _) => simulateFlowDown(left, maxY)
          case (_, None) => simulateFlowDown(right, maxY)
          case (Some(Flow.DownFlow), _) | (_, Some(Flow.DownFlow)) =>
            State.empty[FlowMap, Unit]
          case _ =>
            simulateFlowSideways(point.above, maxY)
        val allPoints =
          (left.x to right.x).map(x => Point2d(x, left.y) -> Flow.Still).toMap
        State.modify[FlowMap](_ ++ allPoints) *> nextFlow
      )
  end simulateFlowSideways

  def runToStability(input: A, maxY: Int): A =
    simulateFlowDown(Point2d(500, 0), maxY).runS(input).value

  def runWithFilter(input: A, filter: FlowMap => FlowMap) =
    val filtered = input
      .filter((_, v) =>
        v match
          case Flow.Clay => true
          case _ => false
      )
      .map(_._1.y)
    val maxY = filtered.max
    val minY = filtered.min
    val result = runToStability(input, maxY)
    filter(result)
      .count((p, v) => p.y <= maxY && p.y >= minY && v != Flow.Clay)
      .show

  def part1(input: A): String =
    runWithFilter(input, identity)

  def removeRow(p: Point2d): State[FlowMap, Unit] =
    State
      .modify[FlowMap](_.removed(p))
      .get
      .flatMap(state =>
        def remove(p: Point2d) =
          state.get(p) match
            case Some(Flow.Still) => removeRow(p)
            case _ => State.empty[FlowMap, Unit]
        remove(p.left) >> remove(p.right)
      )

  def part2(input: A): String =
    runWithFilter(
      input,
      result =>
        val noDown = result.filterNot(_._2 == Flow.DownFlow)
        val danglers = noDown.collect {
          case (p, Flow.Still) if (noDown.get(p.below).isEmpty) =>
            p
        }.toList
        danglers.traverse(removeRow).runS(noDown).value
    )

end Program
