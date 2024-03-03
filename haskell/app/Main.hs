module Main where

import Monkey qualified (someFunc)

main :: IO ()
main = do
  putStrLn "Hello, Haskell!"
  Monkey.someFunc
