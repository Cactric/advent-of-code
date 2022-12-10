import Data.Char
import Data.List.Split

readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

-- Cycle number, cycle value
-- where the value is the value at the *end* of the cycle
type Cycle = (Int, Int)

cycleNum :: Cycle -> Int
cycleNum (n, v) = n

cycleVal :: Cycle -> Int
cycleVal (n, v) = v

-- Value -> Cycle number -> Cycle values -> Cycle Values
addx :: Int -> Int -> [Cycle] -> [Cycle]
addx x n vs = (n+2, ((cycleVal (head vs)) + x)):(n+1, (cycleVal (head vs))):vs

noop :: Int -> [Cycle] -> [Cycle]
noop n vs = (n+1, (cycleVal (head vs))):vs

-- Instructions -> Cycle number -> Cycle values -> Cycle values
recursiveExec :: [String] -> Int -> [Cycle] -> [Cycle]
recursiveExec [] n vs = vs
recursiveExec (i:is) n vs = if op == "addx" then
    recursiveExec is (n+2) (addx d n vs) else {-if op == "noop" then-}
    recursiveExec is (n+1) (noop n vs) where
        op = head $ splitOn " " i
        d = read $ (splitOn " " i) !! 1

signalStrengths :: [Cycle] -> [Int] -> [Int]
signalStrengths [] ss = ss
signalStrengths ((n,v):xs) ss = if n `mod` 40 == 19 then
    signalStrengths xs (((n+1) * v):ss) else
    signalStrengths xs ss

executor :: [String] -> Int
executor x = sum (signalStrengths (recursiveExec x 0 [(0,1)]) [])

lit :: Cycle -> Bool
lit nc = (n <= (c+1)) && (n >= (c-1)) where
    c = (cycleVal nc) `mod` 40
    n = cycleNum nc `mod` 40

plot :: [Bool] -> String
plot xs = map (isLit) xs where isLit x = if x then '#' else '.'

beamRacer :: [Cycle] -> String
beamRacer cs = plot $ map lit cs

-- From Stack Overflow
wrap :: Int -> a -> [a] -> [a]
wrap _ _ [] = []
wrap n c cs | n == length cs = c:cs++[c]
            | otherwise      = c:x ++ wrap n c y
              where (x,y) = splitAt n cs

main :: IO()
main = do
    fileContents <- readLines "input"
    
    putStr "Part one: "
    putStrLn $ show $ executor fileContents
    
    putStr "Part two: "
    putStrLn $ wrap 40 '\n' $ reverse $ beamRacer (recursiveExec fileContents 0 [(0,1)])

test :: IO()
test = do
    fileContents <- readLines "simpleInput"
    putStr "Part one test: "
    putStrLn $ show $ executor fileContents
    
    putStrLn "Part two:"
    putStrLn $ wrap 40 '\n' $ reverse $ beamRacer (recursiveExec fileContents 0 [(0,1)])
