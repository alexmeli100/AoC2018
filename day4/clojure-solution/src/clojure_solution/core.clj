(ns clojure-solution.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn read-file [path] 
  (with-open [rdr (clojure.java.io/reader path)] 
    (doall (sort (line-seq rdr)))))

(defn get-num [patt line] 
  (->> 
    (re-find patt line)
    second 
    Integer/parseInt))

(defn merge-data [g start end data]
  (let [mins (zipmap (range start end) (repeat 1))]
    (->
      (update-in data [g :total] (fnil + 0) (- end start))
      (update-in [g :mins] #(merge-with + mins %)))))

(defn process [lines] 
  (loop [[x & xs :as l] lines g nil start nil data {}] 
    (cond 
      (empty? l) data
      (str/ends-with? x "shift") 
        (recur xs (get-num #"#(\d+)" x) start data)
      (str/ends-with? x "asleep")
        (recur xs g (get-num #":(\d+)" x) data)
      (str/ends-with? x "up") 
        (recur xs g start (merge-data g start (get-num #":(\d+)" x) data)))))

(defn max-val [pred d] 
  (key (apply max-key pred d)))

;; part 1 and 2
(defn solve [data] 
  (let [max-guard1 (max-val #(:total (val %)) data)
        max-guard2 (max-val #(val (apply max-key val (:mins (val %)))) data)
        max-mins1 (max-val val (get-in data [max-guard1 :mins]))
        max-mins2 (max-val val (get-in data [max-guard2 :mins]))] 
    [(* max-guard1 max-mins1) (* max-guard2 max-mins2)]))

(defn -main
  [& args]
  (time (println (solve (process (read-file "resources/input.txt"))))))