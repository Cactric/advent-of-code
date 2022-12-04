import Data.Char
import Data.List.Split

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

parseSingle :: String -> (Int, Int)
-- “2-4”
parseSingle x = (read (head (splitOn "-" x)), read ((splitOn "-" x) !! 1))

parsePair :: String -> [(Int, Int)]
-- “2-4,6-8”
parsePair x = map parseSingle (splitOn "," x)

within :: (Int, Int) -> (Int, Int) -> Bool
within (a,b) (c,d) = (a >= c) && (b <= d)

overlapping :: (Int, Int) -> (Int, Int) -> Bool
overlapping (a,b) (c,d) = within (a,b) (c,d) || within (c,d) (a,b)

pairOverlapping :: [(Int, Int)] -> Bool
pairOverlapping [] = False
pairOverlapping (x:y:_) = overlapping x y

countTrues :: [Bool] -> Int -> Int
countTrues [] c = c
countTrues (x:xs) c = if x == True then countTrues xs (c+1) else countTrues xs c

main :: IO()
main = do
    fileContents <- readLines "input"
    
    let pairs = map parsePair fileContents
    let overlapBools = map pairOverlapping pairs
    
    putStr "Part one: "
    putStrLn $ show (countTrues overlapBools 0)
    
    putStr "Part two: "
    putStrLn "?"
