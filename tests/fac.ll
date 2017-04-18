(def defmacro (macro defmacro (name, args, body)
  (List `def name (List `macro name args body))))

(defmacro defn (name, args, body)
  (List `def name (List `fn name args body)))

(defn fac (x)
  (if (number_eq x 0)
    1
    (number_mul x (fac (number_sub x 1)))))

(fac 16)
