# Decimal -> Fraction Approximation

### This simple algorithm finds the simplest fraction closest to a decimal. Eg. "approximate the value of pi with an error of less than 0.5%" (returns the fraction 22/7)

Accuracy is specified with "accuracy_score", the minimum fractional error
between the input and the output fraction.    

# Rust refactor!
Now refactored in Rust, which runs thousands of times faster.

Timer started    
Starting approximation    
Timer stopped    
3.141592653589793 approximates to 3 + 244252/1725033, which equals 3.1415926535898153. This approximation is *99.99999999999929%* accurate.    
Time elapsed is: 7.26475ms    

Compare this to Python, which took 11.82 seconds: *~1627 times slower.*

## Python

Sample output given input_value = 3.1415926535897932384626433832795028841971, accuracy_score = 0.001):    
"    
numerator:  0    
denominator:  2    
numerator:  0    
denominator:  3        
...    
numerator:  9    
denominator:  64    
Desired accuracy: 0.001 * 100%    
End value: 3.140625    
End accuracy: 1.0003081086057053    
Approximation of 3.141592653589793: 3 + 9/64, or 201/64.    
"
