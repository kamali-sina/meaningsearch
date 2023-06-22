# meaningsearch

A simple package that helps you find meaningful lines of any given input. Especially useful in CTFs.

## How to install

Simply run:

    cargo install meaningsearch

to install the package.

## How to use

You can use the command below to run meaning search. End your input with ctrl + D.

    meaningsearch <options>

You can also find meaningful lines in the output of a bin using :

    cat <somefile> | meaningsearch <options>

### --file

You can use the 'file' flag to meaning search a file:

    meaningseach --file <path to file>

### --threshhold

By default the meaning checker deems a line meaningful if it contains at 30% meaningful content. You can use the 'threshhold' flag to specify a different threshhold for your meaning checker. In the following example we specify a threshhold of 80%:

    cat <somefile> | meaningsearch -t 0.8
