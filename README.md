# flapjack
> Keep a record of your transactions in multiple accounts.

# What is it?
Flapjack is a command line program that helps you keep track of money in multiple accounts. It features a log-based database so changes can be edited by hand if needed.

# Examples
```
------------------------------------
Options: Set[0] Increment[1] Decrement[2] Create[3] Destroy[4] View[5] Exit[6]
>>> 5
------------------------------------
+-----------------+--------+
| Wallet          | Amount |
+-----------------+--------+
| Cash            | 58     |
+-----------------+--------+
| Checking (Bank) | 255    |
+-----------------+--------+
| Savings (Bank)  | 720    |
+-----------------+--------+
| Total           | 1033   |
+-----------------+--------+
------------------------------------
```

```
Options: Set[0] Increment[1] Decrement[2] Create[3] Destroy[4] View[5] Exit[6]
>>> 2
------------------------------------
Decrement amount for which wallet?: Savings (Bank)[0] Cash[1] Checking (Bank)[2] BACK[3]
>>> 2
Decrement wallet amount by:
>>> 60
Enter comment:
>>> New pokemon game just dropped
The wallet Checking (Bank)'s amount will be decremented by 60 with the comment as "New pokemon game just dropped". Confirm? (y/n)
>>> y
Decremented wallet Checking (Bank)'s amount by 60
------------------------------------
```

# Personal Notes
This program works great with an ssh program like Terminus on your phone, so it is always accessable. Personally, it also helps me to spend less money when I have to deduct the money from my accounts by hand.

# License
MIT License
