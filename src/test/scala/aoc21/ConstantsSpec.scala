package aoc18

class ConstantsSpec extends munit.FunSuite:
  test("it has first 10 primes") {
    assertEquals(
      primes.take(10).toList,
      List(2, 3, 5, 7, 11, 13, 17, 19, 23, 29)
    )
  }
