from __future__ import absolute_import, print_function

import re
import os
import sys
import subprocess

#print(os.getcwd())
#os.chdir(sys.path[0])
#print(os.getcwd())
os.chdir(os.path.join(sys.path[0], os.pardir))
# print('posix' in sys.modules)
# print('ntpath' in sys.modules)
# print( sys.prefix)
# print(sys.modules['os'])

#print(os.getcwd())
#print("ssssssssssssssss")
#print(sys.path[0])
#print(os.path.join(sys.path[0], os.pardir, 'tests', 'headers'))
#
#Checking for #include directives of system headers...
# tests/headers/complex.h:1:#include <sksdk>
print('Checking for #include directives of system headers...')
include_found = False
headers_dir = os.path.join(sys.path[0], os.pardir, 'tests', 'headers')
for f in os.listdir(headers_dir):
    h_file = os.path.join(headers_dir, f)
    with open(h_file, 'r') as content:
        for idx, line in enumerate(content.readlines()):
            match = re.search(r'#include\s*<.*>', line)
            if match:
                rel_path = os.path.relpath(h_file, os.getcwd())
                print('{}:{}: {}'.format(rel_path, idx + 1, match.group()))
                include_found = True

if include_found:
    msg = """
    Found a test with an #include directive of a system header file!

    There is no guarantee that the system running the tests has the header
    file, let alone the same version of it that you have. Any test with such an
    include directive won't reliably produce the consistent bindings across systems.
    """.replace('    ', '')
    sys.exit(msg)
else:
    print('Found none; OK!')

#steps
cwd_path = os.getcwd()
with open(os.path.join(sys.path[0], 'test.commands'), 'r') as steps:
    for line in steps.readlines():
        line = line.strip()
        if line.strip().startswith('#') or not line.strip():
            continue

        if line.strip().startswith('cd '):
            cwd_path = os.path.normpath(os.path.join(cwd_path, line.split(' ')[1]))

        if '%no_diff_assert%' in line:
            no_diff_assert()

        env_var = re.findall(r'env\(\w+\)', line)
        if env_var:
            for env in env_var:
                # print(env[4:-1])
                # print(os.environ.get(env[4:-1], ''))
                line = line.replace(env, os.environ.get(env[4:-1], ''))

        print(line)
        print(cwd_path)
        subprocess.Popen(line, shell=True, cwd=cwd_path)


def no_diff_assert():
    # git add -u
    # git diff @
    # git diff-index --quiet HEAD
    subprocess.check_call('git add -u', shell=True)
    subprocess.check_call('git diff @', shell=True)
    subprocess.check_call('git diff-index --quiet HEAD', shell=True)
