---
layout: post
title: "Untapped potential in Rust's type system"
author: "Jakob Meier"
categories: Blogging
tags: [rust,nuts,dynamic-typing]
image: 21/sky_is_the_limit.jpg
image_tooltip: "The sky is the limit!"
# thumbnail_style: no-crop
lang: en
ref: untapped-rust
techs:
---

<p class="intro">
Today, I'm writing about what types can be used for other than checking code properties.
It will involve a good chunk of dynamic typing, and yes it's in Rust.
There are some wild ideas in it, so fasten your seatbelt and get ready for a ride!
</p>

# Overview
The article is divided into introduction, background, three sections containing the main content, and a conclusion.
The three sections in the middle each cover their own idea with a separate motivation.
What connects is the way runtime type evaluation is applied. In that aspect, they build on top of each other.

{:.no_toc}
* TOC
{:toc}


# Introduction

Types are a very abstract concept. What even are they? 
For me, the answer depends quite a bit on the programming language and the general context of the discussion.

When I wrote my very first lines of program code, in C++, a type was for me just the thing to define a variable.
As I got more practice, with C++ and Java, types in my mind became essentially equivalent to classes or primitives.
But I didn't think much about types anyway. They were just a necessity to make the compiler happy.

Expanding to JavaScript, I realized that types can also be hidden in the background.
In that case, they must be right to make the runtime happy, which seemed to be more forgiving than the compiler.
On the other hand, I hated it when errors only appeared at runtime that I knew a compiler could tell me before.

Then, I learned Haskell. Types became a completely different concept.
It seemed like entire programs could be written in the type system itself.
I was impressed.

After all of that, I learned Rust.
I loved how strongly typed everything felt with Rust.
Comparing to C and C++, Rust removed the most frustrating parts from them.
Forgetting to initialize variabels was no longer possible, null pointers ceased to exist, and memory management became a blast.

Fast-forward to today. Rust showed me several completely new concepts that can be achieved with its clever type system.
Lifetimes incorporate the memory management aspect inside the type.
The distinction between `&mut` and `&` types defines if aliasing is allowed.
And in a way, types implementing the `Future` trait describe an entire finite state machine.

But today I want to talk about runtime type evaluation in Rust.
I've come across some practical programming problems that I wasn't able to solve without some (safe) downcasts here and there.
I then took it to extreme levels of dynamic typing that I didn't expect to be possible.
Along the way, I had to reconsider once again what a type actually is.
And since I found the results quite interesting and surprising, I wanted to share it in this article.


# Background: Dynamic Types in Rust

In some languages, the type of every (non-primitive) value is embedded in machine code.
It's like a hidden field implicitly present in every object.
This is one way to enable dynamic typing.
But Rust does not include type information overhead with every value.

However, Rust offers ways to manually store type information which can be used also at runtime.
It's possible to transform a value of statically known type into a fat pointer that combines the value with a virtual function table (vtable) for one trait.
These fat pointers are called [trait objects][docs-trait-object].

Trait objects essentially provide opt-in runtime type information.
But their power is fairly limited as they only gives access to the functions of a specific trait and its parents traits.
To know if we are dealing with a specific type, one more trick is required.

Using only tools [from the core standard library][any-docs], we can ask the compiler for a `TypeId` of any type and store this for our own use at runtime.
The compiler will then put a unique constant number there for the type ID.

Here is how type IDs are created. ([Run it on the playground!][type-id-example-0])

```rust
use core::any::{Any, TypeId};
fn main() {
    let one_hundred = 100u32;
    // Get the type ID usaing a value of that type.
    let t0 = one_hundred.type_id();
    // Get the type ID directly
    let t1 = TypeId::of::<u32>();
    assert_eq!(t0, t1)
}
```

There are two variants shown, one with a value of a type and the other with just the type as a generic parameter.
Both are function calls on the source code level. But the compiler should optimize them away and put a constant value in their place. 

The `TypeId` value can then be used, at runtime, for essentially just three things.
We can compare it to another `TypeId`, it can be used as a hash key, and we can print the ID for debugging purposes, which just shows a random-looking integer value.
But we cannot do other things, such as looking up if a trait is implemented for that type ID.


Here is how type IDs could be used to emulate a dynamic type check. ([Run it on the playground!][type-id-example-1])
```rust
fn count_rectangles(shapes: &[Box<dyn Shape>]) -> usize {
    let mut n = 0;
    for shape in shapes {
        // Need to derefernce once or we will get the type of the Box!
        let type_of_shape = shape.deref().type_id();
        if type_of_shape == TypeId::of::<Rectangle>() {
            n += 1;
        } else {
            println!("{:?} is not a Rectangle!", type_of_shape);
        }
    }
    n
}
```

The method `type_id()` is defined on the `Any` trait, which has a blanket implementation for, unsurprisingly, any type. (There is a small restriction on the type but this is beyond the scope of this article.)

The real dynamic typing starts when we use a trait object of `dyn Any`.
It can perform what's called a checked downcast, going from a general type to a more specific type. (See [`downcast_ref`][downcast_ref] and [`downcast`][downcast] for the official docs.)

Here is a usage example. ([Run it on the playground!][downcast-example])
```rust
fn remove_first_rectangle(shapes: &mut Vec<Box<dyn Any>>)
    -> Option<Box<Rectangle>>
{
    let idx = shapes
        .iter()
        .position(|shape| shape.deref().type_id() == TypeId::of::<Rectangle>())?;
    let rectangle_as_unknown_shape = shapes.remove(idx);
    rectangle_as_unknown_shape.downcast().ok()
}
```

A downcast here is no magic, though.
If we wanted to manually implement it (without help of the compiler) we could also check if the type ID matches our expectation and then follow up with a [transmute] call.

But enough background for now. Let's get creative with these concepts in the three following sections!

# Section 1: A Heterogenous Collection of Singletons
![Image: A couple of balls in different sizes.](/assets/img/21/hetero.jpg)

This section shows how magic like this can work in Rust and why it matters.
```rust
// Putting two different types in the same collection, with no keys.
collection.set( 3.14 );
collection.set( 888 );

// Taking out the values of the two types again, 
// automatically getting the value of the correct type
assert_eq!( 3.14, *collection.get::<f32>() );
assert_eq!(  888, *collection.get::<u32>() );
```


## Storing heterogenous data
Most collections in Rust are homogenous, that is, they store objects which are all of the same type. For example, a `Vec<f32>` only stores floats.
But we can make it one-way heterogenous by using pointers to trait objects.

For example, `Vec<Box<dyn ToString>>` stores a collection of pointers. The pointer types that can be accepted by this vector includes `Box<f32>`, `Box<u64>`, and many other types.
Thus, the data types we can put in are heterogenous. But what we get out is just a pointer to a trait object (`Box<dyn ToString>`) and the actual type of the inner value cannot be recovered.

