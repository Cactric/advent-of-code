import Data.Char
import Data.List.Split
import Data.List

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

catchUp :: (Int, Int) -> (Int, Int) -> (Int, Int)
catchUp (hx, hy) (tx, ty) = if mags < 2^2 then (tx, ty) else ((tx + dx), (ty + dy)) where
    dx = signum (hx - tx)
    dy = signum (hy - ty)
    mags = (hx - tx)^2 + (hy - ty)^2

doLeft :: Int -> (Int, Int) -> [(Int, Int)] -> [(Int, Int)]
doLeft 0 h ts = (catchUp h (head ts)):ts
doLeft n (hx, hy) ts = doLeft (n-1) nh $ (catchUp nh (head ts)):ts where
    nh = (hx-1, hy)

doDown :: Int -> (Int, Int) -> [(Int, Int)] -> [(Int, Int)]
doDown 0 h ts = (catchUp h (head ts)):ts
doDown n (hx, hy) ts = doDown (n-1) nh $ (catchUp nh (head ts)):ts where
    nh = (hx, hy - 1)

doUp :: Int -> (Int, Int) -> [(Int, Int)] -> [(Int, Int)]
doUp 0 h ts = (catchUp h (head ts)):ts
doUp n (hx, hy) ts = doUp (n-1) nh $ (catchUp nh (head ts)):ts where
    nh = (hx, hy + 1)

doRight :: Int -> (Int, Int) -> [(Int, Int)] -> [(Int, Int)]
doRight 0 h ts = (catchUp h (head ts)):ts
doRight n (hx, hy) ts = doRight (n-1) nh ((catchUp nh (head ts)):ts) where
    nh = (hx + 1, hy)

doAction :: String -> (Int, Int) -> [(Int, Int)] -> [(Int, Int)]
doAction a h ts = 
    if d == "L" then doLeft n h ts else
    if d == "R" then doRight n h ts else
    if d == "U" then doUp n h ts else
    {-if d == "D" then-} doDown n h ts where
        d = head(splitOn " " a)
        n = (read (head(tail (splitOn " " a))))

moveHead :: String -> (Int, Int) -> (Int, Int)
moveHead a (hx, hy) = 
    if d == "L" then (hx-n, hy) else
    if d == "R" then (hx+n, hy) else
    if d == "U" then (hx, hy+n) else
    {-if d then-} (hx, hy - n)  where
        d = (splitOn " " a) !! 0
        n = read((splitOn " " a) !! 1)

-- Actions → Head → Old tails → New tails
ropeAction :: [String] -> (Int, Int) -> [(Int, Int)] -> [(Int, Int)]
ropeAction [] _ ts = ts
ropeAction (a:as) h ts = ropeAction as (moveHead a h) (doAction a h ts)

-- Returns the number of positions the tail visits at least once
ropeSimulate :: [String] -> Int
ropeSimulate x = length (nub (ropeAction x (0,0) [(0,0)]))

main :: IO()
main = do
    fileContents <- readLines "input"
    
    putStr "Part one: "
    putStrLn $ show (ropeSimulate fileContents)
    
    putStr "Part two: "
    putStrLn "?"
