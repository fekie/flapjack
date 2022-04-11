# flapjack
> Keep a record of your transactions in multiple accounts.

# What is it?
Flapjack is a command line program that helps you keep track of money in multiple accounts. It features a log-based database so changes can be edited by hand if needed.

# Usage
```
git clone https://github.com/Chloe-Woahie/flapjack.git
cargo run --release
```

# Program Examples
```
------------------------------------
Options: Set[0] Increment[1] Decrement[2] Create[3] Destroy[4] View[5] Exit[6]
>>> 3
------------------------------------
Wallet Name:
>>> Paypal
The wallet will be named Paypal. Confirm? (y/n)
>>> y
Created wallet: Paypal
------------------------------------
```

```
------------------------------------
Options: Set[0] Increment[1] Decrement[2] Create[3] Destroy[4] View[5] Exit[6]
>>> 2
------------------------------------
Decrement amount for which wallet?: Savings (Bank)[0] Checking (Bank)[1] Cash[2] BACK[3]
>>> 1
Decrement wallet amount by: 
>>> 60
Enter comment: 
>>> New pokemon game just dropped
Wallet: Checking (Bank)
Amount: 60
Comment: "New pokemon game just dropped"
Is this correct? (y/n)
>>> y
Decremented wallet Checking (Bank)'s amount by 60
------------------------------------
```

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

# Log Example
```
# the program will register this line a comment
CREATE "Checking (Bank)"
CREATE "Savings (Bank)"
INCREMENT "Checking (Bank)" 50 "this is a comment for this transactions"
INCREMENT "Savings (Bank)" 40
INCREMENT "Checking (Bank)" 25.50 "this is another comment for the transaction"
SET "Savings (Bank)" 200 
DECREMENT "Checking (Bank)" 20.5
```

# Personal Notes
This program works great with an ssh program like Terminus on your phone, so it is always accessable. Personally, it also helps me to spend less money when I have to deduct the money from my accounts by hand.

# License
MIT License
