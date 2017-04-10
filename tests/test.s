(def defn (Macro `defn `(name, args, body)
  `(List `def name (List `Fn name args body))))

(defn add (a, b)
  (add_uint64 a, b))

(add 1, 1)
