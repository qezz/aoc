module Main where

import System.Environment (getArgs)
import Data.Sequence (mapWithIndex, foldlWithIndex, fromList)

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


reportIsSafe :: Ord a => Num a => [a] -> Bool
reportIsSafe lst = (ruleAllInc lst || ruleAllDec lst) && ruleDiffInRangeIncl 1 3 lst

splitAtWithout :: Int -> [a] -> [a]
splitAtWithout n lst = take n lst ++ drop (n+1) lst

everyWithout1 :: [a] -> [[a]]
-- everyWithout1 lst = mapWithIndex (\i _ -> splitAtWithout i lst) lst
everyWithout1 lst = foldlWithIndex (\acc i _ -> acc ++ [splitAtWithout i lst]) [] (fromList lst)

reportIsSafeDamp :: [Int] -> Bool
reportIsSafeDamp = any reportIsSafe . everyWithout1

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

  -- print $ (reportIsSafe . everyWithout1) [1, 2, 3, 4]
  -- let listOfLists = everyWithout1 [1, 10, 0, 10, 0]
  -- let res = any reportIsSafe listOfLists
  -- print res

  let safe2 = length $ filter id $ map reportIsSafeDamp nums

  putStrLn $ "part2: " ++ show safe2
