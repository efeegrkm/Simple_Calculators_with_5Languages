#lang racket

(display "İlk sayıyı girin: ")
(define a (read))
(display "İşlem gir (+, -, *, /)vs.: ")
(define op (read))
(display "İkinci sayıyı gir ")
(define b (read))
(define result
  (cond
    [(eq? op '+) (+ a b)]
    [(eq? op '-) (- a b)]
    [(eq? op '*) (* a b)]
    [(eq? op '/)
     (if (zero? b)
         (begin
           (display "Divided by zero")
           (exit))
         (/ a b))]
    [else
     (begin
       (display "Invalid operator.")
       (exit))]))



(display "Sonuç: ")
(display result)
(newline)
