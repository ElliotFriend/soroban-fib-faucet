# Soroban Fibonacci Faucet <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [Motivation](#motivation)
- [How it Works](#how-it-works)
  - [Basic Overview](#basic-overview)
  - [Initialization](#initialization)
- [Remaining Questions and Considerations](#remaining-questions-and-considerations)
- [TODO](#todo)

## Motivation

You know the "Fibonacci Sequence" right? I'm not getting all weird here. It's
not magical, or supernatural, or anything like that. I just like using it as a
practice problem when I'm trying to learn a programming language, or some other
technology. Anyway, the sequence goes like this:

```text
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

That's just the first 10 digits, but you probably get the idea. Each number is
derived by adding the previous two numbers. If you're curious, here's a small
python script I wrote to give you `n` digits of the sequence.

```python
#!/usr/bin/env python3
import sys
"""Returns `n` Digits of the Fibonacci Sequence.

Run this from your shell like `./fibonacci.py 25`

"""

# The sequence always starts with [0, 1], so we'll start there
f_seq = [ 0, 1 ]

while len(f_seq) <= int(sys.argv[1]):
    f_seq.append(f_seq[-1] + f_seq[-2])

print(f_seq)
```

> I'm sure it's bad, gimme a break. I wrote it (*\*checks notes\**) ALMOST 10
> YEARS AGO!!!??? Oof, I'm old.

## How it Works

Anyway, enough with the math lesson. Let's move on to what I have here. But first, a brief message from our legal department:

> *Disclaimer*: This *Fibonacci Faucet* is a unique, interesting, and
> terrible-in-practice scheme for distributing a smart asset on Soroban. This is
> (hopefully obviously) not meant to be developed or deployed in the real world,
> but rather is a thought experiment. It's meant as a tangible, "graspable"
> concept to use as I familiarize myself with Soroban and Rust. Don't do this in
> real life. No, seriously. Just. Don't.

### Basic Overview

So, the basic flow of what happens in this (once the contract has been
initialized, more on that later):

1. A user invokes the `signup` function to add their address to the list of
   members.
2. The contract will check a few things:
   1. the FibFaucet is open,
   2. there is room on the list,
   3. the invoking `Address` is not already a member,
   4. the invoking `Address` is not the `Admin` user, and
   5. the `signup` function was invoked by an `Account` rather than a
      `Contract`, sorry no cross-contract calls for this function.
3. If all that checks out, the `Address` of the invoker will be added to the `Signups` vec, as well as stored in a `Member(Address)` data key with the calculated payout.
4. When the `Admin` user chooses to do so, they can invoke the `disburse` function to iterate through the `Signups` vec, and mint the required number of tokens to each member.

### Initialization

As long as the contract has not already been initialized, any `Account` invoker
(again, no cross-contract calls in this part) can invoke the `init` function
which then deploys a token contract, sets the `Admin` user to be the invoker,
and sets up some `CONTRACT_DATA` placeholders.

The FibFaucet contract is set as the `Admin` for this token, allowing us to
`mint` those tokens by using simple `invoker()` authentication. The `token`
contract ID is salted using the first 11 digits of the Fibonacci Sequence, as
`Bytes`.

## Remaining Questions and Considerations

Here's a grab bag of other things that are running through my mind as I put this
together.

- Ownership in Rust continues to confuse me. Have I used the `&` borrower thing
  in the right place(s)? Have I used the `clone()` method inefficiently
  (probably)? Are there any general rules/conventions to guide the use of those
  kinds of things (again, probably)?
- Literally, this is just giving tokens to anybody who asks for them.
  Potentially lots of them, too. It would be trivial for someone to use a
  *second* account to game the system and get more of the token. In this
  experiment, I don't really care about that, tbh.
- As for performance, is there any benefit/detriment to abstracting so many
  `data().set/get()` calls into different functions like I've done here?
- I'd be interested to run some experiments on pricing and such for how this
  contract runs/lives on-chain. Of course, that kind of stuff is still being
  determined, but still could be interesting.

## TODO

As far as I'm concerned, this is still a bit of a work-in-progress. Some ideas
I'm considering for implementation are included here:

- [ ] Some kind of "token admin" function where the `admin` user can submit
  functions and arguments that will then be passed on to the token contract so
  it can be interfaced with and managed like any other Soroban token.
- [ ] MOAR TESTS!! Or, rather, ANY TESTS!! Also, I want to figure out the
  difference between `test.rs` and `testutils.rs`, since I see those two files a
  lot in these projects.
- [ ] Improved returns on successful invokation. Something more meaningful than
  `null` would be probably be helpful to a user.
- [ ] Some way to display the chain (in order) of everyone who's been a part of
  the sequence.
  - Maybe it would be interesteing to provide the transaction hash or something
    from when they signed up and/or when they were paid.
- [ ] An "automatic" mode. The user signs up and is paid right then. This could
  be turned on/off by the admin account at `init` time, or `reset` time, or any
  time I s'pose.
- [ ] I thought about some kind of "shuffle" system. Maybe the signups are
  locked in when someone invokes the `signup` function, but when the `disburse`
  function is invoked by an admin, the signups are shuffled and the payments are
  decided then?