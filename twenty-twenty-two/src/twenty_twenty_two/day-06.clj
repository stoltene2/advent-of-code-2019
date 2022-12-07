(ns twenty-twenty-two.day-06
  (:require
   [clojure.string :as str]
   [clojure.java.io :as io]))


(def input (->> "input-06.txt"
                io/resource
                io/file slurp
                str/split-lines
                first))

(defn solution1 []
  (loop [idx 4
         val input]
    (let [next-4 (into #{} (take 4 val))]
      (if (= 4 (count next-4))
        idx
        (recur (inc idx) (rest val))))))

(defn solution2 []
  (loop [idx 14
         val input]
    (let [next-14 (into #{} (take 14 val))]
      (if (= 14 (count next-14))
        idx
        (recur (inc idx) (rest val))))))
