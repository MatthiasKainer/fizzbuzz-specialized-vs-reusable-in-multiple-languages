# A fizzbuzz example: specialisation vs reusability

Code companion to the article on matthias-kainer.de

## Running examples

The `run` script will give you all possible commands. Example:

```
❯ ./run
usage: ./run <example>

  python: Runs the python example. Requires pipenv and pyenv or python 3.8 on your path for python
  rust: Runs the rust example. Requires rustc & cargo
  js: Runs the javascript example. Requires nvm or node > 16
  kotlin: Runs the kotlin example. Requires kotlinc-jvm 1.6.10 (JRE 17.0.2+8)
  all: Runs all examples

❯ ./run all

###########################
#         PYTHON          #
###########################

Elapsed specialized:            208.17ms
Elapsed reusable:               897.88ms
Same results?                   True

###########################
#        KOTLIN           #
###########################

Elapsed specialized:            136 ms
Elapsed reusable:               310 ms
Same results?                   true

###########################
#          JS             #
###########################

Same results?                    true
Elapsed specialized:            127.04168399982154ms
Elapsed reusable:               245.83684499934316ms

###########################
#        DOTNET           #
###########################

Elapsed specialized:            70ms
Elapsed reusable:               296ms
Same results?                   True

###########################
#         RUST            #
###########################

Elapsed specialized:            63.15ms
Elapsed reusable:               118.80ms
Same results?                   true
```

If you have all requirements install, you can run all examples in sequence with the `./run-all` script