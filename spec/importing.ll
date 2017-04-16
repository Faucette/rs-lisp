;; Each namespace is a collection of defs that is iterable
;; you can pass namespaces around like functions/objects

; ## BASIC IMPORTS
;
; you import by passing a vector to the import function
; this is the most basic way to import namespaces
; all defs inside each namespace can be referenced
; as <namespace>/<def>
;
; ## Examples
; λ=> (import [std.fmt])
; nil
; λ=> (std.fmt/println "hola mundo")
; "hola mundo"
; nil
;
(import [std.fmt
         std.io
         std.crypto])



; ## RENAMED IMPORTS
;
; to rename your imports to something short you need to
; pass a vector of vectors to import.
; Each vector contains 3 elements <symbol> <keyword> <symbol>
;
; ## Examples
; λ=> (import [std.fmt :as f])
; nil
; λ=> (f/println "hola mundo")
; "hola mundo"
; nil

(import [[std.fmt :as fmt]
         [std.web_sockets :as ws]])



; ## SCOPED IMPORTS
;
; You can also pull functions directly into the current namespace.
; Like renamed imports you need to bas a vector of vectors
; adhering to the following shape: <symbol> <keyword> <vector>
;
; ## Examples
; λ=> (import [std.fmt :only [println, print]])
; nil
; λ=> (println "hola mundo")
; "hola mundo"
; nil
(import [std.fmt :only [println, print]])
