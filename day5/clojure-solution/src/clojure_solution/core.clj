(ns clojure-solution.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn update-stack [c stack] 
  (if (and (not-empty stack) (= (bit-xor (int (peek stack)) (int c)) 32))
      (pop stack)
      (conj stack c)))
    
;; part1
(defn react [input] 
  (->> (reduce #(update-stack %2 %1) [] input) count))

;; part 2
(defn find-shortest [m c input] 
  (->> 
    (str/replace input (re-pattern (str c "|" (str/upper-case c))) "")
    react 
    (min m)))

(defn part2 [input] 
  (reduce #(find-shortest %1 %2 input) Integer/MAX_VALUE "abcdefghijklmnopqrstuwxyz"))

(defn -main
  [& args] 
  (time (println (part2 (slurp "resources/input.txt")))))