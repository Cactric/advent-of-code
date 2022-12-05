import Data.Char
import Data.List.Split
import Data.List

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

crate :: String -> Char
crate x = if (head x == '[') then x !! 1 else ' '

parseCrate :: String -> [Char]
parseCrate x = map (crate) (chunksOf 4 x)

parseCrates :: [String] -> [[Char]]
parseCrates x = map parseCrate x

transposeCrates :: [[Char]] -> [[Char]]
transposeCrates ([]:_) = []
transposeCrates x = (map head x) : transposeCrates (map tail x)

parseMove :: String -> (Int, Int, Int)
--                      Num, From,To
parseMove x = (read (b !! 1), read (b !! 3), read (b !! 5)) where b = splitOn " " x

-- From StackOverflow
replaceNth :: Int -> a -> [a] -> [a]
replaceNth _ _ [] = []
replaceNth n newVal (x:xs)
  | n == 0 = newVal:xs
  | otherwise = x:replaceNth (n-1) newVal xs

numberList :: [a] -> [(Int, a)]
numberList [x] = [(i,x) | i <- [1..]]

doMove :: (Int, Int, Int) -> [[Char]] -> [[Char]]
doMove (n, from, to) y = replaceNth (from-1) (drop n (y !! (from-1))) (replaceNth (to-1) (reverse (take n (y !! (from-1))) ++ (y !! (to-1))) y)

orderMove :: (Int, Int, Int) -> [[Char]] -> [[Char]]
orderMove (n, from, to) y = replaceNth (from-1) (drop n (y !! (from-1))) (replaceNth (to-1) (take n (y !! (from-1)) ++ (y !! (to-1))) y)

move :: [String] -> [[Char]] -> [[Char]]
move (x:xs) y = if (isPrefixOf "move " x) 
    then move xs ( (doMove (parseMove x) y))
    else move xs y
move [] y = y

-- Part 2 function
move9001 :: [String] -> [[Char]] -> [[Char]]
move9001 (x:xs) y = if (isPrefixOf "move " x) 
    then move9001 xs ( (orderMove (parseMove x) y))
    else move9001 xs y
move9001 [] y = y


headOrEmpty :: String -> Char
headOrEmpty [] = '_'
headOrEmpty (x:xs) = x

top :: [[Char]] -> String
top x = map (headOrEmpty) x

main :: IO()
main = do
    fileContents <- readLines "input"
    let crates = transposeCrates (parseCrates fileContents)
    let initial = map (dropWhile (==' ')) crates
    
    putStr "Part one: "
    putStrLn $ top (move fileContents initial)
    
    putStr "Part two: "
    putStrLn $ top (move9001 fileContents initial)
