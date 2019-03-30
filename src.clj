(define [math a b c d]
  (+ a (- b c) d))

(math 1 2 3 (math 1 2 3 4))

(if (> 4 (math 1 2 3 4))
  (add 1 2)
  (println 3))

(cond
  (= 1 2) (this wont run)
  (= 1 2) (this will run)
  :else (println 1))
