(ns clojure-solution.core
  (:require [clojure.string :as str])
  (:gen-class))

(declare parse)

(defn parse-input [path] 
  (->> 
    (str/split (slurp path) #"\s+")
    (map #(Integer/parseInt %))))

(defn get-children [r input] 
  (loop [x r in input values [] t 0] 
    (if (= x 0) 
      [t values in]
      (let [[total value data] (parse in 0)] 
        (recur (dec x) data (conj values value) (+ t total))))))

(defn sum [meta data] 
  (apply + (take meta data)))

(defn sum-scores [values data meta] 
  (->> 
    (filter #(and (> % 0) (<= % (count values))) (take meta data))
    (map #(nth values (dec %)))
    (reduce +)))

(defn parse [input total] 
  (let [[children meta] (take 2 input) 
        new (drop 2 input)] 
    (if (= children 0)
      (let [value (sum meta new)] 
        [value value  (drop meta new)]) 
      (let [[total values data] (get-children children new) 
            value (sum-scores values data meta)] 
        [(+ total (sum meta data)) value  (drop meta data)]))))

(defn -main
  [& args]
  (let [input (parse-input "resources/input.txt")
        [total value _] (parse input 0)] 
    (printf "Total: %d\nRoot value: %d\n" total value)))
