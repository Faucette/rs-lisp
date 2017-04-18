(def string "this is a string")
(def char 'a')


(def defmacro (macro defmacro (name, args, body)
  (List `def name (List `macro name args body))))

(defmacro defn (name, args, body)
  (List `def name (List `fn name args body)))

(defn add (a, b)
  (number_add a, b))

(def result (number_add 1, 1))

(print "1 + 1 = ", result)

(defn fac (x)
  (if (number_eq x 0)
    1
    (number_mul x (fac (number_sub x 1)))))

(fac 5)

[0, 1, 2, 3, 4]
{:key "value"}

(do
  (print [])
  (print [true, false]))

(let (a 10 b 20)
  (print a, b))

(def expr `(number_add 1, 1))
(eval expr)


(namespace bob
  (def x 10)
  x)

x ; should be nil