To have a fully heterogenous collection, the getter-method should be able to return objects of different types.
This is trivially possible in dynamically typed languages, such as Python or JavaScript.
In statically typed languages, however, a function can only return one specific type, as defined by the function signature.

As an easy way out, languages with subtyping often have a most general type, which is a super type of all others.
For example, `Object` in Java is a super type of all classes.
This can be used in the function signature to define the return type.
The caller can then perform a downcast on the returned value.

In Rust, a trait object of type `dyn Any` can be considered the most general type.
It is the only type which (almost) all other types can be coerced into.
And as explained in the background section, `Any` is also the (only) trait that allows downcasting.
Thus, we can return `&Box<dyn Any>` in the getter method and the caller can downcast.

Returning `Box<dyn Any>` directly is not a nice interface, though.
To avoid manual downcasting on the caller side, it can be hidden behind a generic function.
Here is a complete example of that. ([Playground link][playground-good-interface])

```rust
use core::any::*;
use std::collections::HashMap;

fn main() {
    let mut collection = HeteroCollection::default();
    collection.set("f32", 3.14f32);
    collection.set("f64", 2.71f64);
    collection.set("another f32", 1.618f32);
    
    let f32_output = *collection.get::<f32>("f32").unwrap();
    assert_eq!( 3.14, f32_output);
}

#[derive(Default)]
struct HeteroCollection {
    data: HashMap<&'static str, Box<dyn Any>>,
}
impl HeteroCollection {
    fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
        let unknown_output: &Box<dyn Any> = self.data.get(key)?;
        unknown_output.downcast_ref()
    }
    fn set<T: 'static>(&mut self, key: &'static str, value: T) {
        self.data.insert(key, Box::new(value));
    }
}
```


The code above essentially emulates a Python dictionary.
Any key can hold any type.
The caller must make sure that key and type match.

Here is a crazy idea, how about we let the compiler do that check?
Below is an implementation which does just that. ([Playground link with example usage][singleton-collection-playground])

```rust
use core::any::*;
use std::collections::HashMap;

struct SingletonCollection {
    data: HashMap<TypeId, Box<dyn Any>>,
}
impl SingletonCollection {
    pub fn get<T: Any>(&self) -> &T {
        self.data[&TypeId::of::<T>()]
            .downcast_ref()
            .as_ref()
            .unwrap()
    }
    pub fn set<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }
}
```

With this approach, the generic type acts as the key.
Consequently, this limits the collection to a single element per type.
But in many cases, this is not a limitation.
New types are cheap!
As demonstrated in the snippet below comparing before and after.

```rust

/// Before
collection.set("name", "Jakob");
collection.set("language", "Rust");
collection.set("dominant hand", DominantHand::Right);

let name = collection.get::<&'static str>("name");
let language = collection.get::<&'static str>("language");
let dominant_hand = collection.get::<DominantHand>("dominant hand");

// After
collection.set(Name("Jakob"));
collection.set(Language("Rust"));
collection.set(DominantHand::Right);

let name = collection.get::<Name>().0;
let language = collection.get::<Language>().0;
let dominant_hand = collection.get::<DominantHand>();

// For completeness: Type Definitions
struct Name(&'static str);
struct Language(&'static str);
enum DominantHand {
    Left,
    Right,
    Both,
    Neither,
    Unknown,
    Other,
}
```

The only functional difference is that the type-key must be known at compile-time, whereas the string could be determined at runtime.
That's okay for now. LAter in section three I'll show an approach to get around this limitation.

Syntactically, there is a bit of an annoyance as a new type has to be defined for every key.
But personally, I think it's not worse than maintaining a list of "magic strings". They would probably end up as separate constants anyway, which is also one line of boilerplate code.

The benefit of the type-key is the compiler can check that key is valid and that the stored value matches the requested type.


## Real world applications and AnyMap

It's time to ask, when would we want to use a singleton heterogenous collection?
Perhaps the most common usage is in a library that wants to manage general state defined by the library user.

In that case, this pattern comes in handy because it allows for the user to store arbitrarily many objects of any type. And the library can manage them without even knowing the types.
Section 2 will have some good examples for this, too.

Worth noting, however, I didn't invent this pattern. It is in fact widely used.
I think I saw it the first time in [Amethyst/Shred][shred] in their [`struct World`][shred-world].

Digging deeper while writing this article, I found that [Chris Morgan][chris-morgan] has wrapped this pattern in a general-purpose collection [`AnyMap`][anymap].
At the time of writing the crate has over 1.3 million all-time downloads.
I would say that classifies as widely used.

So, types can be used as keys and the community is doing that already.
To uncover untapped potential, let's have a look at opportunities beyond that in the next section.

<!-- ## Section 2: Types as Channels and Events -->
# Section 2: Type-Oriented Message Passing
![Image: A rainbow in the sky.](/assets/img/21/rainbow.jpg)

In this section, we'll see some dynamic dispatch based on types. Not dynamic dispatch based on names and types combined, no, dispatch based on types only.
Additionaly, even the objects will be dynamically looked up by their type, which means the caller does not even need access to the object!

What I'm going to show you could be described as *object-oriented message passing* with the twist that types are used as object addresses and also for dynamic dispatch.

But let me be very clear about the terminology here. 
I'm referring to the general idea of [object oriented programming (OOP)][oop-wikipedia], which does not necessitate classes.
It's just objects and methods that I'm using.

Furthermore, [message passing][message-passing-wikipedia] in this context is a specific term for invoking a method on an object.
Essentially, a message with the identifier of the method and the argument values are sent to an object, which the object dispatches internally and executes.

This process can be implemented in Rust and dynamic types come in very handy.

## What I want to achieve and why
Last year, I wrote about a problem I was facing with Rust running in the browser through WASM. (See [*Rust meets the web - a clash of programming paradigms*][wasm-article])

To make a long story short, it boils down to threads not running continuously in the browser. Instead, closures have to be registered to be called in an interval.
Sharing data between those can get hairy, which I described in that article.

Below is an artificial example that illustrates how code for the browser may use callback closures.
```rust
fn main() {
    let window = get_window_from_browser();
    let body = get_body_from_browser();
    let state = MyDummyState::new();

    window.set_interval(
        100,
        move || {
            // do something every 100ms
            state.update();
        }
    );
    body.on_click(
        move |x,y| {
            // do something on every click
            state.apply_click(x,y);
        }
    );
}
```

This example doesn't compile. (Even under the assumption that all the functions exist with correct signature.)
The problem is that `state` is moved inside two closures, which doesn't work. 
Borrowing instead of moving wouldn't work either, since the closures outlive the current stack frame used by the main function.

