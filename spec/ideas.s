;; primitives
;; U8, U16, U32, U64, USize
;; I8, I16, I32, I64, ISize
;; F32, F64
;; Str, Char, Bool, Fn


;; defines
(def name USize 10)
(def name Fn (fn add(a USize, b USize) USize (+ a b)))

;; functions
(fn name (arg0, arg1) Bool
  (= arg0, arg1))

;; if else
(if (= a, b)
  (true)
  (false))

;; real type with fields
(Type :StructName Any (
  :field0 ISize,
  :field1 USize))

(Function :name (a, b)
  (+ a, b))

;; create new types
(StructName 1, 1)

;; bits type of 4 bytes
(Type :BitTypeName Any 4)

;; abstract type
(Type :StructName Any :abstract)


(mod name (
  ;; introduces new sub scope
))

;; let
(let (a U8, b Char, c Bool) (256, 'c', true)
  (= a, b, c))

(def add
  [a Any, b Number]
  (+ a, b)
