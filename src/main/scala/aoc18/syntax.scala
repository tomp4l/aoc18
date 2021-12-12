package aoc18

import cats.effect.IO

extension (s: String)
  def toIntIO: IO[Int] =
    IO.fromOption(s.toIntOption)(new Exception(s"Couldn't parse int from: $s"))

  def toBinaryInt: Int =
    Integer.parseInt(s, 2)

extension [A](option: Option[A])
  def toIO(t: => Throwable) = IO.fromOption(option)(t)
  def toIOException(message: => String) =
    option.toIO(new Exception(message))
