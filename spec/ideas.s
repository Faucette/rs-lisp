


fn fac<T: Zero + One + Mul + Sub>(x: T) -> T {
  if x == T::zero() {
    T::one()
  } else {
    x * fac::<T>(x - T::one())
  }
}

fac(10_usize); // infer type and call
fac::usize(10_usize);


fn fac(x: Number) -> Number {
  if x == 0 {
    1
  } else {
    x * fac(x - 1)
  }
}


UInt < Unsigned < Integer < Number
fac(10)

fn fac(x) {
  if x == 0 {
    1
  } else {
    x * fac(x - 1)
  }
}

fac(10)


@spec fac(UInt) -> UInt
fn fac(x) {
  if x == 0 {
    1
  } else {
    x * fac(x - 1)
  }
}

(spec fac (UInt) UInt)
(fn fac (x)
  (= x, 0)
    1
    (* x, (fac (- x, 1))))


// generic function
(gfn fac (T (Zero, One, Mul, Sub)) (x T) T
  (= x, (zero T))
    (one T)
    (* x (fac (- x (one T)))))

// define before use
(def fac::USize (fac (UInt)))
(fac::USize 10)

// check if template function then try and compile one then call
(fac 10)

// runtime check lookup/define then call
(fac (UInt) 10)


(fn fac (x Number) Number
  (= x, 0)
    1
    (* x (fac (- x 1))))

(fac 10)


(fn fac (x)
  (= x, 0)
    1
    (* x (fac (- x 1))))
