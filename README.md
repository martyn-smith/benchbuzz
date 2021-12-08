Some Fizzbuzz Language Benchmarks
===

Background
---

[High throughput fizzbuzz is a thing](https://codegolf.stackexchange.com/questions/215216/high-throughput-fizz-buzz/), and has been making the rounds on the usual suspects lately. As has, perhaps, a renewed focus on [high performance generally](https://www.youtube.com/watch?v=Ge3aKEmZcqY). The highly unofficial record for fizzbuzz stands at 55 GiB/s on a ~5 GHz machine, or... ten bytes per clock cycle. Wow. 

There is, for context, an argument *against* high performance computing: said solution also, apparently, took several months (and doesn't run on ARM, or quite possibly any non-Linux box). I don't mean to be condescending or sour grapes here - `ais523s-temporary-account` has done some astonishing work. They clearly care more about high-performance computing than I. 

So, that's today's question: how fast can I, an enterprise coder who expects to bang out a reasonably portable fizzbuzz client over a morning coffee, make fizzbuzz? More importantly, what language is best?

While fizzbuzz (especially the paper variant) is *debatable* as an interviewing technique, I quite like it as a tyre-kicking for a new language. You've got almost all the common basics - output, conditionals, string formatting - just error handling, compound data types and network I/O missing really. Okay, maybe that's quite a lot, but you need to learn the former first so at the very least fizzbuzz is a good reference. It also gives you a good feel for how opinionated the language is on things like mutability, functional vs imperative, etc.

Also, I don't have spare iOS, macOS, windows and Android clients and SDKs lying around.

Method
---

Algorithm for all is, initially at least, the same: start with an empty string (or list), append fizz or buzz as appropriate, then the number if the string is empty. (I do NOT assume the arguments are coprime, so the normal '15' trick is not applied). This is *not*, as we shall see, the most performant, but it *is* more adaptable if the client decides they don't want to use a pair of primes any more. Enterprise, remember. I've made small exceptions to layout for what I'd consider idiomatic in each language (e.g. no one in their right mind malloc()'s in a loop in C). I've also made no attempt at multithreading - assuming you have to print numbers in order and with correct padding, I'm not sure it would help much.

I have, however, at least attempted some easy wins. Examples:

* running with JIT
* calling `.lock()` on STDOUT
* removing allocation in hot loops

All measured by running `./fizzbuzz (or $RUNTIME fizzbuzz) | pv > /dev/null`. 

Results recorded over ten seconds after a ten second burn-in period, and all implementations are available in the appropriate $LANG directories.

Results
---

| Language      | Notes               | Speed (MiB/s) |
|---------------|---------------------|---------------|
| C (GCC) 11.2  | -O3                 | 120           |
|               | +no-malloc          | 160           |
| CPython 3.9   |                     | 12            |
| PyPy 7.3.5    |                     | 70            |
| Ruby 2.7      |                     | 12            |
|               | JIT                 | 12            |
| Jula 1.7      |                     | 0.045         |
| Rust 1.55     | -O                  | 20            |
|               | + lock,             |               |
|               |   writeln!,         |               |
|               |   better formatting | 100           |
|               | + pre-allocation    | 150           |

Conclusions
---

A Naive implementation in an interpreted languages will probably get you around 12 MiB/s. Less than a morning's worth of work and a language change will get you 10x that, but then, unless you're writing your entire system in the more performant language, you'd still be dealing with problems enterprise devs hate - managing interop between different languages. Depends on your shop.

Julia is *really* slow, and this implementation also locks out my terminal - no idea.

It shouldn't be surprising that, in a test like this, the syscalls to stdout will matter more than the integer arithmetic. Even so, I'm surprised how little Python's infamously slow integer arithmetic matters. Although PyPy does speed things up considerably.

Naive Rust isn't *much* faster than interpreted languages, demonstrating that fearless and *fast* concurrency is not as simple as picking a language. Possible explanations:

* Locking stdout (a concurrency feature)
* Making a syscall at every '\n' (although the C implementation does that as well)
* Allocating memory for a new string for each iteration

Although not shown here, locking stdout on its own didn't seem to change the initial result. Writing to buffers clearly did, and it's interesting that, for a usize, `x.to_string()` is noticeably faster than `format!("{}", x)` (a 3GiB/s version Rust implementation is out there - and uses some special library for even faster formatting). Pre-allocation gets you a faster client than naive *C*, presumably because of the different string types (C strings are null-terminated, so appending is O(n). Rust Strings are not). Pre-allocation is, however, not very functional - if you're reading the code with pre-allocation, you've got this String being declared that, initially at least, *has no purpose*. Whether that's nicer for your purposes is up to you.

Further work
---

Expand to more languages: Bash, Ruby 3.0, JS, Go, gfortran.
Investigate Julia's weirdness.
Clearly, this needs a blockchain implementation too.
