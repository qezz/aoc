(ns day01.core
  (:gen-class))

(import '(java.io BufferedReader StringReader))

(def short-numbers
  [[["0"] 0]
   [["1"] 1]
   [["2"] 2]
   [["3"] 3]
   [["4"] 4]
   [["5"] 5]
   [["6"] 6]
   [["7"] 7]
   [["8"] 8]
   [["9"] 9]])

(def numbers
  [[["zero" "0"] 0]
   [["one" "1"] 1]
   [["two" "2"] 2]
   [["three" "3"] 3]
   [["four" "4"] 4]
   [["five" "5"] 5]
   [["six" "6"] 6]
   [["seven" "7"] 7]
   [["eight" "8"] 8]
   [["nine" "9"] 9]])

(defn find-match [item-list line]
  (let [filtered (filter #(clojure.string/starts-with? line %) (first item-list))]
    (if-not (empty? filtered)
      (second item-list)
      nil)))

(defn check2 [numbers line]
  (loop [i 0]
    (if (>= i (count numbers))
      nil
      (let [res (find-match (nth numbers i) line)]
        (if-not (nil? res)
          res
          (recur (inc i)))))))

(defn extract-code2-rec [lookup-table x y line]
  (if (empty? line)
    (if (nil? x)
      nil
      (+ (* 10 x) y))

    (let [found (check2 lookup-table line)
          tail-str (apply str (rest line))]
      (if (nil? found)
        (extract-code2-rec lookup-table x y tail-str)
        (if (nil? x)
          (extract-code2-rec lookup-table found found tail-str)
          (extract-code2-rec lookup-table x found tail-str))))))

(defn extract-code2-init [lookup-table line]
  (extract-code2-rec lookup-table nil nil line))

(defn figure-out [name lookup-table input-line-reader]
  (let [res (doall
             (map #(extract-code2-init lookup-table %) input-line-reader))]

    (println (str name ": " (reduce + res)))))

(defn -main
  "Adent of Code 2023 - Day 1"
  [& args]

  (def input (slurp "../input01.txt"))

  (let [lines (line-seq (BufferedReader. (StringReader. input)))]
    (figure-out "part1" short-numbers lines)
    (figure-out "part2" numbers lines)))
