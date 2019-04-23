(ns clojure-solution.core
  (:require [clojure.java.io :as io] 
            [clojure.math.combinatorics :as comb]
            [clojure.data :refer [diff]])
  (:gen-class))

(defn check [freq x c] 
  (if (some #(= (val %) x) freq) (inc c) c))

(defn scan [[i j] freq] 
  [(check freq 2 i) (check freq 3 j)])

(defn part1 [lines] 
  (->> 
    (map frequencies)
    (reduce scan [0 0])
    (reduce *)))

;; part 2

(defn check-boxes [[x y]] 
  (->> 
    (map vector x y)
    (filter (fn [[i j]] (not= i j)))
    count 
    (= 1)))
  
(defn part2 [lines] 
  (->> 
    (comb/combinations lines 2) 
    (some #(when (check-boxes %) %))
    (map seq)
    (apply diff)
    last))

(defn solve [path] 
  (with-open [rdr (io/reader path)] 
    (part2 (line-seq rdr))))

(defn -main
  [& args]
  (println (solve "resources/input.txt")))
