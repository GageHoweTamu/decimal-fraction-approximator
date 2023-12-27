from fractions import Fraction
import time

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
input_value = 3.14159265358979323846264338327950288419716939937510
accuracy_score = 0.0000000000001
##############

t1 = time.perf_counter() # Start timer

whole_part, fractional_part = separate_parts(input_value)
numerator, denominator = approximate_float(fractional_part)

print("\n")

print(f"Desired value: {input_value}")
print(f"Desired accuracy: {accuracy_score} * 100%\n")

val = (whole_part * denominator + numerator)/denominator
print(f"End value: {val}")
print(f"End accuracy: {input_value/val}")

print(f"\nApproximation of {input_value}: {whole_part} + {numerator}/{denominator}, \
or {whole_part * denominator + numerator}/{denominator}.\n")

t2 = time.perf_counter() # Start timer

print(f"Time elapsed: {t2 - t1} seconds")
