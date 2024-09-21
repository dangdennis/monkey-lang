import mill._, scalalib._

object monkey extends ScalaModule {
  def scalaVersion = "3.5.0"
  def ivyDeps = Agg(
    ivy"com.lihaoyi::scalatags:0.12.0",
    ivy"com.lihaoyi::mainargs:0.6.2"
  )

  object test extends ScalaTests {
    def ivyDeps = Agg(ivy"com.lihaoyi::utest:0.8.4")
    def testFramework = "utest.runner.Framework"
  }
}
