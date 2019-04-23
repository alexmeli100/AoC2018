(ns clojure-solution.core
  (:require [clojure.java.io :as io])
  (:gen-class))

(defn parse-line [line] 
  (->> 
    (re-seq #"\d+" line)
    (map #(Integer/parseInt %))))

(defn read-file [path] 
  (with-open [rdr (io/reader path)] 
    (doall (map parse-line (line-seq rdr)))))

(defn get-indices [[_ x y w h]] 
  (for [i (range w) j (range h)] [(+ i x) (+ j y)]))

(defn update-grid [grid claim] 
  (->> 
    (get-indices claim)
    (reduce #(assoc %1 %2 (inc (get %1 %2 0))) grid)))

(defn part1 [claims data] 
  (count (filter #(> (val %) 1) data)))

;; part 2

(defn check-overlap [claim data] 
  (when (every? #(= 1 (get data %)) (get-indices claim)) 
    (first claim)))

(defn part2 [claims data] 
  (some #(check-overlap % data) claims))

(defn -main
  [& args]
  (let [claims (read-file "resources/input.txt") 
        data (reduce update-grid {} claims)] 
    (time (println (part2 claims data)))))
