using System;
using System.Collections;
using System.Diagnostics;

using Data;

string FizzBuzz(int i)
{
    if (i % 15 == 0) return "FizzBuzz";
    else if (i % 3 == 0) return "Fizz";
    else if (i % 5 == 0) return "Buzz";
    else return i.ToString();
}


string numberOrAllMatchingRules(int i, List<Rule> rules)
{
    var results = string.Join("",
        from rule in rules
        where rule.Matches(i)
        select rule.Keyword
    );

    return results != "" ? results : i.ToString();
}

var range = Enumerable.Range(1, 1000001).ToList();
var watch = Stopwatch.StartNew();
var specialized = string.Join(";", from i in range select FizzBuzz(i));
watch.Stop();
Console.WriteLine("Elapsed specialized:\t\t{0}ms", watch.ElapsedMilliseconds);

watch.Restart();
var rules = new List<Rule> {
    new Rule(i => i % 3 == 0, "Fizz"),
    new Rule(i => i % 5 == 0, "Buzz"),
 };
var reusable = string.Join(";",
    from i in range
    select numberOrAllMatchingRules(i, rules));
watch.Stop();
Console.WriteLine("Elapsed reusable:\t\t{0}ms", watch.ElapsedMilliseconds);
Console.WriteLine($"Same results?\t\t\t{specialized == reusable}");