To resolve this, I would have to put the data behind a shared smart pointer, like `Arc<>` and then introduce inner mutability.
That's annoying and I'd like to have a better way.

Back when I wrote the article complaining about these problems, I didn't really solve the issue, I just pointed it out.
But by now, I think I've got a satisfying solution that I've been using for many months.

As hinted earlier, the solution I eventually came up with involves a global storage of singleton objects with dynamic method registration and dynamic dispatch on those methods.
Let me just show you some code, hopefully it will make things a bit clearer.

```rust
struct MyObject {
    counter: u32,
}
struct MethodA;
struct MethodBWithArguments {
    text: String,
}
impl MyObject {
    fn method_a(&mut self, _arg: MethodA) {
        self.counter += 1;
        println!(
            "Object invoked a method {} times. This time without an argument.",
            self.counter
        );
    }
    fn method_b(&mut self, arg: MethodBWithArguments) {
        self.counter += 1;
        println!(
            "Object invoked a method {} times. This time with argument: {}",
            self.counter, arg.text
        );
    }
}

fn main() {
    /* registration */
    let obj = MyObject { counter: 0 };
    my_library::register_object(obj);
    my_library::register_method(MyObject::method_a);
    my_library::register_method(MyObject::method_b);

    /* invocations */
    my_library::invoke::<MyObject, _>(MethodA);
    my_library::invoke::<MyObject, _>(MethodBWithArguments {
        text: "Hello World!".to_owned(),
    });

    /* Output */
    // Object invoked a method 1 times. This time without an argument.
    // Object invoked a method 2 times. This time with argument: Hello World!
}
```

What's happening here is that I register an object (`obj`) and its methods to a globally managed state of `my_library`.
After that, I'm invoking methods on that object without actually referencing `obj`.
This is possible since `my_library` has it stored globally.

The global storage keeps only one object of each type. (It uses a heterogenous singleton collection internally.)
Therefore, the object that should be called is known as long as the type is specified.


This becomes very useful when working with closures as callbacks.
We could now have many different callbacks that all invoke methods on a shared object, without actually worrying about the data sharing part.
```rust
fn main() {
    // ...

    div.on_click(
        || {
            my_library::invoke::<MyObject>(MethodBWithArguments{
                test: "Clicked something!".to_owned(),
            }
        }
    );
}
```

So, I've implemented this (and more) in a library called [Nuts][nuts].
The naming is a bit different in the actual library. Objects are called activities, for instance. This is simply because I didn't think of it as objects and methods until I started the third attempt of writing this article.

So much about what the goal is. Now the challenge is how to implement the functions of `my_library`.

## Implementation

To implement the interface I just introduced, we need a bunch of global state sitting hidden in the background to store objects and methods.
Let's not worry about how the global state is stored and picked up. To stay focused on dynamic typing, we just assume the methods `register_object` and `register_method` are called on a `Nut` object.
The [Playground][playground-simplified-nuts] includes the glue to make it work if you want to run it yourself.

Under that assumption, what should go inside `Nut`?
Let's start with a collection to store objects.

```rust
pub struct Nut {
    objects: HashMap<TypeId, Box<dyn Any>>,
}
```
This is exactly what I showed you in section 1 as `SingletonCollection`.
A collection that can hold different objects, indexed by their type.

With this state, we can already implement `register_object`.

```rust
impl Nut {
    fn register_object<OBJECT>(&mut self, obj: OBJECT)
    where
        OBJECT: Any,
    {
        let key = TypeId::of::<OBJECT>();
        let boxed_obj = Box::new(obj);
        self.objects.insert(key, boxed_obj);
    }
}
```

It gets tricker with the methods.
We need to store an arbitrary number of methods with heterogenous types.
To store them in a single collection, we need to find a general trait object that covers them all.

`Box<dyn Any>` would work to store them.
But we'll need to call the methods later.
This will require a downcast to the actual type.

To be honest, it *could* probably be done this way.
But we can make our life much easier if we store callable function pointers instead.
We just need to find a general-enough callable type.

First, we have to pick one of the traits `Fn`, `FnOnce` and `FnMut` as our base trait.
`FnMut` is the most general of them, we shall go with that to not limit the user.
(You can read up on the differences between them in the documentation of [FnMut][docs-fnmut] and on what they are exactly in the Rustonomicon chapter about [Higher-Rank Trait Bounds][docs-higher-level-trait-bound])

Next, what are the arguments?
Each method will have a mutably borrowed object as a first argument (`&mut self`), and some argument struct as the second.
So, we could try something like `FnMut(&mut dyn Any, dyn Any)`.

But passing trait objects by value like this doesn't work, since `dyn Any` is of unknown size.
At least for the second argument, we need to wrap it in a box.
And since we store boxes of our objects anyway, let's also wrap the first argument.
Which puts us at `FnMut(&mut Box<dyn Any>, Box<dyn Any>)`.

Finally, this has to go inside a hash map.
The hash map value is a trait object of the trait `FnMut`, so it has to be wrapped into yet another Box.

The hash map key should be a combination of two types, `(TypeId,TypeId)`.
The first type ID is for the object and the second for the method argument.
This allows to store many methods for each object. And a lookup is still only one hash.

Putting it all together, the `Nut` struct looks like this.
```rust
pub struct Nut {
    objects: HashMap<TypeId, Box<dyn Any>>,
    methods: HashMap<(TypeId, TypeId), Box<dyn FnMut(&mut Box<dyn Any>, Box<dyn Any>)>>,
}
```

Wow, that's a type definition to scare off any readers.
But please bare with me!
The nice thing is that calling these methods is now just a matter of three simple steps.
1. Look up the object.
2. Look up the method.
3. Call the method with the object and the invocation argument.

Or, wrapped in code, it looks like this.

```rust
pub fn invoke<OBJECT, ARGUMENT>(&mut self, arg: ARGUMENT)
where
    OBJECT: Any,
    ARGUMENT: Any,
{
    let object_key = TypeId::of::<OBJECT>();
    let method_key = (TypeId::of::<OBJECT>(), TypeId::of::<ARGUMENT>());
    if let Some(obj) = self.objects.get_mut(&object_key) {
        if let Some(method) = self.methods.get_mut(&method_key) {
            method(obj, Box::new(arg));
        }
    }
}
```

The method key is constructed as a tuple of the types for the object and the argument.
Then, we check if the object and the method are present and simply call it if both lookups were successful.

In this process, the argument type is essentially used to perform a dynamic dispatch, since it selects the method on the object.

Next, the hardest part, putting the methods inside the collection.
The compiler doesn't just auto-convert a method to its counter-part using trait objects instead of specific types. No coercion going on here, unfortunately.

