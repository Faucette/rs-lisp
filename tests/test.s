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

(def result (add 1, 1))

(print "1 + 1 = ", result)

(defn fac (x)
  (if (int_eq x 0)
    1
    (int_mul x (fac (int_sub x 1)))))

(fac 5)

[0, 1, 2, 3, 4]

(do
  (print vec_a)
  (print vec_b)
  (print vec_c))

(deftype Person (age, name))

(Person 10, "Bob")
