(def string "this is a string")
(def char 'a')


(def defmacro (macro defmacro (name, args, body)
  (List `def name (List `macro name args body))))

(defmacro defn (name, args, body)
  (List `def name (List `fn name args body)))

(defmacro deftype (name, super, value)
  (List `def name (List `type name super value)))

(defn add (a, b)
  (uint_add a, b))

(def result (uint_add 1, 1))

(print "1 + 1 = ", result)

(defn fac (x)
  (if (uint_eq x 0)
    1
    (uint_mul x (fac (uint_sub x 1)))))

(fac 5)

[0, 1, 2, 3, 4]
{:key "value"}

(do
  (print [])
  (print [true, false]))

(deftype Person (age, name) Any)

(def bob (Person 42, "Bob"))

(let (a 10 b 20)
  (print a, b))

(def expr `(uint_add 1, 1))
(eval expr)


(namespace bob
  (def x 10)
  x)

x ; should be nil
