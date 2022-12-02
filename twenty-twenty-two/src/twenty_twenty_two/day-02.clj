(ns twenty-twenty-two.day-02
 (:require
  [clojure.java.io :as io]
  [clojure.string :refer [split-lines]]
  [clojure.set :refer [map-invert]]))

(def input (->> "input-02.txt"
                io/resource
                io/file slurp
                split-lines
                (map #(clojure.string/split %1 #" "))
                (into [])))

(def choice-points
  "A map to score how many points you get when choosing a value"
  {:rock 1
   :paper 2
   :scissor 3})

(def beats
  "A map termining what beats the supplied key"
  {:rock :paper
   :paper :scissor
   :scissor :rock})

(def loses
  "A map determining what loses to a supplied key"
  (map-invert beats))

(defn score [[elf me]]
  "Given an elf and what I should play determine the score. If I have
  the same thing then we give 3pts. If I have something that beats elf
  then I get 6. 0 otherwise."
  (let [round (cond
                (= elf me) 3
                (= me (beats elf)) 6
                :default 0)
        selected (get choice-points me)]
    (+ round selected)))

(defn parse [str]
  "Parse a string into Rock, Paper, Scissor"
  (case str
    "A" :rock
    "B" :paper
    "C" :scissor
    "Y" :paper
    "X" :rock
    "Z" :scissor))

(defn parse-pair [[elf-str me-str]]
  "Parse a vector of strings into their representations. For example, [A B] -> [:rock :paper]"
  [(parse elf-str) (parse me-str)])


(defn solution-pt1 []
  "part 1 solution"
  (apply + (map (comp score parse-pair) input)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Part 2

(defn parse-with-strategy [str]
  "Parse a string into Rock, Paper, Scissor and determining the strategy as well"
  (case str
    "A" :rock
    "B" :paper
    "C" :scissor
    "Y" :draw
    "X" :lose
    "Z" :win))

(defn determine-play [[elf-str strategy-str]]
  "Interpret the code and return a pair of what was played. In the
  code if it is (X) we need to lose, (Y) means draw and (Z) means win"
  (let [elf (parse-with-strategy elf-str)
        strategy (parse-with-strategy strategy-str)]
    (case strategy
      :win [elf (beats elf)]
      :lose [elf (loses elf)]
      :draw [elf elf])))


(defn solution-pt2 []
  "part 2 solution"
  (apply + (map (comp score determine-play) input)))
