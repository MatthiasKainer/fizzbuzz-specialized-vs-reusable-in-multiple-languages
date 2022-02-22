import kotlin.system.measureTimeMillis

fun fizzbuzz(i: Int) =
        if (i % 3 == 0 && i % 5 == 0) "FizzBuzz"
        else if (i % 3 == 0) "Fizz" 
        else if (i % 5 == 0) "Buzz" 
        else i.toString()

data class Rule(val matches: (i: Int) -> Boolean, val keyword: String)

fun numberOrAllMatchingRules(i: Int, rules: List<Rule>): String {
    val result = rules
        .map { (matches, keyword) -> if (matches(i)) keyword else "" }
        .joinToString("")
    return if (result != "") result else i.toString()
}

fun main() {
    val numbers = (1 until 1000000)
    val resultSpecialized: String
    val timeInMillisSpecialized = measureTimeMillis { resultSpecialized = numbers.map { fizzbuzz(it) }.joinToString(";") }
    println("Elapsed specialized:\t\t$timeInMillisSpecialized ms")

    val resultReusable: String
    val timeInMillisReusable = measureTimeMillis { 
        val rules = listOf(
            Rule({i -> i % 3 == 0}, "Fizz"),
            Rule({i -> i % 5 == 0}, "Buzz"),
        )
        resultReusable = numbers.map { numberOrAllMatchingRules(it, rules) }.joinToString(";") 
    }
    println("Elapsed reusable:\t\t$timeInMillisReusable ms")

    println("Same results?\t\t\t${resultSpecialized == resultReusable}")
}
