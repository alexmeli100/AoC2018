(ns clojure-solution.core
  (:require [loom.graph :as loom])
  (:gen-class))

(defn make-graph [graph line] 
  (let [[_ x y] (re-find #"Step (.) must be finished before step (.) can begin\." line)] 
    (assoc graph x (conj (get graph x #{}) y))))

(defn get-min [deps] 
  (apply min-key #(int (char (first %))) deps))

(defn read-file [path] 
  (with-open [rdr (clojure.java.io/reader path)] 
    (reduce make-graph {} (line-seq rdr))))

(defn min-dep [dg] 
  (->> 
    (for [node (loom/nodes dg) :when (= (loom/in-degree dg node) 0)] node)
    (apply min-key #(int (char (first %))))))

(defn solve [graph] 
  (loop [order [] dg graph] 
    (if (empty? (loom/nodes dg))
      order 
      (let [m (min-dep dg)] 
        (recur (conj order m) (loom/remove-nodes dg m))))))

(defn part1 [graph] 
  (let [dg (loom/digraph graph)] 
    (solve dg)))

(defn get-tasks [completed tasks min-time] 
  (->> 
    (map (fn [[k v]] [k (- v min-time)]) tasks)
    (filter (fn [[k v]] (not (some #(= k %) completed))))
    (into {})))

  (defn get-available [dg tasks] 
    (for [node (loom/nodes dg) 
          :when (and (= (loom/in-degree dg node) 0) (not (contains? tasks node)))] 
      node))

(defn schedule [dg tasks available time] 
  (if (and (not-empty available) (< (count tasks) 5)) 
      (let [m (get-min available)] 
        [dg (assoc tasks m  (- (int (first m)) 4)) time])
      (let [min-time (val (apply min-key val tasks))
            completed (map first (filter (fn [[k v]] (= v min-time)) tasks)) 
            new (get-tasks completed tasks min-time)]
        [(apply loom/remove-nodes dg completed) new (+ time min-time)])))

(defn part2 [graph] 
  (loop [dg graph tasks {} time 0] 
    (if (and (empty? (loom/nodes dg)) (empty? tasks)) 
        time 
        (let [available (get-available dg tasks) 
              [new-dg new-tasks new-time] (schedule dg tasks available time) ] 
          (recur new-dg new-tasks new-time)))))

(defn -main
  [& args]
  (time (println (part1 (read-file "resources/input.txt")))))
