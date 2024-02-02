Not yet finished!

Most complex thing ready for now:

A program that gets input X and returns (2*X+3)*4+5 showinng some functionality

define five :>
Int :>
    5
;>

define readAndDouble :>  
Int :>  
&emsp;readInt =>  
&emsp;identity |>  
&emsp;add  
;>  

define main :>  
() :>  
&emsp;readAndDouble => 
&emsp;add 3 => 
&emsp;identity |> 
&emsp;add => 
&emsp;identity |> 
&emsp;add => 
&emsp;five => 
&emsp;add => 
&emsp;println   
;>  
