---
layout: post
title: "Rust meets the web - a clash of programming paradigms"
author: "Jakob Meier"
categories: Blogging
tags: [wasm, rust, Paddlers, webdev]
image: 20/sky.jpg
image_tooltip: "The sky as seen from a immigrant. I took it when I moved to the UK."
# thumbnail_style: no-crop
lang: en
ref: nuts1
techs:
  js:
    title: "JavaScript"
    description: "JavaScript is the scripting language used on virtually all modern websites, as well as on web servers running NodeJS."
    url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript"
    img: js.svg
  rust:
    title: "Rust"
    description: "The Rust programming languages had its first stable release in May 2015. Although it is in its core a systems programming language, it has been adopted rapidly in different environments. Many programmers love using it and the community is growing quickly."
    url: "https://rust-lang.org/"
    img: rust.png
  wasm:
    title: "WebAssembly"
    url: "https://webassembly.org/"
    img: wasm.svg
    description: "WebAssembly is a new web standard (1.0 since October 2017) that allows running bytecode in the browser. While it is possible to compile C/C++ and other languages to Wasm, right now the tooling for Rust is definitely the most advanced. Partly because both the Rust and the Wasm project originated at Mozilla and are actively pushed by them."
---

<!-- TODO: Lighter tone or more concise or preferably both -->
<!-- TODO: More fore-shadowing / string it all together with a rotem faden -->

<p class="intro"></p>

Most code running on the web is event-based, garbage-collected, and dynamically typed.
In stark contrast, Rust is a compiled language with static type- and memory-safety without a garbage-collector.
What are the implications for a project that compiles Rust to WebAssembly?
I try to answer this question with a fictive story and hands-on code examples.
<!-- and I introduce a Rust library which helps to overcome some of the worst obstacles. -->

