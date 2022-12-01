(ns two-thousand-twenty-two.day-one
  (:require [clojure.string :refer [split-lines]]))

(def input (clojure.string/split-lines (slurp "input.txt")))

(defn solution-pt1 [input]
  (->> input
       (map parse-long)
       (partition-by nil?)
       (map #(or (apply + %1) 0))
       (apply max)))


(defn solution-pt2 [input]
  (->> input
       (map parse-long)
       (partition-by nil?)
       (map #(or (apply + %1) 0))
       sort
       reverse
       (take 3)
       (apply +)))
