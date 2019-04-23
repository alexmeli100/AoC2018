module Main where

import System.IO 
import Control.Monad
import qualified Data.IntSet as S

readInts :: String -> [Int]
readInts = map (readInt . filter (/= '+')) . lines 
  where readInt = read :: String -> Int

part1 :: String -> Int
part1 = sum . readInts

part2 :: String -> Int 
part2 = go S.empty . scanl (+) 0 . cycle . readInts 
  where go s (x:xs) 
          | x `S.member` s = x
          | otherwise = go (S.insert x s) xs

main :: IO ()
main = do
  input <- readFile "input.txt"
  print $ part1 input
  print $ part2 input

