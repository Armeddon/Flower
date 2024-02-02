Not yet finished!

Most complex thing ready for now:

define add3 :>
Int -> Int -> Int -> Int :>
&emsp;add => add
;>

define double :>  
Int -> Int :>  
&emsp;id |>  
&emsp;add  
;>  

define main :>  
() :>  
&emsp;readInt =>  
&emsp;double |>
&emsp;double |>
&emsp;double |>
&emsp;add3 =>
&emsp;println
;>
