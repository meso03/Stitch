import json, stitch_py as sp
from stitch_py import StitchHandle

args = (
    '--file "C:\\Users\\mesom\\OneDrive\\backup\\Documents\\UROP\\synthestitch\\data\\origami\\bigram_test.json" '
    '--domain list '
    '--model bigram '
    '--bigrams-path "C:\\Users\\mesom\\OneDrive\\backup\\Documents\\UROP\\synthestitch\\data\\origami\\sample_probabilities.json" '
    # '--verbose-eval '
    '--threads 8'
)

def sum_list_py(args):
    (vec,) = args      # one arg: list[int]
    return sum(vec)

def mul_py(args):
    x, y = args
    return x * y

def add_py(args):
    x, y = args
    return x + y

# Create a handle (this builds the native Simple DSL inside)
handle = StitchHandle()

# Register Python-backed primitives (early-capture into THIS handle’s DSL)
handle.register("sum_py", "list int -> int", sum_list_py)
handle.register("mul_py", "int -> int -> int", mul_py)
handle.register("add_py", "int -> int -> int", add_py)

# IMPORTANT: run using the handle’s DSL (so your registrations are visible)
print(handle.run_cli(args))