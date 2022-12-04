(ns twenty-twenty-two.day-04
  (:require
   [clojure.java.io :as io]
   [clojure.string :refer [split-lines split]]))


(def input (->> "input-04.txt"
                io/resource
                io/file slurp
                split-lines
                (into [])))

(defn- parse-range [str]
  "Parse a range out of a string"
  (let [lower-upper (split str #"-")
        lower (parse-long (first lower-upper))
        upper (parse-long (second lower-upper))]
    [lower upper]))


(defn- parse-assignment [str]
  "Parse an assignment from a string"
  (let [assignments (split str #",")
        a1 (first assignments)
        a2 (second assignments)]
    [(parse-range a1) (parse-range a2)]))


(defn- range-contains? [[r1 r2]]
  "Returns true if r1 is fully contained in r2 or vice versa."
  (let [r1-l (first r1)
        r1-u (second r1)
        r2-l (first r2)
        r2-u (second r2)]
    (or
     ;; r2 contained in r1
     (and (<= r1-l r2-l)
          (<= r2-u r1-u))

     ;; r1 contained in r2
     (and (<= r2-l r1-l)
          (<= r1-u r2-u)))))


(defn solution1 []
  (->> input
       (map parse-assignment)
       (map range-contains?)
       (filter identity)
       count))


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Part 2

(defn- disjoint? [[r1 r2]]
  "True if there is not overlap"
  (let [r1-l (first r1)
        r1-u (second r1)
        r2-l (first r2)
        r2-u (second r2)]
    (or (< r1-u r2-l)
        (< r2-u r1-l))))

(defn- overlaps? [[r1 r2]]
  "Returns true if r1 and r2 overlap at all"
  (not (disjoint? [r1 r2])))

(defn solution2 []
  (->> input
       (map parse-assignment)
       (map overlaps?)
       (filter identity)
       count))
