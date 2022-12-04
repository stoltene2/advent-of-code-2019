(ns twenty-twenty-two.day-03
  (:require
   [clojure.java.io :as io]
   [clojure.set :as set]
   [clojure.string :refer [split-lines]]))


(def input (->> "input-03.txt"
                io/resource
                io/file slurp
                split-lines
                (into [])))

(def priorities {\a 1 \b 2 \c 3 \d 4 \e 5 \f 6 \g 7 \h 8 \i 9 \j 10 \k 11 \l
  12 \m 13 \n 14 \o 15 \p 16 \q 17 \r 18 \s 19 \t 20 \u 21 \v 22 \w 23
  \x 24 \y 25 \z 26 \A 27 \B 28 \C 29 \D 30 \E 31 \F 32 \G 33 \H 34 \I
  35 \J 36 \K 37 \L 38 \M 39 \N 40 \O 41 \P 42 \Q 43 \R 44 \S 45 \T 46
  \U 47 \V 48 \W 49 \X 50 \Y 51 \Z 52 nil 0})

(defn- find-priority [bag]
  "For a given bag find which item is common to both and print out
  it's priority value"
  (let* [compartment-size (/ (count (clojure.string/split bag #"")) 2)
         c1 (into #{} (take compartment-size bag))
         c2 (into #{} (drop compartment-size bag))]
    (priorities (first (set/intersection c1 c2)))))

(defn solution1 []
  "Print solution for part 1"
  (apply + (map find-priority input)))



;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; pt 2

(def sample-input-2 ["vJrwpWtwJgWrhcsFMMfFFhFp"
                     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                     "PmmdzqPrVvPwwTWBwg"])


(defn- to-chars [str]
  "split string to chars"
  (into #{} (take (count str) (clojure.string/split str #""))))

(defn- group-result [group]
  "Given a vec of 3 bags find which element is common to the group and
  return the priority value"

  (let [b1 (to-chars (first group))
        b2 (to-chars (second group))
        b3 (to-chars (nth group 2))]
    (-> (set/intersection b1 b2 b3)
        first ; First item of set
        first ; convert from string to char
        priorities)))


(defn- go [bags sum]
  "Given a collection of bags, iterate through 3 at a time and sum up
  the priorities from each group."
  (if (empty? bags)
    sum
    (let [result (group-result (take 3 bags))]
      (recur (drop 3 bags) (+ sum result)))))


(defn solution2 []
  (go input 0))
