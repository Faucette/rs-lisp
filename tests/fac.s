(def defmacro (macro defmacro (name, args, body)
  (List `def name (List `macro name args body))))

(defmacro defn (name, args, body)
  (List `def name (List `fn name args body)))

(defmacro deftype (name, super, value)
  (List `def name (List `type name super value)))

(defn fac (x)
  (if (uint_eq x 0)
    1
    (uint_mul x (fac (uint_sub x 1)))))
