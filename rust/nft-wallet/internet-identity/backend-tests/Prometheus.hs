module Prometheus where

import           Data.Attoparsec.Text
import           Data.Char            (isAlphaNum)
import qualified Data.Map             as M
import qualified Data.Text            as T
import           Data.Word            (Word64)

type MetricName = T.Text
type MetricValue = Double
type Timestamp = Word64

type KeyValue = (T.Text, T.Text)
type Params = [KeyValue]

type MetricsRepository = M.Map MetricName [(Params, MetricValue, Maybe Timestamp)]

lookupMetric :: MetricsRepository -> T.Text -> Either String (MetricValue, Maybe Timestamp)
lookupMetric repo name =
  case M.lookup name repo of
    Just [(_, value, ts)] -> Right (value, ts)
    Just _ -> Left "ambiguous request: metric value is not unique"
    Nothing -> Left "metric not found"

parseMetricsFromText :: T.Text -> Either String MetricsRepository
parseMetricsFromText = parseOnly (parser <* endOfInput)

-- | A parser for prometheus text data exposition format.
--
-- See  https://prometheus.io/docs/instrumenting/exposition_formats/#text-format-details 
parser :: Parser MetricsRepository
parser = M.fromListWith (++) <$> metrics
  where
    metrics = many' (skipMany comment *> metric)

    metric = do
      metricName <- name
      ps <- option [] params
      skipSpace
      val <- double
      skipSpace
      ts <- option Nothing (Just <$> decimal)
      skipSpace
      return (metricName, [(ps, val, ts)])

    params = char '{' *> (keyValue `sepBy` (char ',' *> skipSpace)) <* char '}'
    keyValue = (,) <$> name <*> (char '=' *> value)
    value = char '"' *> escapedString <* char '"'
    comment = char '#' *> manyTill anyChar endOfLine
    name = takeWhile1 (\c -> isAlphaNum c || c == '_')
    escapedString = scan False (\esc c -> case c of
                                '\\' -> Just (not esc)
                                '"'  -> if esc then Just False else Nothing
                                _    -> Just False)
