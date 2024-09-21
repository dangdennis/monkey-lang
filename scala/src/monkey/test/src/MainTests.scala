package test.monkey

import utest._

object MainTests extends TestSuite {
  val tests = Tests {
    val x = 2
    test("outer1") {
      val y = x + 1

      test("inner1") {
        assert(x == 2, y == 3)
        (x, y)
      }
      test("inner2") {
        val z = y + 1
        assert(z == 4)
      }
    }
    test("outer2") {
      test("inner3") {
        assert(x > 1)
      }
    }
  }
}
