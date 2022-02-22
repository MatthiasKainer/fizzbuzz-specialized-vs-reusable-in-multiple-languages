const { PerformanceObserver, performance } = require('perf_hooks');

const obs = new PerformanceObserver((items) => {
    items.getEntries().forEach(({ name, duration }) => {
        console.log(`${name}${duration}ms`);
    })
    performance.clearMarks();
});
obs.observe({ type: 'measure' });

function fizzbuzz(i) {
    if (i % 3 == 0 && i % 5 == 0) return "FizzBuzz"
    else if (i % 3 == 0) return "Fizz"
    else if (i % 5 == 0) return "Buzz"
    else return i
}

function numberOrAllMatchingRules(i, rules) {
    const result = rules
        .map((rule) => rule.matches(i) ? rule.keyword : "")
        .join("")
    return result !== "" ? result : i
}

const numbers = Array.from({ length: 1000000 }, (_, i) => i + 1)
performance.mark("Specialized Start");
const specialized = numbers.map(fizzbuzz).join(";")
performance.mark("Specialized End");
performance.measure("Elapsed specialized:\t\t", "Specialized Start", "Specialized End")
performance.mark("Reusable Start");
const rule = (matches, keyword) => ({ matches, keyword })
const rules = [rule(i => i % 3 == 0, "Fizz"), rule(i => i % 5 == 0, "Buzz")]
const reusable = numbers.map((i) => numberOrAllMatchingRules(i, rules)).join(";")
performance.mark("Reusable End");
performance.measure("Elapsed reusable:\t\t", "Reusable Start", "Reusable End")
console.log("Same results?\t\t\t", specialized === reusable)
