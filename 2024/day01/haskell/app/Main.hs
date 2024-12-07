{-# LANGUAGE ScopedTypeVariables #-}

module Main where

import Data.List

import Data.Map (Map)
import qualified Data.Map as Map


concat' :: String -> String -> String
concat' a b = a ++ b

listOfListsToStr :: [[String]] -> String
listOfListsToStr = concat . map (concat . intersperse " ")

xAppend :: [[a]] -> [a] -> [[a]]
xAppend acc x =
  let (firstList: secondList: _) = acc
      (firstItem: secondItem: _) = x
  in
    [firstList ++ [firstItem], secondList ++ [secondItem]]

oneByOneDiff :: [Integer] -> [Integer] -> [Integer]
oneByOneDiff [] [] = []
oneByOneDiff (x:xs) (y:ys) = (x - y) : oneByOneDiff xs ys

main :: IO ()
main = do
  content <- readFile "../input.txt"
  let lns = lines content

  let listOfListsOfStrings :: [[String]] = map words lns
  let listOfListsOfInts :: [[Integer]] = map (map read) listOfListsOfStrings

  let folded2 :: [[Integer]] = foldl xAppend [[], []] listOfListsOfInts

  let l1 = sort (folded2 !! 0)
  let l2 = sort (folded2 !! 1)

  let s = zipWith (\a b -> abs (a - b) ) l1 l2
  let x = sum s

  putStrLn $ "part1: " ++ show x

  let freqmap :: Map Integer Integer = foldl (
        \acc x -> Map.insertWith (+) x 1 acc
                      ) Map.empty l2

  -- putStrLn $ "l1: " ++ show l1
  -- putStrLn $ "freq map: " ++ show freqmap

  let res = foldl (\acc val ->
                     let t = Map.findWithDefault 0 val freqmap
                     in
                       acc + (val * t)
                  ) 0 l1

  putStrLn $ "part2: " ++ show res