<!-- [Paddlers](https://github.com/jakmeier/paddlers-browser-game) -->

## Table of contents
{:.no_toc} 
* TOC
{:toc}

## Overview

Today, I tell a tale of a cultural clash.
Programming culture, that is.
The JavaScript culture on one end, and the Rust culture on the other.
<!-- But it is a culture nevertheless. -->
<!-- What do I mean by *programming culture", I hear you ask. -->
<!-- For me, this is a set of programming paradigms that are often used together. -->
<!-- As an example, take object-oriented programming -->
<!-- Some programmers are very attached to their programming language of choice. -->
<!-- Usually, because that language turned out to be very suitable for  -->

To lay down the necessary foundation, I start by talking about what makes the two ecosystems unique and different from each other.
This leads to the suggestion that it might be difficult to write idiomatic Rust code that runs in the browser.
I want to find out how viable it is and when it makes sense to use Rust in the browser.

<!-- I sketch a possible solution to the proposed dilemma with some Rust code. -->
<!-- Using these idea, I will alVery briefly, I will introduce a new publish-subscribe library called *Nuts*, which I have been working on in the last months. -->
<!-- A full coverage of the library will follow in the next blog post. -->

## The land of JavaScript
### How the Browser was Intended to be Used
<!-- I am definitely not an expert on the history of browsers, I was too young to program when browsers first saw the light of day.  -->
<!-- light of day? -->
<!-- But to make a point about the fundamentally different approach of JavaScript compared to Rust, I have to write a little bit about the fundamental design elements of JavaScript as the main programming language for browsers. -->

When browsers have first been created, people were very excited.
Text and images could be placed and styled with dozens of possibilities.
It is a second Gutenberg Revolution, everyone gets access to an infinite amount of information that was previously unreachable. What a great achievement!

Then, those elements became interactive with the addition of a scripting language that can run directly in the browser and modify what is displayed.
Suddenly, the browser looks less like a book on a screen and more like newspapers from Harry Potter.

<!-- Code running in the browser was added as an additional nice-to-have feature. -->
Fast-forward to 2020, browsers are the doorstep to so much more than just animated books.
Everyone uses them for everything.
Be it watching videos of cute cats or managing stock portfolios. It all happens in the browser nowadays.
JavaScript evolved to support all these different use cases.

<!-- To make the transition from 1995 to now, JavaScript had to address several major points.
Back then, programming complex code JavaScript was *cumbersome* and the results in the browser *slow* and *insecure*.
We are blessed that this topic received so much attention by browser developers that, by and large, the issues have been resolved. -->

### The JavaScript Core Features
The inhabitants of JavaScript land, let us call them *JavaScriptler* from now on, are very open people.
It is one of their greatest strengths that they cooperate with many other JavaScriptlers.
Communication between them has to be easy and without any road blockers.
This idea is at the heart of JavaScript's culture.

The downside is that they naively run anyone's code.
To avoid major damage being inflicted by an adversary JavaScriptler, a code running in JavaScript land had to be limited in what it can do.
We also say it executes in a sandbox within the browser.
While other languages like C or Python communicate directly with the host operating system (OS), JavaScript can only communicate with the browser.

<!-- As a result, modern websites are full of code and there are highly dynamic interactions between DOM elements. -->

<!-- To address the speed problems that come with such a sandbox, just-in-time (JIT) compilers [BOOKMARK TODO] put traditional compilers to shame with their crazy-efficient compilation times and remarkable optimizations at runtime. -->

A huge number of libraries and frameworks in JavaScript land have tried to make programming more accessible and simple.
Their combined power is really what defined modern life in JavaScript land.
Arguably, the sheer number of frameworks made it more complex than it has simplified thing.
But there is always just the basic JavaScript without frameworks as the baseline.
For today, I want to talk only about that fundament of JavaScript and its interactions with the browser.

Memory management is a big differentiator of programming languages.
Any language has to provide means to allocate and release memory but the approach can vary.
In JavaScript, it is all done silently in the background.
And sharing memory between functions and closures could not be simpler, it just works.
This fits very well in the general philosophy of JavaScript land, enabling easy communication.

Simple memory management is enabled by the garbage-collectors, which are a bunch of very busy inhabitants of the land.
They clean up all the leftover memory by fast-paced communication of busy programs that cannot be bothered to tidy up behind themselves.
<!-- Running in an execution environment provided by the browser allows to make design choices like using a garbage-collector. -->
<!-- Natively compiled languages, like Rust, basically have to cope with what ever the hardware and operating system provides them. -->
<!-- The designers of JavaScript have had a bit more freedom here. -->
<!-- Another important choice has been made when the JavaScript event loop has been defined. -->
<!-- -T-O-D-O-: Example of JS with easy data sharing, that will not go well with Rust. -->

This should give you an idea of how a JavaScriptler thinks.
But there is one more very important topic to cover, the *event loop*.
Let me explain what the event loop is in the next section, it will be important in the comparison to Rust.

### The JavaScript Event Loop
<!-- and how it Prevents Race Conditions -->
The land of JavaScript is ruled by a thing called [*the event loop*](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop), which schedules different tasks (threads) to run orderly.
It defines several rules that all code living in the browser has to obey.
For simple JavaScript programming, it is usually okay not to worry about them.
But then there are also quite basic cases in which it matters a lot.

Below is a JavaScript example with a promise, a timeout of 0 seconds, and some console logging.
If you can tell me with confidence in what order the output appears, you have studied the event loop laws well.

[JS Fiddle](https://jsfiddle.net/2oL4s7km/6/)
```js
// Create promise which resolves after a timeout of 0ms.
// This forces the promise to be enqueue as a new thread in 
// the event loop instead of executing immediately in this thread.
// The function within the timeout acts very much like
// a thread in other programming models.
let promise = new Promise((resolve, reject) =>
  setTimeout(
    () => {
      console.log("[A] Inside Promise");
      resolve("DONE");
    }),
  0
);

// Add another message after promise has been resolved.
promise.then(
  result => console.log("[B] Promise returned: ", result),
  error => console.log("[C] Promise failed: ", error)
);

// Write to the console when this code block finshes executing.
console.log("[D] End of code");

// SOLUTION
// Output order: D, A, B

```

If the output comes as a surprise to you, you should learn about rule number one of the event loop.

1. Once a thread is running, it runs to completion without interruptions of other threads.

In the example above, this means that even though the timeout has been set to be resolved immediately, it has to wait in the queue until the currently running thread is done.
If you have understood this one rule, you know enough about the JavaScript to follow the rest of the article. 

But why does this rule exist?
Would it not be more efficient to start executing the second thread of the example immediately?
Especially, considering that virtually all modern consumer devices have multiple cores which could work on the two threads in parallel.
I am glad you asked.

### Race conditions? Not with JavaScript!
When the result of some code depends on the order in which the threads access the same data, we call that a *race condition*.
Sometimes, this is intended and perfectly fine.
But in other cases, the programmer does not even know that there is a race condition, hence not all possible outcomes will be accounted for. In that case, race conditions are bad.

<!-- Complicated bugs may occur only in very specific timing conditions. -->
<!-- This is the kind of bug ticket your typical lazy programmer would just close as *not-reproducible*. -->
<!-- Until it happens when someone important watches, when it suddenly becomes the focus of the entire team for days if not weeks. -->

There are different solutions to avoid the risk of race conditions.
The founders of JavaScript thought about this carefully. 
They came to a drastic conclusion and decided that no concurrency between threads is allowed, for the safety of everybody.

This resolves the race conditions by removing the possibility that multiple threads ever run at the same time.
However, the existence of multiple threads should still be allowed, or otherwise, it would be very annoying to write code.
So they came up with the event loop, which sequentially runs one thread after another without interleaving them.

### Clicker Game Example in JavaScript

Here is some example code in JavaScript for a simple clicker game where a player collects apples.

<script async src="//jsfiddle.net/jakmeier/z6qmk7xw/13/embed/"></script>

The variables `apples` and `trees` in this example are allocated automatically and they are easily accessible from other functions and threads, just like JavaScriptlers are used to.
There is also no race condition here, thanks to the event loop.
Without it, (A) `apples -= 1;` within the `buy()` function would have a race condition with (B) `apples += trees;` in the closure given to `setInterval()`.

How is it a race condition?
Assume `apples` is 10 and `trees` is 5;
In the normally intended timing, after both statements execute, the result should be 14 apples and 6 trees.
But with one possible timing of a multi-core processor, A reads 10 and B reads 10,too before A has a chance to write.
Then A writes 9, which is immediately overwritten by B writing 15, so we end up with 15 apples and 6 trees. (Buying the tree was for free.)
This is possible because `+=` is not an atomic operation in hardware, it will be compiled into a read, add, and write operation executed sequentially.

Times have changed but the traditions of our ancestors have remained unchallenged within the browser. Most people in JavaScript have probably forgotten about the problem with race conditions because the problem has long been solved for them.
But there are other regions, outside the browser, which have found different solutions to race condition problem.

<!-- ### Asynchronous and Parallel Code in JavaScript
The demonstrated behavior in the previous example is a direct consequence of the concurrency model used in JavaScript.
It has proven itself to be very useful for event-based programming.
For example, when registering two event-listeners on HTML UI elements, the registering code will not be interrupted by the first event being fired. This avoids a lot of tricky cases which are easily forgotten by programmers.

```js
TODO: Example with jQuery initilization
button A: set text to AAA and remove listener on B
button B: set text to BBB and remove listener on A
```

The problem of the model is that multi-core machine (desktops, laptops, phones, ...) cannot easily use all their cores to parallelize the workload because all asynchronous tasks are executed in sequence.
It is of course possible to do parallel programming in JavaScript, but it is a bit more involved.

I think it is fair to say that JavaScript has not initially been designed with parallel programming in mind.
There was simply no reason for it just to manipulate a few hundred DOM nodes.
But the world has moved on from the 90s. Browsers are used for more than just displaying text and static images. Devices use many cores. 
Thus, it is time to enable easier parallel programming. -->

<!-- 
 - JS
    - Portable (Not compiled)
    - JIT for performance (JS magic)
    - Single-threaded with run-to-completion scheduling on the event loop(?)
 - DOM
    - Code mostly just manipulates DOM
 - Events
    - Closures everywhere
    - Many cross references for shared data
 - Garbage collection -->

## The land of Rust
<!-- ### What Rust was Designed for -->
<!-- 
 - Compiled to CPU native assembly
 - Lots of type information to support compile-time optimizations (Rust magic)
 - Memory safety without garbage collection
    - Compiler needs to know a lot about memory usage
    - Data sharing avoided as much as possible
    - single root of execution tree
 - Multi-threading built into the language/std -->
<!-- Ah, where should I even start with Rust. -->
<!-- I am definitely a bit of a fanboy for the language but I will try to stay factual. -->

![Image: A private road with a friendly sign.](/assets/img/20/private_drive.jpg)

A Rust citizen, also known as Rustacean, is very picky about its program code, as opposed to the openness found in JavaScriptlers.
All programs have to be scanned by the compiler and nothing is executed before all checks are done.

And rules to get in Rust land are very strict indeed.
Trying to smuggle through an ordinary `7` as an `f32`?
Nope, the `7` only qualifies for integers, you would have to use `7.0` instead.
That type of narrow thinking is very typical in Rust land.
<!-- Everything has to be compiled completely into native instructions.
compiles it code to native instructions  with a strong type system.
Zero cost abstractions allow for high-level concepts to be compiled into native binary code that requires no dedicated runtime environment. -->

Many of the early Rustaceans are refugees from C++, which is one of the countries impacted the hardest by race conditions.
It is therefore deeply engraved into Rustaceans that they want to prevent any future race condition disasters.
But they are used to build very performance-oriented stuff for a living.
Operating systems, numerical libraries, and world simulations are daily business for a Rustacean.

Naturally, they depend a lot on the benefits of multi-processors. They cannot do without.
The single-threaded approach as seen in JavaScript is not an option in Rust land.

### Lifetimes and MRSW
The founders of Rust are very smart people and they discovered a different approach to solve the issue of race conditions.
Instead of forbidding multiple-threads, they forbid sharing mutable data.

Immutable data can be shared, no problem.
Mutable data can also be passed from one point to the other.
But mutable data sharing from two different locations at the same time is strictly prohibited and will be prosecuted with hefty compiler errors.

This can be formulated as the number one rule of Rust.

1. Each variable can have either **m**ultiple **r**eaders or a **s**ingle **w**riter. (MRSW)

<!-- 
Instead of hiding the complexity of modern machines, Rust allows programmers to embrace it.
Programming for multiple cores is never trivial but Rust has all the necessary features included to make it as easy as possible, without giving up on correctness or speed. -->

<!-- ### Memory Management
Deceptively similar to JavaScript, memory management is implicit in the source code.
But the big difference is that there is no garbage-collector.
Instead, the correct time to (de-) allocate memory is derived by compiler.

This is awesome for performance but it also has drawbacks.
Most importantly, we have to make sure the compiler can derive the lifetime for every memory location used.
Otherwise, there would be a potential for use-after-free bugs, but luckily the compiler will complain and tell us if a lifetime is impossible to derive -->

<!-- ### Lifetimes in Rust
Race conditions are prohibited by the Rust compiler. 
But in contrast to Javascript, instead of an event loop, the issue is solved with the *multiple reader single writer (MRSW)* rule on each data location.
In Rust, this rule is enforced at compile-time.
Each object has a place in memory and an associated lifetime that starts at initialization of the object and ends when the object is deallocated. -->

<!-- The following code snippet shows how these rules work in practice.
Code that does not compile has been commented out with the error message prepended.

[Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2c7f8f0f95e31950d80efc399023ba3b)
```rust
struct MyStruct {
    num: u64,
}
fn main() {
    let mut b = MyStruct { num: 0 };
    let mut a = MyStruct { num: 0 };
    /* `a` is alive */
    
    /* Borrowing mutably twice in a row is okay */
    add_two(&mut a);
    add_two(&mut a);
    
    println!("{}", a.num);
    // Output: 4
    
    /* Borrowing twice at the same time is not okay */
    
    // error[E0499]: cannot borrow `a` as mutable more than once at a time
    // add_two_to_both(&mut a, &mut a);
    
    add_two_to_both(&mut a, &mut b);
    println!("{} {}", a.num, b.num);
    // Output: 6 2

    swallow(a);
    /* `a` has been moved, we cannot use it again. */
    
    /* Using stuff that has been passed on to another place is not okay. */
    /* For what we know, it could be used mutably by some other thread.  */
    
    // error[E0382]: borrow of moved value: `a`
    // add_two(&mut a);

    // error[E0382]: borrow of moved value: `a`
    // println!("{}", a.num);
}
// borrows mutably
fn add_two(a: &mut MyStruct) {
    a.num += 2;
}
// borrows mutably
fn add_two_to_both(a: &mut MyStruct, b: &mut MyStruct) {
    add_two(a);
    add_two(b);
}
// takes ownership
fn swallow(_a: MyStruct) {
    // ...
    // doesn't really matter what happens here
    // for example, the data could be sent to another thread
    // ...
}
``` -->
<!-- [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=80ee33e234578337e0d54081431a7c34) -->
<!-- [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ce3de9ed34f90991eec09bb2ce02f5be) -->

<!-- In this example, `a` is accessed mutably several times by different function calls to `add_two()`.
That is okay because the function only borrows the memory location.
When it returns, all references to `a` are gone as well.

But accessing it twice at the same time, or accessing it after moving it somewhere else is not okay.
It would otherwise violate the Rust rule number 1. -->

<!-- To visualize in my mind what is allowed and what not, I like to think of the execution tree rooted at `fn main()` and how all memory accesses fit in. -->

<!-- ### Borrowing Trees
Taking the example from before, all mutable accesses to the same data can be drawn in a tree when following this rule.
The root of the tree is the allocation of the memory.
Mutable borrows form the edges.
Accesses to the data are the leaves.

![Image: Graphic of code tree](/assets/img/20/nuts/exec-tree.svg)
*Tree of all mutable accesses to the variable `a`.*

This tree is walked in *depth-first* order during execution.
Or looking only at the leaves, simply from left to right.

Every memory access can always be traced back to the root where the memory is allocated.
In this example, the entire tree is known at compile-time.

More complex examples involving multiple threads and mutexes will also have an underlying borrowing-tree like this but the exact shape can be determined at compile time.
Eitherway, the existence of a such a tree is guaranteed by the Rust compiler.

Borrowing trees are really just a rephrasing of the MRSW rule. -->
<!-- I came up with it recently. -->
<!-- I do not know if it is mathematically equivalent, to be honest. -->
<!-- I will refer to it again later in this article when talking about event-based programming in Rust. -->

The MRSW rule helps a lot to prevent race conditions.
But it is also very restrictive to follow these rules all the time.
When multiple threads need to communicate, they have to move data between them and sometimes they would like to share mutable data.
But data movement between threads is seen as high risk and subject to strict border controls in Rust.

To alleviate this, Rustaceans can get exempts under certain conditions.
For example, they can apply for a `Mutex` or various atomic types when they are in great need.
These types are built directly into the standard library that almost all Rustaceans have agreed to follow.

<!-- These rules are hard to bring together with multi-core processing. -->

The [department of `Send` and `Sync`](https://doc.rust-lang.org/nomicon/send-and-sync.html) performs all the necessary checks automatically in the background, without many of the Rustaceans even noticing.
Usually, they only realize that there are checks when they have tried to smuggle a type across the thread-border that was not designed for it, such as a non-atomic reference counted pointer (`Rc`).
The compiler will then tell them that they have to follow thread-safety rules, int this case they should use an atomic reference counted pointer (`Arc`).
<!-- Essentially, they check that no data is shared between threads that does not follow all the necessary rules. -->

Let me show you an example of these rules in practice.
The following code snippet takes a range of numbers and adds them up.
To avoid integer overflows for large inputs, I also added a modulo operation in each step.

```rust
fn modulo_add_range(a: i64, b: i64, c: i64) -> i64 {
    (a..b).fold(0, |a, b| (a + b) % c)
}
```
The syntax in this Rust code is a bit different from JavaScript. `(a..b)` produces a sequence of integers between `a` and `b`. `|args| body` is the closure syntax of Rust, equivalent to `(args) => body` in JavaScript.
Thus, the fold just adds all integers between `a` and `b` modulo `c`.

This can be evoked from a single thread like so:
```rust
let a = 1;
let b = 100_000_000;
let c = 7;
let result = modulo_add_range(a,b,c);
println!("Result is {}", result);
```

But a real Rustacean would not settle for a single-threaded solution.
We can easily divide the work between several threads. 
I am testing with a Ryzen 5 3600, thus I will use 12 threads.

```rust
// Start 12 threads and each gets equal workload of size `step`
let threads = 12;
let step = (b - a) / threads;

// All threads will add their result to this collector.
let mut result: i64 = 0;

// Need to store handlers in a vector to wait for threads to finish.
let mut handles = vec![];
for i in 0..threads {
    let handle = std::thread::spawn(|| {
        // Find start and end for this thread
        let sub_a = a + i * step;
        let sub_b = if i < threads - 1 { sub_a + step } else { b };
        // Reuse function from single-threaded example
        result += modulo_add_range(sub_a, sub_b, c);
    });
    handles.push(handle);
}

// Wait for all threads
for h in handles {
    h.join().expect("Deadlock?");
}

// A final modulo is necessary because we did not do them in the last steps.
result = result % c;
println!("Result is {}", result);
```

Looks good? Not to the Rust compiler. It vigorously throws an error at us.

```
error[E0499]: cannot borrow `result` as mutable more than once at a time
```

The `result` variable is mutable, thus it cannot be shared across threads.
Comparing to JavaScript, this is a problem because we do not have the event loop.
It is a race condition very similar to the previous example with apples and trees.
In languages like C++, this is allowed but it is inherently a race condition.

To solve this, we can use an `AtomicI64`.
This type only requires read access and yet it can update the number safely, with some predefined atomic operations such as `fetch_add()`.
Because this operation is atomic in hardware, it cannot create a race condition and hence the Rust compiler is okay with it.

Having applied this fix, we try again to get past the grumpy compiler staff.
Sure enough, a new wave of complaints rains down on us.

```
error[E0373]: closure may outlive the current function, but it borrows `result`, which is owned by the current function
error[E0373]: closure may outlive the current function, but it borrows `i`, which is owned by the current function
error[E0373]: closure may outlive the current function, but it borrows `a`, which is owned by the current function
error[E0373]: closure may outlive the current function, but it borrows `step`, which is owned by the current function
error[E0373]: closure may outlive the current function, but it borrows `threads`, which is owned by the current function
error[E0373]: closure may outlive the current function, but it borrows `b`, which is owned by the current function
error[E0373]: closure may outlive the current function, but it borrows `c`, which is owned by the current function
error: aborting due to 7 previous errors
```

Lifetimes are not respected for the variables `result`, `i`, `a`, `step`, `threads`, `b`, and `c`!

The problem is that we tried to move a reference to `result` (and other variables) from our initial thread to 12 new ones.
The variables are all locally allocated on the stack and the compiler is a bit worried that they might not be alive for long enough.

In JavaScript, all variables live forever and the garbage collector will take care of cleaning up.
But in Rust, we have to tell upfront how long the variable should live, so that the compiler can plan the clean up for us.
And when we move the references in 12 threads, the compiler loses track and cannot determine a suitable lifetime anymore.

Actually, in this example, we can see that `result` will have to live exactly until the `println!()` statement.
But the compiler is unfortunately not able to derive that in this case.
To make the compiler feel better about it, we can tell him that we want it to be a `static` variable (instead of local).
The compiler will then allocate it outside of the stack.

What about the other variables?
We could make them all `static`.
But an easier solution is to move a copy of each variable inside the closures that start the new threads.
We just have to add the `move` keyword at the start of the closure definition and the compiler will know what to do.

Again we ask the compiler to ratify our code.
This time, we are finally lucky, no problems are spotted.
Here is the final code:

```rust
    let threads = 12;
    let step = (b - a) / threads;

    // All threads will add their result to this atomic collector
    static ATOMIC_RESULT: AtomicI64 = AtomicI64::new(0);
    
    let mut handles = vec![];
    for i in 0..threads {
        let handle = std::thread::spawn(move || {
            let sub_a = a + i * step;
            let sub_b = if i < threads - 1 { sub_a + step } else { b };
            // Reuse function from single-threaded example
            let partial_result = modulo_add_range(sub_a, sub_b, c);
            // Once per thread, use more expensive atomic add (without modulo)
            (ATOMIC_RESULT).fetch_add(partial_result, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    // Wait for all threads
    for h in handles {
        h.join().expect("Deadlock?");
    }

    // A final modulo is necessary because we did not do them in the last steps.
    let result = ATOMIC_RESULT.load(Ordering::Relaxed) % c;
    println!("Result is {}", result);
```

This example shows the way of life for a Rustacean.
They do not complain about it, mind you.
Sure, we had to go through a bit of a hassle to make it work.
But the Rust compiler awards us a valuable certificate for the absence of race conditions in this code.

It is quite impressive how the Rust compiler manages to keep everything safe in this way.
But I think we have enough background and it is finally time to talk about the alliance of Rust and JavaScript.
<!-- 
Anything coming from outside of Rust is marked as `unsafe`. An unfortunate fate for foreigners because Rustaceans are not exactly known for their tolerance in that regard. ([See the actix disaster from 2019](https://www.theregister.com/2020/01/21/rust_actix_web_framework_maintainer_quits/)) -->

<!-- But for JavaScript and Rust, it is a whole different story. The live on completely different soil. Rustaceans need nothing more but bare-metal processors whereas JavaScript citizens need a well-prepared atmosphere as their execution environment. -->

<!-- Okay, but why am I writing all of this? Nobody with a sane mind would ever suggest to mix JavaScript and Rust, right? -->

## Rust on the Web
![Image: A fence resembling the prison that Rustaceans have to live inside when visiting the browser.](/assets/img/20/fence.jpg)
As the world becomes more progressive and global, the cultures of JavaScript and other countries have met each other and learned from one another.
In the early 2010s, mad scientists conducted experiments to see what a unified world would look like, with projects like [Native Client](https://developer.chrome.com/native-client) and [asm.js](http://asmjs.org/).

And then, in 2015, the [WebAssembly (WASM)](https://webassembly.org/) movement started. 
Its goal until this day is to bring natively-compiled languages right into the land of JavaScript.
And an unlikely partner has declared itself to be the primary partner of JavaScript: Rust.

<!-- Today, Rust code can be compiled to WASM and executed within browsers.
(Other languages also work but I will focus on Rust only.)
So  -->

As Rustaceans find themselves inside of JavaScript land, they feel quite comfortable right from the back.
The environment has been adopted to look just like the typical stack-machine that Rustaceans are so used to with unmanaged linear memory.
And that is pretty much all they need to get started.

The JavaScriptlers look at the Rustaceans and they are delighted by the look of these strange visitors who arrived.
Of course, they must be kept inside a safety chamber and not be released to the rest of JavaScript for everyone's safety.
Inside that box, they have got a big array of memory that looks like it is unmanaged to the Rustaceans, but really it is still protected by the browser.

### Integer Micro Benchmarks
The JavaScriptlers heard that Rustaceans are good at math with large numbers.
So they wanted to make a competition JavaScriptlers against Rustaceans.
The Rust code they used is the function `modulo_add_range` from the previous example.
In JavaScript, the code that does the same looks like this:

```js
function modulo_add_range_js(a, b, c) {
    let acc = 0;
    for (let i = a; i < b; i++) {
        acc = (acc + i) % c;
    }
    return acc;
}
```
Both teams will get the number `a,b,c` at runtime, to avoid compiler optimizations.
Then, the time is measured it takes each time to come up with the final answer.

The Rustaceans, being very performance-oriented, tried to use their multi-threaded code. But oh dear, it failed!
Even though it passed all Rust compiler checks, the browser does not like the calls to `std::thread::spawn()`.

```
panicked at 'failed to spawn thread:
Custom { kind: Other, error: "operation not supported on wasm yet" }'
```
Right, threads are not the same in JavaScript land.
Rustaceans are not allowed to use them here.
It is at this moment when they realize what prison they find themselves inside.

Astonished by this, they did not know better than to use the single-threaded implementation instead.
So it is really just this one-liner from before.

```rust
#[wasm_bindgen]
pub fn modulo_add_range_wasm(a: i64, b: i64, c: i64) -> i64 {
    (a..b).fold(0, |a, b| (a + b) % c)
}
```

Compiled to WASM, this uses `i64.add` for adding and `i64.rem_s` for the module.
Both should be very efficient on a 64-bit machine like the one I am using for benchmarking.
I bet the JavaScriptler team will have no chance even on a single thread!

First round, `a = 1, b = 100'000'000, c=7`.

![Data plot](/assets/img/20/plots/small_number.svg)

What a surprising result!
The Rustaceans are only marginally faster than JavaScriptlers, far below a per cent difference.
Not even native Rust shows a meaningful difference in performance.
It looks like the JIT compiler of JavaScript has no problems generating very efficient machine code for adding many small numbers.
Only the multi-threaded Rust version is significantly better, as expected. (Mind the log-log scale, the difference is a factor of about 6 but it can look smaller on this graph.)

What about adding larger numbers?
The next round is with `a = u32Max - hundredMillion, b = u32Max, c = 7`, where `u32max = 2^(32) -1 = 4'294'967'295`.
This number is too high to fit in an `i32` (32-bit signed integer), hence it affects the possible machine operations the JIT compiler can use.
Will this shift of the input range be enough to bring down the JIT compiler?

![Data plot](/assets/img/20/plots/medium_number.svg)

Now there is a substantial difference between JavaScript and WASM, while the WASM and native AMD64 implementation still show comparable results.

As a final test, they wanted to test with even larger numbers.
JavaScript normal numbers are only accurate up to `jsMax = Number.MAX_SAFE_INTEGER = 2^(53) - 1`.
For larger integers, the results might be inaccurate.
To have guaranteed correct results, the type `BigInt` has to be used explicitly in JavaScript, while Rust is just fine with an `i64` with numbers up to `2^(63)-1`.
<!-- So they tried how Rustaceans cope with that. -->

With `a = jsMax - hundredMillion, b = jsMax, c = 7`, the input values are still within the safe range of JS numbers. But the intermediate results get slightly above that line.
Therefore, the run can be timed with `BigInt` and without, but keep in mind that the latter result will get wrong results.

![Data plot](/assets/img/20/plots/large_number.svg)

See there, if JS uses `BigInt`, it slows down even more, getting close to 100 seconds to compute what Rust does in 8.53 seconds in a single thread or 1.46 seconds in 12 threads.

Great, so we conclude that WASM is just superior to JavaScript, right?
Then let us go and replace all JavaScript with Rust immediately!

<!-- For example, Rustaceans are much better at adding a lot of integers -->
<!-- But they also wonder why Rustacean threads are so afraid of touching each other. -->

<!-- Before jumping to conclusion, the JavaScriptlers decide to let the Rustaceans do some of the work that is very common for JavaScript. Just some easy interactions between UI elements in the browser. -->

<!-- The Rustaceans have no idea about the event loop and its rules, they still follow their multiple-reader single-writer rule, which makes them move a bit awkward at times. -->

### Clicker Game Example in WASM
Remember the clicker game from earlier, implemented in JavaScript?
We shall transform it to Rust here and now.

The HTML can be reused entirely.

```html
<h1>
  Awesome Clicker Game
</h1>
<main>
  <!-- Rust will insert dynamic text here-->
  <div class="button" onclick="buy()">
    Plant tree
  </div>
</main>
```
To replace the JavaScript code, we call into the browser API directly from Rust and manipulate the DOM in this way.
To start, here is an initialization function in Rust.
It sets up the state previously set up by JavaScript in the global scope since Rust does not allow to run code in the global scope.

```rust
pub fn init() {
    let apples = 1;
    let trees = 0;

    // window and document have to be fetched from JS world
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let main = document
        .get_elements_by_tag_name("main")
        .item(0)
        .unwrap();

    let dynamicText = document
        .create_element("p")
        .unwrap();
    
    main.prepend_with_node_1(&dynamic_text).unwrap();

    update_text();
}
```

Puh, that is a lot of verbose stuff. 
For example `unwrap()`, which takes the inner value of a `Result` or `Option` and panics if it is not present.
Well, since we are now working with Rustaceans, we have to deal with that.
The unwraps here explicitly show the possibility to crash, where previously, in JS, it was not visible but still possible.

The code style conventions have also changed to snake_case for functions but otherwise, it is mostly the same so far.
Next, we need the `update_text()` function.
Maybe something like that:

```rust
pub fn update_text() {
    dynamicText.innerText = format!("You have {} apples and {} trees.", apples, trees);
}
```

Hm, but `dynamicText`, `apples`, and `trees` are not accessible.
A different solution is required.

Can we make all variables global?
Rust requires all globals to be initialized from the start, hence before `init()` gets called.
That is possible for primitives like `apples` and `trees`. But for `dynamicText` it is not possible.

This could be solved by initializing with `None` and overwriting with `Some(...)` in `init()`.
`None` is the closest equivalent to a JavaScript `null`, since Rust generally does not feature null pointers.
This leads to code that is a bit more blown up to work with the types wrapped in an `Option`.

```rust
static mut DYNAMIC_TEXT: Option<Element> = None;
static mut APPLES: i32 = 1;
static mut TREES: i32 = 0;

pub fn update_text() {
    if let Some(dynamic_text) = DYNAMIC_TEXT.as_mut() {
        dynamic_text.set_inner_html(
            &format!("You have {} apples and {} trees.", APPLES, TREES)
        );
    }
}
```

But there is a problem.

```
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |     if let Some(dynamicText) = DYNAMIC_TEXT.as_mut() {
   |                                ^^^^^^^^^^^^ use of mutable static
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |             &format!("You have {} apples and {} trees.", APPLES, TREES)
   |                                                          ^^^^^^ use of mutable static
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |             &format!("You have {} apples and {} trees.", APPLES, TREES)
   |                                                                  ^^^^^ use of mutable static
```

So there it is again, the moody compiler.
Mutating a global variable is `unsafe`, it grumbles.
Well, we have to admit that it clearly violates the MRSW rule, as multiple threads could write this at the same time.

We can actually just mark the code as `unsafe` and the compiler will be ok with it.
Of course, that means no thread-safety certificate but in the browser with the event loop, such certificates do not mean much.

```rust
pub unsafe fn update_text() {
//  ^^^^^^
```
Rustaceans will probably get angry with us if we go that way but it is definitely possible to do.
There are other solutions available that would be more respectful towards Rustacean culture. 
Atomics can be used for the numbers and for the `dynamicText` we could use a `thread_local!` + `RefCell`.
The full code examples are also available with a completely [safe variant](https://github.com/jakmeier/www.jakobmeier.ch/blob/e842dfe84b89b15833de7c02c8c6c0acc05c0602/appendix/wasm-tests/clicker-game/src/lib.rs#L78) in the [appendix](https://github.com/jakmeier/www.jakobmeier.ch/tree/gh-pages/appendix/wasm-tests).
For brevity's sake, we will go and with the unsafe version.

In the mind of a JavaScriptler, it is completely ridiculous to say global mutable variables are unsafe.
After all, everything in the browser is controlled by the event loop.

<!-- And the code is only meant for the browser. -->
<!-- Thus, for the sake of this story, we will go with this simple solution and just put that `unsafe` wherever necessary. -->
<!-- To preserve peace, we search for a more elegant solution.
We can avoid the `unsafe` by using a `Mutex` to guarantee the variable is only accessed once at the time.
For the `apples` and `trees`, we can try to use `AtomicI64` again and all should be fine, right?
yeah, probably. but  -->

In this spirit, the `buy()` function is easy to implement.

```rust
pub unsafe fn buy() {
    if APPLES > 0 {
        TREES += 1;
        APPLES -= 1;
        update_text();
    }
}
```

Finally, the interval to increase the number of apples periodically.
The closure setup needs a bit of weird syntax to work right now.
But trust me, it does exactly what the JavaScript code also did.

```rust
fn collect_apples() {
    unsafe {
        APPLES += TREES;
        update_text();
    }
}

fn set_timer() {
    let window = web_sys::window().unwrap();
    
    // Prepare closure for access by JS
    let boxed_function = Box::new(collect_apples);
    let closure = Closure::wrap(boxed_function as Box<dyn Fn()>);

    // setInterval() (Rust has no overloading, it must have a different
    //                name for every possible set of parameters)
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            5000,
        )
        .unwrap();

    // Leak memory on purpose to ensure the unchecked ref is always valid
    closure.forget();
}
```

The last bit, the `forget()`, tells the compiler to not do its clean up magic for this closure.
It is a little bit like declaring something static but it looks even uglier.
And it serves well as a round-up of this genuinely disastrous insult of Rustacean culture.

Unfortunately, I cannot show the WASM it in a JS Fiddle but there is no visible difference to the JS Fiddle.
You can download the code from the [appendix](https://github.com/jakmeier/www.jakobmeier.ch/tree/gh-pages/appendix/wasm-tests) if you want to play around with it.

The final code works, it does everything it needs.
But it looks terribly verbose from a JavaScriptler's perspective and Rustaceans will be offended if you show this to them.
In the end, everyone is disappointed.
<!-- The safe Rust version would be better on the Rust side but even worse for JavaScriptler to look at. -->

## Conclusion
The Rustaceans have done everything they have been asked by the JavaScriptlers.
They had to go out of their way and settle with single-threaded execution.
But that is alright, they are ready to adopt the local ways in JavaScript land.

The JavaScriptlers have been disillusioned about the magical speed of WASM and they find it a bit odd to watch how Rustaceans follow nonsensical rules in their world. But it turns out, they can live together and sometimes the Rustaceans can do a job better than any JavaScriptler.

Shall we count this cultural nearing a success?
I would say yes, it is an amazing first step to have compiled languages run in all major browsers.
But the road still has too many bumps that need fixing.

If we want Rust on the web to be a success, it has to be much more approachable.
What we need is library support.
I am not even talking about fully-fledged frameworks but rather simple helper utilities that solve the worst pain points.

JavaScript was not built in one day. And just like JavaScript evolved to fit the browser, Rust will need to grow, too.
I am optimistic that we, the communities of Rust and JavaScript, will come up with great solutions.
Hopefully, Rustaceans will soon feel comfortable in the browser.

Thanks for reading all the way down here!
If you have experience with WASM yourself that you would like to share, please do so on Reddit or get in touch directly in an [email](mailto:inbox@jakobmeier.ch).
I am especially interested in hearing about the pain points of other people and potential solutions.
Are there already great libraries around? Have you thought about libraries you want to create, or are have you already created them?
Let me know, I would love to have a discussion.

Discussions on [/r/javascript](https://www.reddit.com/r/javascript/comments/j4ou0g/rust_meets_javascript_a_clash_of_programming/) and
[/r/rust](https://www.reddit.com/r/rust/comments/j4ot6b/rust_meets_the_web_a_clash_of_programming/).

## Epilogue

Ruth is a young inhabitant of the browser with Rustacean parents.
She has visited Rust land a couple of times and was amazed by the multi-threaded power.
Ever since she experienced that freedom, she feels caged in JavaScript land.
But their parents found a job in JavaScript land and that is where they plan to stay for the foreseeable future.

The young child understands both cultures a fair bit.
But she cannot understand the political tension between the two that makes everything seem so hard in her life.
In her mind, it should all be very simple.
She dreams of a future that makes the life for Rustaceans simple and yet gives them the freedoms they seek.

*The example below is a completely functional Rust code. The libraries behind it are [stdweb](https://github.com/koute/stdweb) and [nuts](https://github.com/jakmeier/nuts). The latter is a project of mine in an early stage. Most importantly, no unsafe code is hidden inside of it and yet the API is as simple as presented here.*

<!-- This is a functional example and it is great that it is possible to run Rust inside the browser.
But, I tried to simplify as much as I could and still it feels very impractical.
Production ready code would be even worse.
If we want Rust on the web to be a success, it has to be much more approachable.
Should we give up Rust on the web then? -->
<!-- 
No. Rust is versatile and it can all be done.
However, the level of expertise required is tremendously high.
We need library support to make this easier. -->

<!-- I believe there will be Rust libraries that can hide most of the complications I presented here.
And I do not think *one size fits all* will work well here.
There are already many helpful libraries but we have to implement many more. -->

<!-- Currently, I am working on [Nuts](https://github.com/jakmeier/nuts), which attempts to combine core principles of JS (easy state sharing) and Rust (compile-time type checks).
But the details of that belong into another post. -->


[Source on github](https://github.com/jakmeier/nuts/blob/ea2bf1ac6909d9b28497ee923489be1ec329739c/examples/clicker-game/src/lib.rs#L12)
```rust
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen]
pub fn init() {
    let apples = 1;
    let trees = 0;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let main = document.get_elements_by_tag_name("main").item(0).unwrap();
    let dynamic_text: Element = document.create_element("p").unwrap();
    main.prepend_with_node_1(&dynamic_text).unwrap();
    let game_state = GameState {
        dynamic_text,
        apples,
        trees,
    };

    let game = nuts::new_activity(game_state);

    game.subscribe(GameState::update_text);
    game.subscribe(GameState::buy);
    game.subscribe(GameState::collect_apples);
    nuts::publish(UpdateTextEvent);

    set_timer();
}

struct GameState {
    dynamic_text: Element,
    apples: i32,
    trees: i32,
}

struct UpdateTextEvent;
struct BuyEvent;
struct CollectEvent;

#[wasm_bindgen]
pub fn buy() {
    nuts::publish(BuyEvent);
}

impl GameState {
    fn update_text(&mut self, _: &UpdateTextEvent) {
        self.dynamic_text.set_inner_html(&format!(
            "You have {} apples and {} trees.",
            self.apples, self.trees
        ));
    }
    fn buy(&mut self, _: &BuyEvent) {
        if self.apples > 0 {
            self.trees += 1;
            self.apples -= 1;
            nuts::publish(UpdateTextEvent);
        }
    }
    fn collect_apples(&mut self, _: &CollectEvent) {
        self.apples += self.trees;
        nuts::publish(UpdateTextEvent);
    }
}

use stdweb::js;
fn set_timer() {
    js! {
        setInterval(
            @{||nuts::publish(CollectEvent)},
            5000
        )
    }
}
```