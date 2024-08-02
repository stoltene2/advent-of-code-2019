(ns twenty-twenty-two.day-07
  (:require [clojure.zip :as z]))

(def root (z/vector-zip []))

;; (-> root
;;     (z/insert-left (z/ {:type :dir :name "foom"}))
;;     (z/append-child {:type :dir :name "bam"})
;;     (z/append-child {:type :dir :name "zam"})

;;     z/down

;;     (z/append-child {:type :dir :name "zzza"})
;;     ;; (z/insert-child {:type :file :name "thing" :size 232})
;;     z/root
;;     )


(def vzip (z/vector-zip [1 [2 3] 4]))

(-> vzip z/node)

(-> vzip z/next z/node)

(-> vzip
    z/next ; 1
    z/next ; [2 3]
    z/next ; 2
    z/next ; 3
    z/next ; 4
    z/next ; [1 [2 3] 4]
    z/end?)

(def loc-seq (iterate z/next vzip))

(->> loc-seq (take 6) (map z/node))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn is-dir? [node]
  (= (:type node) :dir))

(defn children [node]
  "Here is where you put your documentation"
  (or (:children node) []))

(defn new-dir [name]
  (z/zipper is-dir?
            children
            (fn [node children] (merge node {:children (seq children)}))
            {:name name :children (seq nil) :type :dir}))


(defn cd-root
  "Returns the loc of the root node"
  [loc]
  (let [p (z/up loc)]
    (if p
      (recur p)
      loc)))


(defn cd [loc name]
  (loop [n (z/down loc)]
    (cond
      (= name "/") (cd-root loc)
      (= (:name (z/node n)) name) n
      :default (recur (z/right n)))))


(defn ls [loc entries]
  "Creates the entries htat are present in the directory"
  (reduce #(z/append-child %1 %2) loc entries))

(def my-zip
  (-> (new-dir "/")
      (z/append-child {:name "file.txt"})
      (z/append-child {:name "other.txt"})
      (z/append-child {:name "dddd" :type :dir})))

(def commands [[:cd "/"]
               [:ls [{:type :dir :name "a"}
                     {:type :file :name "b.txt" :size 14848514}
                     {:type :file :name "c.dat" :size 8504156}
                     {:type :dir :name "d"}]]])


;; NOTES: there is a difference between returning a node or a location
(comment
  (-> (new-dir "/")
      (z/append-child {:name "file.txt"})
      (z/append-child {:name "other.txt"})
      (z/append-child {:name "dddd" :type :dir})
      (cd "dddd")
      (ls [{:name "bottom.txt"}])
      z/root
      )
  )

(loop [dirs my-zip
       c commands]
  (let [[cmd args] (first c)]
    (if-not (empty? c)
      (cond (= cmd :cd) (recur (cd dirs args)
                               (rest c))
            (= cmd :ls) (recur (ls dirs args)
                               (rest c)))

      dirs)))



[:cd "/"]

;; $ ls
;; dir a
;; 14848514 b.txt
;; 8504156 c.dat
;; dir d

[:ls [{:type :dir :name "a"}
      {:type :file :name "b.txt" :size 14848514}
      {:type :file :name "c.dat" :size 8504156}
      {:type :dir :name "d"}]]

;;$ cd a
[:cd "a"]


;;$ ls
;;dir e
;;29116 f
;;2557 g
;;62596 h.lst
[:ls [{:type :dir :name "e"}
      {:type :file :name "f" :size 29116}
      {:type :file :name "g" :size 2557}
      {:type :file :name "h.lst" :size 62596}]]

;;$ cd e
[:cd "e"]

;; $ ls
;; 584 i
;; $ cd ..
;; $ cd ..
;; $ cd d
;; $ ls
;; 4060174 j
;; 8033020 d.log
;; 5626152 d.ext
;; 7214296 k
