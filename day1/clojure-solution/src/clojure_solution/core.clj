(ns clojure-solution.core
  (:require [clojure.java.io :as io])
  (:gen-class))

(defn readInts [path] 
  (with-open [rdr (io/reader path)] 
    (doall (map #(Integer/parseInt %) (line-seq rdr)))))

(defn part1 [changes] 
  (reduce + changes))

(defn part2 [changes] 
  (let [freq (reductions + (cycle changes))] 
    (loop [[x & xs] freq seen #{0}] 
      (if (seen x) 
        x 
        (recur xs (conj seen x))))))

(defn -main
  [& args]
  (let [ints (readInts "resources/input.txt")] 
    (println (part2 ints))))
