(ns clojure-solution.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn read-file [path] 
  (->> 
    (slurp path)
    str/split-lines
    (map (comp (partial map #(Integer/parseInt %)) #(str/split % #", ")))))

(defn manhattan-dis [[x1 y1] [x2 y2]] 
  (+ (Math/abs (- x2 x1)) (Math/abs (- y2 y1))))

(defn update-grid [grid point input bounds] 
  (let [dis (map-indexed (fn [i p] [i (manhattan-dis point p)]) input)
        min (apply min-key second dis)] 
    (cond 
      (> (count (filter #(= (second %) (second min)) dis)) 1) grid
      (some #(contains? bounds %) point)
        (assoc-in grid [(first min) :infinite] true)
      :else (update-in grid [(first min) :count] (fnil + 0) 1))))

(defn get-coords [input] 
  (let [x-coords (map first input) 
        y-coords (map second input)] 
    [(apply min x-coords) (inc (apply max x-coords)) 
     (apply min y-coords) (inc (apply max y-coords))]))

(defn get-points [[minX maxX minY maxY]] 
  (for [x (range minX maxX) y (range minY maxY)] [x y]))

(defn part1 [input] 
  (let [coords (get-coords input)] 
    (->> 
      (reduce #(update-grid %1 %2 input (set coords)) {} (get-points coords))      
      (filter #(not (:infinite (val %))))
      (apply max-key #(:count (val %)))
      val)))

(defn total-dis [input point] 
  (->> 
    input 
    (map #(manhattan-dis point %))
    (reduce +)))

(defn part2 [input] 
  (->> 
    (get-points (get-coords input))
    (filter #(< (total-dis input %) 10000))
    count))

(defn -main
  [& args]
  (time (println (part2 (read-file "resources/input.txt")))))
