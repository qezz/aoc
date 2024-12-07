{-# LANGUAGE ScopedTypeVariables #-}

module Main where

import Data.List

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

  putStrLn $ show x
