import re


def most_vowels(tuple_of_strings: tuple) -> int:
    """
    Find the word with the most vowels in it.

    :param tuple_of_strings: a tuple of strings
    :precondition: tuple_of_strings must contain only strings
    :postcondition: retuns the number of vowels in the string with the biggest number of vowels
    :return: number of vowels as integer

    >>> most_vowels(())
    0
    >>> most_vowels(('a213', 'eve', 'applesauce'))
    5
    """
    if len(tuple_of_strings) <= 0:
        return 0
    if len(tuple_of_strings) > 1:
        if most_vowels(tuple_of_strings[1::]) > len(re.findall("[aeiouAEIOU]+?", tuple_of_strings[0])):
            return len(re.findall("[aeiouAEIOU]+?", tuple_of_strings[-1]))
    else:
        return len(re.findall("[aeiouAEIOU]?", tuple_of_strings[0]))


def main():
    most_vowels(('a', 'eve', 'applesauce'))


if __name__ == '__main__':
    main()
