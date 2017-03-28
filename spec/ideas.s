;; primitives
;; U8, U16, U32, U64, USize
;; I8, I16, I32, I64, ISize
;; F32, F64
;; Str, Char, Bool, Fn


;; defines
(def name USize 10)
(def anon Fn (fn add(a USize, b USize) USize))

;; functions
(fn name (arg0, arg1) Bool
  (= arg0, arg1))

;; if else
(if (= a, b)
  (true)
  (false))

;; struct (fn struct (field0, field1) StructName)
(struct StructName (
  field0 isize,
  field1 USize))

(. StructName 0)

;; let
(let (a U8, b Char, c Bool) (256, 'c', true)
  (= a, b, c))
