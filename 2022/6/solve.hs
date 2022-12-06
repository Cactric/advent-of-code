import Data.Char
import Data.List

different :: [Char] -> Bool
-- The nub function removes elements that aren't unique
different x = length (x) == length (nub x)

packetStart :: [Char] -> Int -> Int
packetStart s i = if different (take 4 s) then i else (packetStart (tail s) (i+1))

messageStart :: [Char] -> Int -> Int
messageStart s i = if different (take 14 s) then i else (messageStart (tail s) (i+1))

main :: IO()
main = do
    fileContents <- readFile "input"
    
    putStr "Part one: "
    putStrLn $ show (packetStart fileContents 4)
    
    putStr "Part two: "
    putStrLn $ show (messageStart fileContents 14)
