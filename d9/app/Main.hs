module Main where

import GHC.Base

readFileToLists :: FilePath -> IO [[Int]]
readFileToLists = fmap (map (reverse . map read . words) . lines) . readFile -- For part1 remove the reverse

predict :: [Int] -> Int
predict v
  | all (== 0) v = 0
  | otherwise = last v + predict (zipWith (flip (-)) v (tail v))

main :: IO ()
main = readFileToLists "input" >>= print . sum . map predict