We need a wrapper closure around the registered method.
By moving the downcasting code inside the wrapper closure, this new closure can be of the more general type we derived earlier, while the inner methods preserves the original signature.
Here is the code for that.

```rust
pub fn register_method<OBJECT, ARGUMENT, FUNCTION>(&mut self, mut method: FUNCTION)
where
    FUNCTION: FnMut(&mut OBJECT, ARGUMENT) + 'static,
    ARGUMENT: Any,
    OBJECT: Any,
{
    let key = (TypeId::of::<OBJECT>(), TypeId::of::<ARGUMENT>());
    let wrapped_method =
        Box::new(move |any_obj: &mut Box<dyn Any>, any_args: Box<dyn Any>| {
            let obj: &mut OBJECT = any_obj.downcast_mut().expect("Type conversion failed");
            let args: ARGUMENT = *any_args.downcast().expect("Type conversion failed");
            method(obj, args)
        });
    self.methods.insert(key, wrapped_method);
}
```

Plenty to unpack here. Let's start with the function signature.

The exact generic type constraints are quite interesting here.
We have three type arguments to describe the permitted functions and closures we accept.
Here we don't have any boxes on the interface, as this is not something we want the user to be bothered with.

If you wonder what the `'static` is for in the trait bound, this a necessary constraint on the lifetime of the type of the function.
If you have never seen such a bound, don't worry, it's not important and I'd rather have your attention on the broader concept that I'm showing you here.
(But if you must know, feel free to check out the error message on the [Playground][playground-simplified-nuts] if you remove the bound and follow the trail from there. 🙂️)

Moving on to the body.
The key is constructed as before in `invoke`.
Then the key and the method are moved inside a boxed closure.

The closure has once again a signature and a body.
The signature has to match exactly with the definition used for the `methods` field in `Nut`.
Therefore, it includes the boxes and only has trait objects rather than specific types.

In the closure body, we just perform two downcasts and call the provided method.
Note that the downcasting here should never fail, since the invoke method looks up the methods by their type and thus always provides `Any` trait objects of the correct underlying type.

Puh, you've made it!
Combining all the snippets, the `my_library` interface from earlier is backed 100%.
Here is a [Playground link][playground-simplified-nuts] if you want to see it in action.

With this approach, I was able to embrace the event-driven browser world that uses callbacks everywhere.
Any registered object is always accessible from anywhere, including from inside closures.

## More about Nuts
The library [Nuts][nuts] I mentioned earlier, covers more than just the case I've shown you so far.
The concept can be taken further to have a complete publish-subscribe library.
This allows to send a method invocation without even knowing which object(s) have such a method, as demonstrated below.

```rust
struct A;
struct B;
pub fn main() {
    /* registration */
    let a = nuts::new_activity(A);
    let b = nuts::new_activity(B);

    a.subscribe(|&mut A, msg: &&'static str| println!("A received: {}", msg));
    b.subscribe(|&mut B, msg: &&'static str| println!("B received: {}", msg));

    /* invocations */
    nuts::publish("Hello World");

    /* Output */
    // A received: Hello World
    // B received: Hello World
}
```

A couple of things are different here.
First, objects are called activities and methods are called subscriptions.
Second, when registering an activity, an activity ID is returned and registration of subscriptions only work on such an ID.
This makes the API a bit cleaner in my opinion, since previously, a method could be registered without the object even being present, which shouldn't happen.

Finally, instead of `invoke`, there is now `publish` which takes no type parameter for the receiver.
Nuts internally keeps a list of subscriptions listening to each message type (`&'static str` in this case) and calls them all when such a message is published.

Thus, the single call to `publish` results in several subscriptions being called.
Consequently, the subscription only gets a borrowed value to work with. (For `&'static str` as message type, this results in the weird double-borrow `&&`.)

This generalization makes Nuts more of a publish-subscribe library.
But the owned data transfer is still supported in Nuts, going under the name of [private channels][docs-private-channel].
Invocations must then use a syntax like `send_to::<Receiver>(msg)` to make it clear which object should receive the message.


But to really solve the problem I had initially, Nuts needed to do more.
Calling a method on an object is pretty good already but sometimes data also needs to be shared between activities.
Therefore, Nuts supports to group activities in [domains][docs-domains]. Each domain has a singleton collection as introduced in section one.
Subscription handlers can access this collection mutably.

This allows them to share arbitrary state. Here is an example.

```rust
let a = nuts::new_domained_activity(A, &nuts::DefaultDomain);
let b = nuts::new_domained_activity(B, &nuts::DefaultDomain);

nuts::store_to_domain(&nuts::DefaultDomain, 0u32);
nuts::store_to_domain(&nuts::DefaultDomain, "This is Nuts!");

a.subscribe_domained(|_, domain, _msg: &()| {
    let counter = domain.get_mut::<u32>();
    *counter += 1;
    println!("A counts to {}", counter);
});
b.subscribe_domained(|_, domain, _msg: &()| {
    let counter = domain.get_mut::<u32>();
    *counter += 1;
    println!("B counts to {}", counter);
});
b.subscribe_domained(|_, domain, _msg: &()| {
    let message = domain.get::<&'static str>();
    println!("B reports message: {}", message);
});

nuts::publish(());

/* Output */
// A counts to 1
// B counts to 2
// B reports message: This is Nuts!
```

This example used the unit type `()` as the message, which works just fine as a topic to listen to.
And it uses the *domained* versions of all methods interacting with activities.
When registering, this means we have to also provide which domain the activity should belong to.
To keep it simple, the default domain is used for both activities.

Then, we can store some values to it using `nuts::store_to_domain`.
This puts them inside the singleton collection associated with the domain, which is provided as the second argument to the callback registered with `subscribe_domained`.

Combining all these features, I used types as a key to hash maps three times: For activities, for subscriptions, and for domains.
None of this would be possible (at least not safely) without the downcast that Rust provides in `core::any`.

Was it worth it to go through all of this for me? Yes! The code in [Paddlers][paddlers] has become so much cleaner. Among many other benefits, it allowed me to implement an abstraction over user input from the browser and forward it to exactly those activities which are interested in them. Unfortunately, there are still more issues with Rust and browser interactions but that's a topic for another day.

## Comparison to existing implementations
In my research for this article, I found that I was not the first to have the idea to store heterogenous functions in a hash map and invoke them by their argument.
[QuietMisdreavus][quiet-misdreavus] has already published crate under the name [handler_map][handler_map].

While handler_map takes a conservative approach and only calls functions, I went a bit more crazy with the concept.
I also dynamically store object and then dispatch methods on those, instead of plain function calls.

Another interesting find was the crate called [Eventbus].
It lacks documentation but what I read from the code, events are shared similar to how messages are published in Nuts.
But there are key differences.

