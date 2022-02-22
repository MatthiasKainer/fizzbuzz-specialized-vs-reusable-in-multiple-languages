import time
from dataclasses import dataclass

def fizzbuzz(i):
    if   i % 3 == 0 and i % 5 == 0: return "FizzBuzz"
    elif i % 3 == 0: return "Fizz"
    elif i % 5 == 0: return "Buzz"
    else : return str(i)
   
@dataclass
class Rule:
    matches: lambda i: bool
    keyword: str
   
def speak(i, rule):
    (matches, keyword) = rule
    return keyword if matches(i) else ""
    
def number_or_all_matching_rules(i, rules):
    result = "".join(map(lambda rule: rule.keyword if rule.matches(i) else "", rules))
    return result if (result != "") else str(i)

def main():
    numbers=range(1, 1000001)
    start_time = time.perf_counter()
    fizzbuzzer = ";".join(map(fizzbuzz, numbers))
    end_time = time.perf_counter()
 
    print(f"Elapsed specialized:\t\t{((end_time - start_time) * 1000):0.2f}ms" )

    start_time = time.perf_counter()
    rules = (Rule(lambda i: i % 3 == 0, "Fizz"), Rule(lambda i: i % 5 == 0, "Buzz"))
    generic = ";".join(map(lambda i: number_or_all_matching_rules(i, rules), numbers))
    end_time = time.perf_counter()
 
    print(f"Elapsed reusable:\t\t{((end_time - start_time) * 1000):0.2f}ms" )
    
    print(f"Same results?\t\t\t{fizzbuzzer == generic}")

if __name__ == "__main__":
    main()
