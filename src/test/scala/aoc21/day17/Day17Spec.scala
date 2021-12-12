package aoc18
package day17

import cats.syntax.all.*

class FlowSpec extends munit.CatsEffectSuite:

  def testCase(input: String, expectedParsed: String, expectedFlowed: String) =
    val parsed = Program.parse(input.split("\n").toList)
    val maxY = parsed.map(_.map(_._1.y).max)

    test("it parses") {
      assertIO(
        parsed.map(_.show),
        expectedParsed
      )
    }

    test("it runs to stability") {
      assertIO(
        (maxY, parsed).mapN((y, p) => Program.runToStability(p, y).show),
        expectedFlowed
      )
    }
  end testCase

  testCase(
    """x=495, y=2..7
      |y=7, x=495..501
      |x=501, y=3..7
      |x=498, y=2..4
      |x=506, y=1..2
      |x=498, y=10..13
      |x=504, y=10..13
      |y=13, x=498..504""".stripMargin,
    """.....|......
      |...........#
      |#..#.......#
      |#..#..#.....
      |#..#..#.....
      |#.....#.....
      |#.....#.....
      |#######.....
      |............
      |............
      |...#.....#..
      |...#.....#..
      |...#.....#..
      |...#######..""".stripMargin,
    """.....|......
      |.....|.....#
      |#..#~~~~...#
      |#..#~~#|....
      |#..#~~#|....
      |#~~~~~#|....
      |#~~~~~#|....
      |#######|....
      |.......|....
      |..~~~~~~~~~.
      |..|#~~~~~#|.
      |..|#~~~~~#|.
      |..|#~~~~~#|.
      |..|#######|.""".stripMargin
  )

  testCase(
    """x=503, y=2..4
      |y=4, x=499..503
      |x=499, y=2..4
      |x=501, y=1..2""".stripMargin,
    """.|...
      |..#..
      |#.#.#
      |#...#
      |#####""".stripMargin,
    """..|...
      |~~~#..
      ||#~#.#
      ||#~~~#
      ||#####""".stripMargin
  )
  testCase(
    """x=498, y=2..6
      |x=505, y=2..6
      |y=6, x=498..505
      |x=500, y=3..4
      |x=502, y=3..4
      |y=4, x=500..502""".stripMargin,
    """..|.....
      |........
      |#......#
      |#.#.#..#
      |#.###..#
      |#......#
      |########""".stripMargin,
    """...|......
      |~~~~~~~~~~
      ||#~~~~~~#|
      ||#~#~#~~#|
      ||#~###~~#|
      ||#~~~~~~#|
      ||########|""".stripMargin
  )

  testCase(
    """x=498, y=2..6
      |x=505, y=3..6
      |y=6, x=498..505
      |y=4, x=500..502""".stripMargin,
    """..|.....
      |........
      |#.......
      |#......#
      |#.###..#
      |#......#
      |########""".stripMargin,
    """..|......
      |..|......
      |#~~~~~~~~
      |#~~~~~~#|
      |#~###~~#|
      |#~~~~~~#|
      |########|""".stripMargin
  )
end FlowSpec
