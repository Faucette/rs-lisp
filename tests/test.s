(def string "this is a string")
(def char 'a')


(def defmacro (macro defmacro (name, args, body)
  (List `def name (List `macro name args body))))

(defmacro defn (name, args, body)
  (List `def name (List `fn name args body)))

(defmacro deftype (name, super, value)
  (List `def name (List `type name super value)))

(defn add (a, b)
  (add_uint64 a, b))

(def result (uint_add 1, 1))

(print "1 + 1 = ", result)

(defn fac (x)
  (if (uint_eq x 0)
    1
    (uint_mul x (fac (uint_sub x 1)))))

(fac 5)

[0, 1, 2, 3, 4]

(do
  (print vec_a)
  (print vec_b)
  (print vec_c))

(deftype Person Any (age, name))

(def bob (Person 42, "Bob"))

(@ bob :age)
(@ bob :age 56)
