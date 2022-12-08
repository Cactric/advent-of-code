import Data.Char

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

toInts :: [[Char]] -> [[Int]]
toInts x = map (map digitToInt) x

-- Easier to do left/right (probably) so let's make up/down left right too
transposeTrees :: [[a]] -> [[a]]
transposeTrees  ([]:_) = []
transposeTrees x = (map head x) : transposeTrees (map tail x)

-- True if visible
checkTree :: Int -> [Int] -> Bool
checkTree x [] = True
checkTree x (y:ys) = if x <= y then False else checkTree x ys

checkRight :: [Int] -> [Bool]
checkRight [] = []
checkRight (x:xs) = (checkTree x xs) : (checkRight xs)

checkLeftRight :: [Int] -> [Bool]
checkLeftRight x = zipWith (||) (reverse (checkRight (reverse x))) (checkRight x)

checkHorizontals :: [[Int]] -> [[Bool]]
checkHorizontals x = map checkLeftRight x

checkVerticals :: [[Int]] -> [[Bool]]
checkVerticals x = transposeTrees (checkHorizontals(transposeTrees x))

combineOr :: [[Bool]] -> [[Bool]] -> [[Bool]]
combineOr [] _ = []
combineOr _ [] = []
combineOr (xs:xss) (ys:yss) = zipWith (||) xs ys : combineOr xss yss

checkCardinal :: [[Int]] -> [[Bool]]
checkCardinal x = combineOr (checkVerticals x) (checkHorizontals x)

countBoolList :: Int -> [Bool] -> Int
countBoolList c [] = c
countBoolList c (x:xs) = if x == True then countBoolList (c+1) xs else countBoolList c xs

countBoolField :: [[Bool]] -> Int
countBoolField x = sum (map (countBoolList 0) x)

-- Part two things:
scoreTree :: Int -> Int -> [Int] -> Int
scoreTree c x [] = c
scoreTree c x (y:ys) = if x <= y then (c+1) else scoreTree (c+1) x ys

scoreRight :: [Int] -> [Int]
scoreRight [] = []
scoreRight (x:xs) = (scoreTree 0 x xs) : (scoreRight xs)

scoreLeftRight :: [Int] -> [Int]
scoreLeftRight x = zipWith (*) (reverse (scoreRight (reverse x))) (scoreRight x)

combineTimes :: [[Int]] -> [[Int]] -> [[Int]]
combineTimes [] _ = []
combineTimes _ [] = []
combineTimes (xs:xss) (ys:yss) = zipWith (*) xs ys : combineTimes xss yss

scoreVerticals :: [[Int]] -> [[Int]]
scoreVerticals x = transposeTrees $ scoreHorizontals $ transposeTrees x

scoreHorizontals :: [[Int]] -> [[Int]]
scoreHorizontals x = map scoreLeftRight x

scoreCardinal :: [[Int]] -> [[Int]]
scoreCardinal x = combineTimes (scoreVerticals x) (scoreHorizontals x)

highestScore :: [[Int]] -> Int
highestScore x = maximum (map (maximum) x)

main :: IO()
main = do
    fileContents <- readLines "input"
    let trees = toInts fileContents
    
    putStr "Part one: "
    putStrLn $ show $ countBoolField (checkCardinal trees)
    
    putStr "Part two: "
    putStrLn $ show $ highestScore $ scoreCardinal trees
