# something

A functional programming language that.

The goal of Something is to be as simple as possible. You could almost say the python of functional staticly typed languages.

```rust
greeting:str: "hello world";
print! greeting;
```

The first thing you'll probably notice is the weird variable syntax. In Something variables and functions are interchangable a variable is just syntax sugar for a function. This isbecomes more prevalent with the function definition syntax.

```rust
add x i32, y i32:i32
    print! (x+y);
end
```

As you can see very similar syntax. The language is designed to make similar ideas look similar in syntax.

There are no formal loops like `for`, `while`, or `loop` like in other languages. Before you get scared away this is fairly common in functional languages. Instead we use recursionand branching. Here is the classic while true loop in Something.

```rust
thankYou name str:i32
    print! "Thank you", name, "\n";

    thankYou! name;
end
```

This will eventually seg fault and exit, but you get the point loops can easily be made using funcitons.
