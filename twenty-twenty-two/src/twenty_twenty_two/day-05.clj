(ns twenty-twenty-two.day-05
  (:require
   [clojure.string :as str]
   [clojure.java.io :as io]))


(def start-stacks
  ['(\Q \F \L \S \R)
   '(\T \P \G \Q \Z \N)
   '(\B \Q \M \S)
   '(\Q \B \C \H \J \Z \G \T)
   '(\S \F \N \B \M \H \P)
   '(\G \V \L \S \N \Q \C \P)
   '(\F \C \W)
   '(\M \P \V \W \Z \G \H \Q)
   '(\R \N \C \L \D \Z \G)])


(def input (->> "input-05.txt"
                io/resource
                io/file slurp
                str/split-lines
                (drop 10)
                (into [])))


(def move-re
  "Parse the quantity, from and to stacks"
  #"move\ (\d+)\ from\ (\d+)\ to\ (\d+)")


(defn extract-move [str]
  "Parse the move out of an instruction and return a vector of
  the [quantity from to]. `from` and `to` are converted to the
  zero-based index."
  (let* [instr (->> (re-matches move-re str)
                    (drop 1)
                    (map parse-long))
         qty  (first instr)
         from (dec (second instr))
         to   (dec (nth instr 2))]
    [qty from to]))


(defn move-items [stacks [qty from to]]
  "Given a collection and a vector `[qty from to]` it will return a
  new collection with qty items moved between `from` and `to` in stack
  based order."

  (let [old-from (nth stacks from)
        move-stack (reverse (take qty old-from))
        new-from (drop qty (nth stacks from))
        new-to (concat move-stack (nth stacks to))
        ]
    (assoc stacks from new-from to new-to)))


(defn solution1 []
  ""
  (->> (map extract-move input)
       (reduce #(move-items %1 %2) start-stacks)
       (map first)
       clojure.string/join))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Part 2


(defn move-items-crane-9001 [stacks [qty from to]]
  "Given a collection and a vector `[qty from to]` it will return a
  new collection with qty items moved between `from` and `to` in their
  original order."

  (let [old-from (nth stacks from)
        move-stack  (take qty old-from)
        new-from (drop qty (nth stacks from))
        new-to (concat move-stack (nth stacks to))
        ]
    (assoc stacks from new-from to new-to)))

(defn solution2 []
  ""
  (->> (map extract-move input)
       (reduce #(move-items-crane-9001 %1 %2) start-stacks)
       (map first)
       clojure.string/join))
