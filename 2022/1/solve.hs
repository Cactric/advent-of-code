import Data.Char
import Data.List.Split
import Data.List


readLines :: FilePath -> IO [String]
readLines = fmap lines . readFile

lineGroup :: [String] -> [[String]]
lineGroup x = splitOn [""] x

groupToInt :: [String] -> [Int]
groupToInt x = map readInt x

readInt :: [Char] -> Int
readInt x = read x :: Int

main :: IO()
main = do
    contents <- readLines "input"
    let blocks = lineGroup contents
    let intBlocks = map groupToInt blocks
    let elfCals = map sum intBlocks
    let maxCals = maximum elfCals
    putStr "Part one: "
    putStrLn $ show maxCals
    -- yay part one is done
    
    let sortedCals = reverse (sort elfCals)
    let topThree = take 3 sortedCals
    putStr "Part two: "
    putStrLn $ show (sum topThree)
    
