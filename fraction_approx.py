from fractions import Fraction

def separate_parts(value):
    whole_part = int(value)
    fractional_part = value - whole_part
    return whole_part, fractional_part

def approximate_float(input_value):
    whole_part, fractional_part = separate_parts(input_value)

    denominator = 1
    numerator = round(fractional_part * denominator)

    while abs(fractional_part - Fraction(numerator, denominator)) > accuracy_score:
        denominator += 1
        numerator = round(fractional_part * denominator)
        print("numerator: ", numerator)
        print("denominator: ", denominator)

    return numerator, denominator

############## Input values here
input_value = 3.1415926535897932384626433832795028841971
accuracy_score = 0.001
##############

whole_part, fractional_part = separate_parts(input_value)
numerator, denominator = approximate_float(fractional_part)

print("\n")

print(f"Desired accuracy: {accuracy_score} * 100%")

val = (whole_part * denominator + numerator)/denominator
print(f"End value: {val}")
print(f"End accuracy: {input_value/val}")

print(f"\nApproximation of {input_value}: {whole_part} + {numerator}/{denominator}, \
or {whole_part * denominator + numerator}/{denominator}.\n")

'''

This program finds the simplest fraction that is closest to a decimal.

Accuracy and running time are specified with "accuracy_score", the minimum fractional error
between the input and the output fraction.

Sample outputs:

Desired accuracy: 0.1 * 100%
End value: 3.2
End accuracy: 0.9817477042468103
Approximation of 3.141592653589793: 3 + 1/5, or 16/5.



'''