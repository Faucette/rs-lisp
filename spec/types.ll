;; BASIC VALUE TYPING

;; assert that x must be an integer
;; assertions of abstract classes only assert that the abstract is somewhere
;; in the parent hierarchy for the concrete instance.
;; because 33 is an Int64 which is a subtype of the abstract type Integer this is fine.
(: Integer
  (def x 33))

;; this will explode because 100 is by default an Int64
(: UInt8
  (def y 100 )) ;; prints: *type error* 100 is an Int64, expected UInt8

;; we can fix the above problem by calling UInt8 on 100
(: UInt8
  (def y (UInt8 100)))

;; when dealing with Lists we apply List to Any to create a List<Any> type.
(: (List Any)
  (def a `(1 "foo" 5.6)))

;; we can do the same thing for vectors
;; because we have Integer < Rational < Real < Number
;; we could do a Vector of Real or a Vector of Number here.
(: (Vector Number)
  (def v [1 6.7 3/4])) ;; v is [1 6.7 3/4]

(: (Vector Float64)
  (def v [1 6.7 3/4])) ;; *type error* 1 is an Integer expected Float64

;; we could convert beforehand though
(: (Vector Float64)
  (convert (Vector Float64) [1 6.7 3/4])) ;; [1.0 6.7 0.75]

;;  fn [typeinfo] name? [] (body)
(fn [(Integer, Integer) Integer] add [a b] (+ a b))

;;  Idea for Generics: just make the vector of size 3
;; ignore this for now though
;; fn [typeinfo] name? [] (body)
(fn [(T Numeric Drop Async) (T, T) T] add [a b] (+ a b))

;; in order to use:
(add (Int8 33) (Int8 65))
