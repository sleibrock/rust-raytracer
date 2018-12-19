;; This is a comment line please ignore
;; hello lol
(define width 640)    ; set the output image width to 640 pixels
(define height 480)   ; set the output image height to 480 pixels

(define filename "output.ppm") ; set the file name output

(define center #(0 0 0))
(define radius 100)

(define cam (camera #(-4 0 0) center))

(define W
  (world '(
	   (sphere center radius (lambert 0.5 0.5 0.5)))))


(render cam width height filename W)


;; end
