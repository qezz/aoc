module Main where

import System.Environment (getArgs)

ruleAllAre :: (a -> a -> Bool) -> [a] -> Bool
ruleAllAre _ [] = True
ruleAllAre _ [_] = True
ruleAllAre f (x:y:xs) = f x y && ruleAllAre f (y:xs)

ruleAllInc :: Ord a => [a] -> Bool
ruleAllInc = ruleAllAre (<)

ruleAllDec :: Ord a => [a] -> Bool
ruleAllDec = ruleAllAre (>)

inRange :: Ord a => Num a => a -> a -> a -> Bool
inRange from to x = from <= x && x <= to

ruleDiffInRangeIncl ::  Ord a => Num a => a -> a -> [a] -> Bool
ruleDiffInRangeIncl from to lst =
  let diffInRange = \a b -> inRange from to $ abs (a - b)
  -- in diff >= from && diff <= to
  -- in
  --   foldl (\acc (x:y:_) -> acc && diff x y >= from && diff x y <= to) True lst
  in
    ruleAllAre diffInRange lst


reportIsSafe :: [Int] -> Bool
reportIsSafe lst = (ruleAllInc lst || ruleAllDec lst) && ruleDiffInRangeIncl 1 3 lst

lineIntoInts :: String -> [Int]
lineIntoInts = map read . words

main :: IO ()
main = do
  args <- getArgs
  let path = case args of
        (x:_) -> x
        []    -> error "Expects path to input file"

  content <- readFile path

  let lns = lines content
  -- print lns

  let nums = map lineIntoInts lns
  -- print nums

  -- print $ ruleAllInc [1, 2, 3]
  -- print $ ruleAllDec [1, 2, 3]
  -- print $ ruleAllInc [1, 2, 3] || ruleAllDec [1, 2, 3]
  -- print $ ruleDiffInRangeIncl 1 3 [1, 2, 3]
  -- print $ ruleDiffInRangeIncl 2 3 [1, 2, 3]

  let safe = map reportIsSafe nums
  let r = filter id safe
  let total = length r
  putStrLn $ "part1: " ++ show total
