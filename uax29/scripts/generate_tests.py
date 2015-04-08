#!/usr/bin/env python

import fileinput
import os
import re
import sys


def load_file(input_path):
    regex = re.compile('^[^#]+')
    tests = []
    for line in fileinput.input(input_path):
        m = regex.match(line)
        if m:
            fields = m.group(0).split()
            test = []
            word = ''
            for i in xrange((len(fields) - 1) / 2):
                break_opportunity = fields[i * 2] == u'\u00f7'.encode('utf-8')
                code_point = int(fields[i * 2 + 1], 16)
                if (0x20 <= code_point <= 0x7e and
                    code_point not in [ord('\\'), ord('"')]):
                    char = unichr(code_point)
                else:
                    char = '\\u{{{:x}}}'.format(code_point)
                if break_opportunity and i != 0:
                    test.append(word)
                    word = ''
                word += char
            if word:
                test.append(word)
            tests.append(test)
    return tests


def write_tests(tests, output_path):
    with open(output_path, 'w') as f:
        f.write('''use defaults;

''')
        for i, test in enumerate(tests):
            f.write('''#[test]
fn test_{:04}() {{
    let mut breaks = defaults::Breaks::new("{}",
        defaults::make_word_break_tree());
'''
                .format(i, ''.join(test)))
            for word in test:
                f.write('    assert_eq!(breaks.next(), Some("{}"));\n'
                    .format(word))
            f.write('''    assert_eq!(breaks.next(), None);
}

''')


def main():
    try:
        input_path = sys.argv[1]
        assert os.path.exists(input_path)
        output_path = sys.argv[2]
    except:
        sys.stderr.write('Usage: {} input output\n'.format(sys.argv[0]))
        exit(1)
    tests = load_file(input_path)
    write_tests(tests, output_path)


if __name__ == '__main__':
    main()
