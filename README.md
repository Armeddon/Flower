Not yet finished!


``define readDouble :>
Int :>
    readInt =>
    identity |>
    add
;>

define main :>
() :>
    readDouble =>
    println
;>``