Functionally, the biggest difference is that each handler in Eventbus can modify the event/message and subsequent handlers will see the changes. And similar to handler_map, handlers in Eventbus have no state like the objects in Nuts.
On a syntax level, Eventbus uses macros (`register_hook!` and `post_event!`) whereas Nuts works fine using regular function calls.


At this point, I also want to briefly differentiate Nuts from the [actor model][actor-wikipedia].

1. Activities (objects) in Nuts can share state, unlike what is the norm for actors.
2. Methods in Nuts are always executed sequentially. As opposed to concurrent execution typical for actors.
3. To communicate to other actors, an actor usually needs to explicitly obtain the receiver address. Either as a parent/child dependency, or by receiving the address inside a message.
  In Nuts, the address is the Rust type of the receiver, hence readily available without explicit setup.

These are the main reasons I wouldn't consider Nuts an actor-system, despite similar use of message passing to define program flow.

This wraps up section two. Next, we'll look at universal type IDs as a generalization of `core::any::TypeId`.

# Section 3: Universal Type IDs
![Image: A sunset.](/assets/img/21/sunset.jpg)

Section 1 and 2 showed how type IDs are useful within a single binary.
They allowed us to write library code that is unaware of the specific types.
The library user then defined those types at compile-time.

But what if we wanted to take type IDs beyond the binary boundary?
What if the type is not known at compile-time at all?

You see, I've got this dream that an API just like the one for Nuts could be used for a networked system.
Endpoints could register and call remote procedures just like I registered and invoked methods on objects in section 2.

I admit, there are other ways to achieve functionally equivalent systems.
[SOAP][soap-wikipedia], born in 1998, comes to mind as a standardized way to share typed objects between machines.
Many more modern alternatives exist. (Please excuse me for not listing them all here.)
However, none of them operate on native Rust types! That's the kind of crazy idea I bring to the table.
This is where the fun's at.

To be fair though, remote procedure calls (RPCs) with a native Rust definition already exist, too.
With [tarpc][tarpc], RPC interfaces are specified in pure Rust code, which is close to what I want to achieve.

But my system wouldn't just be for point-to-point RPCs. 
It would be a dynamic publish-subscribe system just like Nuts from section 2, but this time networked.
Like Nuts, routing between nodes would occur based solely on the compile-time types and there would be not a single layer defined by weakly-typed strings, like URIs in your average REST API.

But hold on. If the type must be known when the binary is compiled, how is this any flexible?
Well, the key guarantee I want to make is that endpoints can safely be recompiled and updated.
All types which haven't been changed should still be compatible with older binaries.
 <!-- and the remainder should be handled appropriately. -->

At the end of the day, to materialize my dream, I need a way to compare types between independently compiled binaries.
Can I use `TypeId`?

## Dirty secrets about TypeId

At the time of writing, `TypeId` is just a wrapper for a private `u64`.
That integer value is constructed by a hash performed the middle-end of the compiler. ([Link to source code][typeid-hash])

I wanted to know what changes to a struct exactly affect its `TypeId` value.
Not feeling like digging too deep into the compiler code, I just tested a few things. 
Here are a list of things that do change the value:
* Renaming the struct
* Renaming fields
* Moving the definition to another module
* Syntax changes (e.g. `MyType{}` to `MyType`)

On the other hand, these things will not change the `TypeId`:
* Changing the type of a field
* Adding methods in an `impl` block or through a `#[derive(...)]`

However, the compiler team is also free to change the hash construction with each update.
<!-- (Hence my unwillingness to look at / discuss compiler implementation details.) -->

Here is a quote from the official documentation of [`TypeId`][docs-typeid].

    While TypeId implements Hash, PartialOrd, and Ord, it is worth noting
    that the hashes and ordering will vary between Rust releases. 
    Beware of relying on them inside of your code!

It's actually not unlikely that major changes come in the near future.
The [oldest unresolved soundness issue of Rust][typeid-collision] is currently the fact that these hash values could (in theory) collide.
A [pull-request][typeid-widen] to increase the integer size has been discussed and rejected only a few months ago.

## Why I'm not using TypeId
Considering all of this, I can't really use `TypeId`.

I realized that `TypeId` doesn't reflect my use case well at all.
It's designed to be used inside a single binary, not shared among many like I'm envisioning.

Put another way, `TypeId` is unique among a set of types of a static code base. Beyond that, the meaning of the term "type" is not well defined.
But a changing code base is exactly the case I want to handle.

For example, I might have a type defined by `struct A { counter: i32}` and later decide it should be `struct A { counter: i64}` instead.
The standard `TypeId` wouldn't change in this case, with the current compiler version.
But for me, these are two incompatible types.
And it would be allowed for both to coexist in the same system of many binaries.
So, to avoid memory corruption, the type ID I'm going to use must change if a field type changes.

My plan became clear. I have to create my own, universal type ID.

## Deriving my own Type ID

A procedural macro seems to be best way to compute a type ID.
A `#[derive(UniversalType)]` can be slapped onto any `struct`, `enum` and `union`.
Unfortunately, I haven't come up with an idea yet to cover other types, such as function pointer or closure types.
But to fullfil my dream of a networked dynamic publish-subscribe system, this is already sufficient.

Now, what should the procedural macro do?
My idea is to create a string for each type that is unique if and only if the types are named the same and their data representation is compatible.

I then hash the string inside the macro, all at compile-time, so that only a numeric value ends up in the actual binary.

The important decision is just what components go into this unique string.
I'll give you my reasoning here. 

