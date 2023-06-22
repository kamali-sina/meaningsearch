# meaningsearch

A simple package that helps you find meaningful lines of any given input. Especially useful in CTFs.

Meaning search supports Leet and can detect even obfuscated meaningful texts.

## How to install

Simply run:

    cargo install meaningsearch

to install the package.

## How to use

You can use the command below to run a meaning search. End your input with ctrl + D.

    meaningsearch <options>

You can also find meaningful lines in the output of a bin using:

    cat <somefile> | meaningsearch <options>

### --file

You can use the 'file' flag to meaning search a file:

    meaningsearch --file <path to file>

### --threshhold

By default, the meaning checker deems a line meaningful if it contains 30% meaningful content. You can use the 'threshold' flag to specify a different threshold for your meaning checker. In the following example, we specify a threshold of 80%:

    cat <somefile> | meaningsearch -t 0.8

## Example

Let's assume we have the following text file called test.txt:

    asdasd a dasd ajnafan oas
    n00bz d0n'7 w1n!
    this is a normal line.
    da kadma akj ad

Running the following command on the said text file results in:

    $ cat test.txt | meaningsearch -t 0.5
    n00bz d0n'7 w1n!
    this is a normal line.
