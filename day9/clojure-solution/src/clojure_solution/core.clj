(ns clojure-solution.core
  (:gen-class))

(def players 416)
(def points 71975)

(defn helper [num remove-item add queue] 
  (loop [i 0 q queue] 
    (if (= i num)
      q 
      (let [e (remove-item q)
            c (add q e)] 
        (recur (inc i) q)))))

(defn rotate [queue num] 
  (cond 
    (= num 0) queue 
    (> num 0) (helper num #(.removeLast %) #(.addFirst %1 %2) queue)
    :else (helper (- (Math/abs num) 1) #(.remove %) #(.addLast %1 %2) queue)))

(defn play [[curr scores circle] marble] 
  (if (= (rem marble 23) 0)
      (let [c (rotate circle -7) 
            new (update scores curr (fnil + 0) (+ marble (.pop c)))] 
        [(rem (inc curr) players) new c])
      (let [c (rotate circle 2)
            new-cirlce (.addLast circle marble)] 
        [(rem (inc curr) players) scores circle])))

(defn solve [] 
  (let [[_ scores _] (reduce #(play %1 %2) [0 {} (java.util.ArrayDeque. [0])] (range 1 points))] 
    (val (apply max-key val scores))))

(defn -main
  [& args]
  (time (println (solve))))
