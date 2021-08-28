# **Pseudo Code**

Passwords will be created recursively


Integer with amount of characters will be provided.


*function create strings* - 
take in ascii range, string vector, and counter

if string counter != O:
    for enumerated i in range ascii range:
        add i in ascii range to string vector
        decrease counter
        call function create string passing (ascii range, string vector , and counter)
if string counter == 0:
    save previous string vector value
    for i in enumerated range ascii range:
        add i value to string vector
        add string vector with string conversion to output vector
        call create string with values( previous string vector)






0000
0001
0002
0003
0004