To start, let me just use the source code of the `struct` (let's ignore `enum` and `union` for now) and strip it of all comments and whitespace.
This way, any change to a field or to the name of the struct will cause the type ID to change, hence it is considered a different type.
Which is exactly what I want.

What about the module the type is defined inside? Or the crate?
At first, I thought I don't care.
Or more precisely, I wanted this to be a non-factor for type uniqueness. 

Why, I hear you ask. The answer is maximum flexibility.
Let's look at each of the two. (crate and module)

A small refactor, such as renaming a parent module, should not change the identity of my type, in my view.
Thus, the module should not be part of it.

Making it even crate independent is the other decision.
I like and dislike the idea that a crate can impersonate types of another.
Liking for the possibility to share types without a cargo dependency, disliking for the risks involved.

Memory safety is not at risk, of course, since the field must match exactly. And proper serialization code would have to be used anyway (no in-memory magic).
The problem is more that every field suddenly becomes part of the public interface of a crate, if it derives this ID.
One big problem with that is that multiple crates could accidentally share types.
Small updates could change that and thus lead to all kind of weird bugs.

But there is one very important reason why the crate and module chain should really be part of the type ID.
That is, otherwise, the generated type ID would disagree with the compiler's notion of a type even within a single binary.

I mean, I want there to be differences when comparing types across binaries, that is kind of the point.
But within a binary, it would be quite flawed to have these discrepancies.
Thus, for the usage example, I'll assume module name and crate are considered for calculating the ID.

On the other hand, a flexible solution could be to introduce namespace as an input to the procedural macro.
If left unspecified, the fully qualified name of the module would be used, including the crate name.
Then, in the default case, the generated type ID should be equivalent (in terms of its equality relation) to `core::any::TypeId`. 

The nice thing is that with this flexibility, someone who knows what she is doing can still do weird sharing by overriding that namespace.
Whoever decides to change a namespace should then just be aware that equally named structs (in different modules) with identical fields have the same universal ID, even if the Rust compiler considers them to be different.

I started implementing a prototype of a universal type idea in a procedural macro, [the code is on Github][github-uti].
If it matures well, I might release it on crates.io at some point.
But at the moment, the implementation is incomplete and there are still open design questions around. (How to handle generic type parameters?)

Let's leave it at that for now. I will finish this section with a brief look at how the type ID could be used in code.

## How would this be used?
In my [example implementation][github-uti], I created a trait called `UniversalType` that can be derived.
For types that implement it, a `UniversalTypeId` can be retrieved, akin to `TypeId` from the Rust core.

The `UniversalTypeId` is best used in conjunction with the standard `TypeId`.
Within each binary, there should be a one-to-one mapping between the two. (Ignoring namespace sharing to keep it simple. In other words, module names and origin crate matter for type ID.)
It's just that another binary might have another `TypeId` associated while my rules for the `UniversalTypeId` sees them as the same type.

With that realization, we can use a `HashMap<UniversalTypeId, Box<dyn Any>>` and then do all the tricks we previously did with `HashMap<TypeId, Box<dyn Any>>`.
Inside a single binary, this is completely equivalent to what I did in section one and two.

But we have to be careful when sending data across binaries. The memory layout of Rust is not stable, so we can't just send the pure binary.
Luckily, Rust has good tooling for safe serialization with [serde][serde-github] and, for example, [bincode].

The implementation to publish a message will always know the type, so calling `serialize` can be done as usual.
The raw data would then be sent over then network, alongside its universal type ID.

```rust
/* Send message */
let message = Ticket { number: 1 };
let header = message.universal_type_id();
let serialized_message: Vec<u8> = bincode::serialize(&message).unwrap();
// Now (header, serialized_message) is sent over the network
```

Deserialization is more interesting.
We'll have to wrap the deserialize-call (with a monomorphized type parameter) into a closure.
To do that, each type that we are expecting to receive should be registered and stored away in a hash map indexed by UniversalTypeId.

I illustrate the concept below in a fully working example.
Notice how simple it is to register a message type in the main function.
And that despite the complexity hidden inside the implementation of `register_message_type()`.

```rust
#[derive(UniversalType, Serialize, Deserialize, Debug)]
struct Ticket {
    number: i32,
}

fn main() {
    /* Setup */
    let mut lib_state = SubscriptionManager::default();
    lib_state.register_message_type::<Ticket>();

    /* Send message */
    let message = Ticket { number: 1 };
    let header = message.universal_type_id();
    let serialized_message: Vec<u8> = bincode::serialize(&message).unwrap();

    // Now assume (header, serialized_message) is sent over the network

    /* Receive message and call subscriber */
    lib_state.forward(header, &serialized_message);

    // Received: Ticket { number: 1 }
}

#[derive(Default)]
struct SubscriptionManager {
    subscribers: HashMap<UniversalTypeId, Box<dyn Fn(&[u8])>>,
}

impl SubscriptionManager {
    fn register_message_type<T>(&mut self)
    where
        T: Any + UniversalType + DeserializeOwned + Debug,
    {
        let deserializer: Box<dyn Fn(&_)> =
            Box::new(|data| match bincode::deserialize::<T>(data) {
                Ok(msg) => {
                    println!("Received: {:?}", msg);
                }
                Err(err) => {
                    println!("ERROR: Failed to parse incoming message. {}", err);
                }
            });
        let uid = UniversalTypeId::of::<T>();
        self.subscribers.insert(uid, deserializer);
    }
    fn forward(&self, uit: UniversalTypeId, raw_data: &[u8]) {
        self.subscribers[&uit](raw_data);
    }
}
```

Of course, instead of debug printing the value, something more useful should be done with it.
The decoder function could convert it to a proper `Box<dyn Any>` trait object and pass it onto functions that work on that.

Or, the value could be handled by Nuts as it exists today. That is, we could call `nuts::publish(msg)` and all local subscribers get to act.

# Final Thoughts
Alright, I'm happy to see you're still reading! Let's recapitulate what I demonstrated in this article.

First, I showed that types can directly be used as keys into a heterogenous collection.
Effectively removing the need for string keys, while not giving up flexibility.

Then I took it a step further. I stored functions in a collection and indexed them by the type ID of their arguments.
This allows for a dynamic dispatch, which I used to implement a publish-subscribe library, called Nuts.
In terms of usability, the most important characteristic of Nuts is that it can be used from anywhere, including inside callback closures, without the need for a receiver address or any other state.

Finally, I sketched an approach to stretch dynamic types beyond the limitations inherited from a compiler with an unstable ABI.

All these ideas offer an interesting mix between dynamic typing with compile-time checked types.
When I started playing around with these concepts, I thought what I did was completely nuts.
I love Rust for its static type system, going full dynamic seemed like such a stupid idea!

But then I started to see how useful it can be.
And the big surprise was that static type checks also rule over essentially all the dynamic-typing code.
As evident by all the complex type arguments shown earlier, you just can't escape the Rust compiler!
Once again, I was blown away by the phenomenal power of Rust's type system.
That's why I wanted to write this article.

It was not an easy article to write, however.
I realized in my first couple of attempts that I didn't even know myself what this should be good for.
Of course, I knew my code makes sense and I had been using it effectively.
But articulating why my (admittedly weird) approach makes any sense was challenging.
Beyond that, striking the balance between brevity and not leaving out important details was extremely difficult.

Anyway, I hope this final version makes my point clear: We (as a community) can use Rust types for more than we've done so far.

I always try to write for the widest possible audience.
But I fear this time, it might only be digestible by veteran Rust programmers.
Please let me know if you have any feedback regarding this. (Or otherwise.)
I'm always looking to improve my writing.

Finally, I'm really interested to hear more opinions about this kind of dynamic typing. (Taking it even further than [AnyMap][anymap] already does.)
Do you think it's a hidden gem waiting to be applied more widely in Rust? Or do you think it very niche and should rarely, if ever be used?

*This blog has been shared on the [Rust programming forum](https://users.rust-lang.org/t/blog-post-untapped-potential-in-rust-s-type-system/60372).*

<!-- ## References -->
[actor-wikipedia]: https://en.wikipedia.org/wiki/Actor_model
[any-docs]: https://doc.rust-lang.org/std/any/index.html
[anymap]: https://github.com/chris-morgan/anymap
[bincode]: https://crates.io/crates/bincode
[crate-blake2]: https://crates.io/crates/blake2
[chris-morgan]: https://chrismorgan.info/
[crater]: https://github.com/rust-lang/crater
[docs-domains]: https://docs.rs/nuts/0.2.1/nuts/struct.DomainState.html
[docs-fnmut]: https://doc.rust-lang.org/nomicon/hrtb.html
[docs-higher-level-trait-bound]: https://doc.rust-lang.org/nomicon/hrtb.html
[docs-private-channel]: https://docs.rs/nuts/0.2.1/nuts/struct.ActivityId.html#method.private_channel
[docs-trait-object]: https://doc.rust-lang.org/reference/types/trait-object.html
[docs-typeid]: https://doc.rust-lang.org/std/any/struct.TypeId.html
[downcast_ref]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref
[downcast]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast
[downcast-rs]: https://crates.io/crates/downcast-rs
[eventbus]: https://crates.io/crates/eventbus
[fold-src]: https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#2117-2120
[handler_map]: https://crates.io/crates/handler_map
[message-passing-wikipedia]: https://en.wikipedia.org/wiki/Message_passing
[nuts]: https://github.com/jakmeier/nuts
[oop-wikipedia]: https://en.wikipedia.org/wiki/Object-oriented_programming
[paddlers]: https://github.com/jakmeier/paddlers-browser-game/
[promise]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise
[quiet-misdreavus]: https://github.com/QuietMisdreavus
[rfc-1849]: https://rust-lang.github.io/rfcs/1849-non-static-type-id.html
[serde-github]: https://github.com/serde-rs/serde
[shred-world]: https://github.com/amethyst/shred/blob/63778ae268970b7526f74fca7ec6e0364a7514c9/src/world/mod.rs
[shred]: https://github.com/amethyst/shred
[soap-wikipedia]: https://en.wikipedia.org/wiki/SOAP
[tarpc]: https://github.com/google/tarpc
[transmute]: https://doc.rust-lang.org/std/mem/fn.transmute.html
[typeid-abuser-list]: https://github.com/rust-lang/rust/pull/75923#issuecomment-699080944
[typeid-hash]: https://github.com/rust-lang/rust/blob/fa72878a61f2b0a2127fe7d700724642fc79ec66/compiler/rustc_middle/src/ty/util.rs#L141
[typeid-collision]: https://github.com/rust-lang/rust/issues/10389
[typeid-widen]: https://github.com/rust-lang/rust/pull/75923
[github-uti]: https://github.com/jakmeier/universal-type-id
[wasm-article]: Rust_on_the_Web.html

<!-- Code examples -->
[type-id-example-0]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=use%20core%3A%3Aany%3A%3A%7BAny%2C%20TypeId%7D%3B%0Afn%20main()%20%7B%0A%20%20%20%20let%20one_hundred%20%3D%20100u32%3B%0A%20%20%20%20%2F%2F%20Get%20the%20type%20ID%20usaing%20a%20value%20of%20that%20type.%0A%20%20%20%20let%20t0%20%3D%20one_hundred.type_id()%3B%0A%20%20%20%20%2F%2F%20Get%20the%20type%20ID%20directly%0A%20%20%20%20let%20t1%20%3D%20TypeId%3A%3Aof%3A%3A%3Cu32%3E()%3B%0A%20%20%20%20assert_eq!(t0%2C%20t1)%0A%7D%0A

[type-id-example-1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=use%20core%3A%3Aany%3A%3A%7BAny%2C%20TypeId%7D%3B%0Ause%20std%3A%3Aops%3A%3ADeref%3B%0A%0Astruct%20Rectangle%3B%0Astruct%20Triangle%3B%0A%0Atrait%20Shape%3A%20Any%20%7B%7D%0A%0Aimpl%20Shape%20for%20Rectangle%20%7B%7D%0Aimpl%20Shape%20for%20Triangle%20%7B%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20shapes%3A%20Vec%3CBox%3Cdyn%20Shape%3E%3E%20%3D%0A%20%20%20%20%20%20%20%20vec!%5BBox%3A%3Anew(Rectangle)%2C%20Box%3A%3Anew(Triangle)%2C%20Box%3A%3Anew(Rectangle)%5D%3B%0A%20%20%20%20let%20n%20%3D%20count_rectangles(%26shapes)%3B%0A%20%20%20%20assert_eq!(2%2C%20n)%3B%0A%7D%0A%0Afn%20count_rectangles(shapes%3A%20%26%5BBox%3Cdyn%20Shape%3E%5D)%20-%3E%20usize%20%7B%0A%20%20%20%20let%20mut%20n%20%3D%200%3B%0A%20%20%20%20for%20shape%20in%20shapes%20%7B%0A%20%20%20%20%20%20%20%20%2F%2F%20Need%20to%20derefernce%20once%20or%20we%20will%20get%20the%20type%20of%20the%20Box!%0A%20%20%20%20%20%20%20%20let%20type_of_shape%20%3D%20shape.deref().type_id()%3B%0A%20%20%20%20%20%20%20%20if%20type_of_shape%20%3D%3D%20TypeId%3A%3Aof%3A%3A%3CRectangle%3E()%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20n%20%2B%3D%201%3B%0A%20%20%20%20%20%20%20%20%7D%20else%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20println!(%22%7B%3A%3F%7D%20is%20not%20a%20Rectangle!%22%2C%20type_of_shape)%3B%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20n%0A%7D%0A

[downcast-example]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=use%20core%3A%3Aany%3A%3A%7BAny%2C%20TypeId%7D%3B%0Ause%20std%3A%3Aops%3A%3ADeref%3B%0A%0Astruct%20Rectangle%3B%0Astruct%20Triangle%3B%0A%0Atrait%20Shape%3A%20Any%20%7B%7D%0A%0Aimpl%20Shape%20for%20Rectangle%20%7B%7D%0Aimpl%20Shape%20for%20Triangle%20%7B%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20mut%20shapes%3A%20Vec%3CBox%3Cdyn%20Any%3E%3E%20%3D%0A%20%20%20%20%20%20%20%20vec!%5BBox%3A%3Anew(Rectangle)%2C%20Box%3A%3Anew(Triangle)%2C%20Box%3A%3Anew(Rectangle)%5D%3B%0A%20%20%20%20remove_first_rectangle(%26mut%20shapes).expect(%22No%20rectangle%20found%20to%20be%20removed%22)%3B%0A%7D%0A%0Afn%20remove_first_rectangle(shapes%3A%20%26mut%20Vec%3CBox%3Cdyn%20Any%3E%3E)%20-%3E%20Option%3CBox%3CRectangle%3E%3E%20%7B%0A%20%20%20%20let%20idx%20%3D%20shapes%0A%20%20%20%20%20%20%20%20.iter()%0A%20%20%20%20%20%20%20%20.position(%7Cshape%7C%20shape.deref().type_id()%20%3D%3D%20TypeId%3A%3Aof%3A%3A%3CRectangle%3E())%3F%3B%0A%20%20%20%20let%20rectangle_as_unknown_shape%20%3D%20shapes.remove(idx)%3B%0A%20%20%20%20rectangle_as_unknown_shape.downcast().ok()%0A%7D%0A

[singleton-collection-playground]: https://play.rust-lang.org/?version=stable&mode=release&edition=2018&code=use%20core%3A%3Aany%3A%3A*%3B%0Ause%20std%3A%3Acollections%3A%3AHashMap%3B%0A%0Astruct%20SingletonCollection%20%7B%0A%20%20%20%20data%3A%20HashMap%3CTypeId%2C%20Box%3Cdyn%20Any%3E%3E%2C%0A%7D%0Aimpl%20SingletonCollection%20%7B%0A%20%20%20%20fn%20get%3CT%3A%20Any%3E(%26self)%20-%3E%20%26T%20%7B%0A%20%20%20%20%20%20%20%20self.data%5B%26TypeId%3A%3Aof%3A%3A%3CT%3E()%5D%0A%20%20%20%20%20%20%20%20%20%20%20%20.downcast_ref()%0A%20%20%20%20%20%20%20%20%20%20%20%20.as_ref()%0A%20%20%20%20%20%20%20%20%20%20%20%20.unwrap()%0A%20%20%20%20%7D%0A%20%20%20%20fn%20set%3CT%3A%20Any%3E(%26mut%20self%2C%20value%3A%20T)%20%7B%0A%20%20%20%20%20%20%20%20self.data.insert(TypeId%3A%3Aof%3A%3A%3CT%3E()%2C%20Box%3A%3Anew(value))%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20mut%20collection%20%3D%20SingletonCollection%20%7B%0A%20%20%20%20%20%20%20%20data%3A%20HashMap%3A%3Anew()%2C%0A%20%20%20%20%7D%3B%0A%0A%20%20%20%20let%20float_input%3A%20f32%20%3D%203.14%3B%0A%20%20%20%20let%20integer_input%3A%20u32%20%3D%20888%3B%0A%0A%20%20%20%20collection.set(float_input)%3B%0A%20%20%20%20collection.set(integer_input)%3B%0A%0A%20%20%20%20let%20float_output%20%3D%20*collection.get%3A%3A%3Cf32%3E()%3B%0A%20%20%20%20let%20integer_output%20%3D%20*collection.get%3A%3A%3Cu32%3E()%3B%0A%20%20%20%20%0A%20%20%20%20assert_eq!(float_input%2C%20float_output)%3B%0A%20%20%20%20assert_eq!(integer_input%2C%20integer_output)%3B%0A%7D%0A

[playground-bad-interface]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=use%20core%3A%3Aany%3A%3A*%3B%0Ause%20std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20mut%20collection%20%3D%20HashMap%3A%3A%3C%26str%2C%20Box%3Cdyn%20Any%3E%3E%3A%3Anew()%3B%0A%20%20%20%20collection.insert(%22f32%22%2C%20Box%3A%3Anew(3.14f32))%3B%0A%20%20%20%20collection.insert(%22f64%22%2C%20Box%3A%3Anew(2.71f64))%3B%0A%20%20%20%20collection.insert(%22another%20f32%22%2C%20Box%3A%3Anew(1.618f32))%3B%0A%20%20%20%20%0A%20%20%20%20let%20unknown_output%3A%20%26Box%3Cdyn%20Any%3E%20%3D%20collection.get(%22f32%22).unwrap()%3B%0A%20%20%20%20let%20maybe_f32_output%20%3A%20Option%3C%26f32%3E%20%3D%20unknown_output.downcast_ref%3A%3A%3Cf32%3E()%3B%0A%20%20%20%20let%20f32_output%3A%20f32%20%3D%20*maybe_f32_output.expect(%22pinky%20promise%20I%20use%20the%20right%20key%22)%3B%0A%0A%20%20%20%20assert_eq!(%203.14%2C%20f32_output)%3B%0A%7D

[playground-good-interface]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=use%20core%3A%3Aany%3A%3A*%3B%0Ause%20std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20mut%20collection%20%3D%20HeteroCollection%3A%3Adefault()%3B%0A%20%20%20%20collection.set(%22f32%22%2C%203.14f32)%3B%0A%20%20%20%20collection.set(%22f64%22%2C%202.71f64)%3B%0A%20%20%20%20collection.set(%22another%20f32%22%2C%201.618f32)%3B%0A%20%20%20%20%0A%20%20%20%20let%20f32_output%20%3D%20*collection.get%3A%3A%3Cf32%3E(%22f32%22).unwrap()%3B%0A%20%20%20%20assert_eq!(%203.14%2C%20f32_output)%3B%0A%7D%0A%0A%23%5Bderive(Default)%5D%0Astruct%20HeteroCollection%20%7B%0A%20%20%20%20data%3A%20HashMap%3C%26'static%20str%2C%20Box%3Cdyn%20Any%3E%3E%2C%0A%7D%0Aimpl%20HeteroCollection%20%7B%0A%20%20%20%20fn%20get%3CT%3A%20'static%3E(%26self%2C%20key%3A%20%26'static%20str)%20-%3E%20Option%3C%26T%3E%20%7B%0A%20%20%20%20%20%20%20%20let%20unknown_output%3A%20%26Box%3Cdyn%20Any%3E%20%3D%20self.data.get(key)%3F%3B%0A%20%20%20%20%20%20%20%20unknown_output.downcast_ref()%0A%20%20%20%20%7D%0A%20%20%20%20fn%20set%3CT%3A%20'static%3E(%26mut%20self%2C%20key%3A%20%26'static%20str%2C%20value%3A%20T)%20%7B%0A%20%20%20%20%20%20%20%20self.data.insert(key%2C%20Box%3A%3Anew(value))%3B%0A%20%20%20%20%7D%0A%7D

[playground-simplified-nuts]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=bfcd9a908ca17401cdf3b212dd8c6453
[playground-simplified-nuts-2]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ef1d9b99e04ddb5ba3bbb6adc4682efb
