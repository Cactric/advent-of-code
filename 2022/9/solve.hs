import Data.Char
import Data.List.Split
import Data.List

type Knot = (Int, Int)

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

catchUp :: Knot -> Knot -> Knot
catchUp (hx, hy) (tx, ty) = if mags < 2^2 then (tx, ty) else
    catchUp (hx, hy) ((tx + dx), (ty + dy)) where
    dx = signum (hx - tx)
    dy = signum (hy - ty)
    mags = (hx - tx)^2 + (hy - ty)^2

doLeft :: Int -> Knot -> [Knot] -> [Knot]
doLeft 0 h ts = (catchUp h (head ts)):ts
doLeft n (hx, hy) ts = doLeft (n-1) nh $ (catchUp nh (head ts)):ts where
    nh = (hx-1, hy)

doDown :: Int -> Knot -> [Knot] -> [Knot]
doDown 0 h ts = (catchUp h (head ts)):ts
doDown n (hx, hy) ts = doDown (n-1) nh $ (catchUp nh (head ts)):ts where
    nh = (hx, hy - 1)

doUp :: Int -> Knot -> [Knot] -> [Knot]
doUp 0 h ts = (catchUp h (head ts)):ts
doUp n (hx, hy) ts = doUp (n-1) nh $ (catchUp nh (head ts)):ts where
    nh = (hx, hy + 1)

doRight :: Int -> Knot -> [Knot] -> [Knot]
doRight 0 h ts = (catchUp h (head ts)):ts
doRight n (hx, hy) ts = doRight (n-1) nh ((catchUp nh (head ts)):ts) where
    nh = (hx + 1, hy)

doAction :: String -> Knot -> [Knot] -> [Knot]
doAction a h ts = 
    if d == "L" then doLeft n h ts else
    if d == "R" then doRight n h ts else
    if d == "U" then doUp n h ts else
    {-if d == "D" then-} doDown n h ts where
        d = head(splitOn " " a)
        n = (read (head(tail (splitOn " " a))))

moveHead :: String -> Knot -> Knot
moveHead a (hx, hy) = 
    if d == "L" then (hx-n, hy) else
    if d == "R" then (hx+n, hy) else
    if d == "U" then (hx, hy+n) else
    {-if d then-} (hx, hy - n)  where
        d = (splitOn " " a) !! 0
        n = read((splitOn " " a) !! 1)

-- Actions → Head → Old tails → New tails
ropeAction :: [String] -> Knot -> [Knot] -> [Knot]
ropeAction [] _ ts = ts
ropeAction (a:as) h ts = ropeAction as (moveHead a h) (doAction a h ts)

-- Returns the number of positions the tail visits at least once
ropeSimulate :: [String] -> Int
ropeSimulate x = length (nub (ropeAction x (0,0) [(0,0)]))

polyChase :: Knot -> [[Knot]] -> [[Knot]]
polyChase h [] = []
polyChase h (ts:tss) = ((catchUp h (head ts)):ts):(polyChase (head ts) tss)

polyLeft :: Int -> Knot -> [[Knot]] -> [[Knot]]
polyLeft 0 h tss = tss
polyLeft n (hx, hy) tss = polyLeft (n-1) nh $ polyChase nh tss where
    nh = (hx-1, hy)
polyRight :: Int -> Knot -> [[Knot]] -> [[Knot]]
polyRight 0 h tss = tss
polyRight n (hx, hy) tss = polyRight (n-1) nh $ polyChase nh tss where
    nh = (hx+1, hy)
polyDown :: Int -> Knot -> [[Knot]] -> [[Knot]]
polyDown 0 h tss = tss
polyDown n (hx, hy) tss = polyDown (n-1) nh $ polyChase nh tss where
    nh = (hx, hy-1)
polyUp :: Int -> Knot -> [[Knot]] -> [[Knot]]
polyUp 0 h tss = tss
polyUp n (hx, hy) tss = polyUp (n-1) nh $ polyChase nh tss where
    nh = (hx, hy+1)

polyAction :: String -> Knot -> [[Knot]] -> [[Knot]]
polyAction a h tss =
    if d == "L" then polyLeft n h tss else
    if d == "R" then polyRight n h tss else
    if d == "U" then polyUp n h tss else
    {-if d == "D" then-} polyDown n h tss where
        d = head(splitOn " " a)
        n = (read ((splitOn " " a) !! 1) :: Int)

polyknot :: [String] -> Knot -> [[Knot]] -> [[Knot]]
polyknot [] h tss = tss
polyknot (a:as) h tss = polyknot as (moveHead a h) (polyAction a h tss)

tentaKnot :: [String] -> Int
tentaKnot x = length (nub (last (polyknot x (0,0) (replicate 9 [(0,0)]))))

main :: IO()
main = do
    fileContents <- readLines "input"
    
    putStr "Part one: "
    putStrLn $ show (ropeSimulate fileContents)
    
    putStr "Part two (not correct): "
    putStrLn $ show $ tentaKnot fileContents

test :: IO()
test = do
    let simple = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]
    let complex = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"]
    
    putStr "Part one (simple, should be 13): \t"
    putStrLn $ show (ropeSimulate simple)
    
    putStr "Part two (simple, should be 1): \t"
    putStrLn $ show (tentaKnot simple)
    
    putStr "Part two (complex, should be 36): \t"
    putStrLn $ show (tentaKnot complex)
