
; all local functions are declared with defn
(defn add [a b]
  (+ a b))

; exported functions are declared as arguments
; to the pub function which marks symbols for exporting.
(pub defn mul [a b]
  (* a b))

; functions themselves can be declared with `fn`
; counter returns a function that increments its
; argument by 10
(defn counter [inc-by-fn]
   (fn [x] (inc-by-fn x 10)))


; Anonymous functions can also be named for recursion
((fn fac [n]
    (if (= 0 n)
       1
       (fac (* (- n 1))))) 5 ) ;; 120

; Varargs catch first two in a and b and put everything else in the vector c
(defn add [ a b ... c ]
   (apply + (push-front a (push-front b c))))

(defn fac (x :UInt) :UInt
  (if (= x, 0_UInt)
    1_UInt
    (* x, (fac (- x, 1)))))

(defn add (a :UInt, b :UInt, ... rest :UInt) :UInt
  (print rest))
