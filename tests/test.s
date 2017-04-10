(def string "this is a string")
(def char 'a')


(def defmacro (macro defmacro (name, args, body)
  (List `def name (List `macro name args body))))

(defmacro defn (name, args, body)
  (List `def name (List `fn name args body)))

(defn add (a, b)
  (add_uint64 a, b))

(def result (add 1, 1))

(print "1 + 1 = ", result)
