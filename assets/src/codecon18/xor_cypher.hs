import Data.Char (digitToInt, chr, isAlphaNum, isLower, isUpper)
import Data.Bits (xor)
import Data.Text (splitOn, pack, unpack)
import Test.QuickCheck.Text(oneLine)
import Data.Ord (comparing)
import Data.List (maximumBy)

-- For an interactive use: type main, then type the input and end it with Ctrl+Z on Windows / Ctrl+D in a Bash Terminal
main :: IO ()
main = interact $ show . solve . oneLine

decode :: Int -> String -> String
decode key (a:b:xs) = decode_digit key a b : decode key xs
decode _ _ = []

decode_digit :: Int -> Char -> Char -> Char
decode_digit key a b = chr (xor key (16*digitToInt a + digitToInt b))

-- decode key (a:b:xs) = chr (xor key (16*digitToInt a + digitToInt b)) : decode key xs
            
splitWords :: String -> [String]
splitWords msg = concatMap words (map unpack (splitOn (pack "-") (pack msg)))

evalCred  :: String -> Integer
evalCred s = foldl sumScore 0 (splitWords s)
    where sumScore acc word = acc + 
            if all isAlphaNum word 
                then if isUpper (head word) && isLower (last word)
                    then 25
                    else 5
                else 0
                
-- Check it out like this:
-- solve "023f6c386b3f39222820326b3f246b392428206b2a6b392332262e6b3f246b392428206b2a6b392332262e6b3f232a3f6c386b39222c233f6b24256b3f22262e"
solve :: String -> String
solve s = maximumBy (comparing evalCred) (map (\key -> decode key s) [0..127])
