import Data.Char

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

parseGame :: String -> (Char, Char)
parseGame x = (x !! 0, x !! 2)

parseGames :: [String] -> [(Char, Char)]
parseGames x = map parseGame x

selectScore :: Char -> Int
selectScore 'X' = 1
selectScore 'Y' = 2
selectScore 'Z' = 3

winScore :: (Char, Char) -> Int
winScore ('A', 'X') = 3 -- Draw (rock & rock)
winScore ('B', 'X') = 0 -- Loss (paper & rock)
winScore ('C', 'X') = 6 -- Win (scissors & rock)
winScore ('A', 'Y') = 6 -- Win (rock & paper)
winScore ('B', 'Y') = 3 -- Draw (paper & paper)
winScore ('C', 'Y') = 0 -- Loss (scissors & paper)
winScore ('A', 'Z') = 0 -- Loss (rock & scissors)
winScore ('B', 'Z') = 6 -- Win (paper & scissors)
winScore ('C', 'Z') = 3 -- Draw (both scissors)
winScore _ = 0 -- Completeness

oneScore :: (Char, Char) -> Int
oneScore (a, x) = selectScore x + winScore (a, x)

calcScore :: [(Char, Char)] -> Int
calcScore h = sum (map oneScore h)

selectTwoScore :: (Char, Char) -> Int
selectTwoScore ('A', 'X') = 3 -- Lose: Rock & Scissors
selectTwoScore ('A', 'Y') = 1 -- Draw: Rock
selectTwoScore ('A', 'Z') = 2 -- Win: Rock & Paper
selectTwoScore ('B', 'X') = 1 -- Lose: Paper & Rock
selectTwoScore ('B', 'Y') = 2 -- Draw: Paper
selectTwoScore ('B', 'Z') = 3 -- Win: Paper & Scissors
selectTwoScore ('C', 'X') = 2 -- Lose: Scissors & paper
selectTwoScore ('C', 'Y') = 3 -- Draw: scissors
selectTwoScore ('C', 'Z') = 1 -- Win: scissors & rock
selectTwoScore _ = 0

winTwoScore :: Char -> Int
winTwoScore 'X' = 0 -- Lose
winTwoScore 'Y' = 3 -- Draw
winTwoScore 'Z' = 6 -- Win
winTwoScore _ = 0 -- ???

twoScore :: (Char, Char) -> Int
twoScore (a, x) = selectTwoScore (a, x) + winTwoScore x

calcTwoScore :: [(Char, Char)] -> Int
calcTwoScore h = sum (map twoScore h)

main :: IO()
main = do
    fileContents <- readLines "input"
    let games = parseGames fileContents
    putStr "Part one: "
    putStrLn (show (calcScore games))
    
    putStr "Part two: "
    putStrLn (show (calcTwoScore games))
