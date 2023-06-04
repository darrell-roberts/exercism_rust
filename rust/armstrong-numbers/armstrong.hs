import Data.List (unfoldr)

armstrong :: Integer -> Bool
armstrong n = result n == n
  where
    digits = tail .
        unfoldr (\(n, d) -> if d /= 0 then Just (d, n `divMod` 10) else Nothing) .
        (,1)
    totalDigits = length $ digits n
    result = sum . fmap (^totalDigits) . digits
