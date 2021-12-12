package aoc18

lazy val primes: LazyList[Int] = 2 #:: LazyList
  .from(3, 2)
  .filter(i => primes.takeWhile { j => j * j <= i }.forall { k => i % k > 0 });
