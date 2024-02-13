Not yet finished!

define msg :>
String :>
&emsp;"Hello, world!"
;>

define helloWorld :>
() :>
&emsp;msg =>
&emsp;println
;>

define five :>
Int :>
&emsp;5
;>

define add3 :>  
Int -> Int -> Int -> Int :>  
&emsp;add => add  
;>  

define main :>  
() :>  
&emsp;readInt =>  
&emsp;id |>  
&emsp;id |>  
&emsp;add3 =>  
&emsp;add 5 =>
&emsp;println  
;>
