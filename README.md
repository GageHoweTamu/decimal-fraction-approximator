# decimal-fraction-approximator

### This program finds the simplest fraction closest to a decimal, like pi.

Accuracy is specified with "accuracy_score", the minimum fractional error
between the input and the output fraction.

### Sample output: (Given input_value = 3.1415926535897932384626433832795028841971, accuracy_score = 0.001)
numerator:  0    
denominator:  2    
numerator:  0    
denominator:  3    
numerator:  1    
denominator:  4        
...    
numerator:  9    
denominator:  62    
numerator:  9
denominator:  63    
numerator:  9    
denominator:  64    

* Desired accuracy: 0.001 * 100%
* End value: 3.140625
* End accuracy: 1.0003081086057053
* Approximation of 3.141592653589793: 3 + 9/64, or 201/64.
