import Data.Char
import Data.List.Split

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
    putStrLn $ show maxCals
    -- yay part one is done
