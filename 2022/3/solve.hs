import Data.Char

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

firstHalf :: String -> String
firstHalf x = take ((length x) `div` 2) x

secondHalf :: String -> String
secondHalf x = drop ((length x) `div` 2) x

same :: String -> Char -> Char
same [] _ = '?'
same (x:xs) c = if (c == x) then c else same xs c

firstOrQuestionMark :: String -> Char
firstOrQuestionMark [] = '?'
firstOrQuestionMark (x:xs) = x

-- This definition raises an exception if it isn't found
inBoth :: String -> String -> Char
inBoth [] y = '?'
inBoth x y = firstOrQuestionMark (dropWhile (== '?') (map (same x) y))

somethingIn :: [String] -> [String] -> [Char]
somethingIn [] _ = []
somethingIn _ [] = []
somethingIn (x:xs) (y:ys) = inBoth x y : somethingIn xs ys

scoreChar :: Char -> Int
scoreChar '?' = 0
scoreChar c = if isUpper c then (ord c - 38) else (ord c - 96)

-- Takes in a list of all the characters that were in both
score :: String -> Int
score s = sum (map scoreChar s)

triplets :: [String] -> [[String]]
triplets (x:y:z:as) = [[x, y, z]] ++ triplets as
triplets _ = []

inThree :: [String] -> Char
inThree (x:y:z) = firstOrQuestionMark (dropWhile (== '?')(map (same (head z)) (map (same x) y)))
inThree _ = '?'

whatInThree :: [[String]] -> [Char]
whatInThree ((x:y:z):xs) = inThree (x:y:z) : whatInThree xs
whatInThree _ = []

main :: IO()
main = do
    fileContents <- readLines "input"
    let leftPockets = map firstHalf fileContents
    let rightPockets = map secondHalf fileContents
    let common = somethingIn leftPockets rightPockets
    let scores = score common
    
    putStr "Part one: "
    putStrLn $ show scores
    
    let threes = triplets fileContents
    let tricommon = whatInThree threes
    
    putStr "Part two: "
    putStrLn $ show (score tricommon)
