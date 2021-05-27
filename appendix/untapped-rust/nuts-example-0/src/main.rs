
mod M {
    pub struct MyType {
        field: String
}
}

pub fn main() {
    // example0();
    // example1();
    println!("{:?}", std::any::TypeId::of::<M::MyType>());
}

struct A;
struct B;

pub fn example0() {
    let a = nuts::new_activity(A);
    let b = nuts::new_activity(B);

    a.subscribe(|&mut A, msg: &&'static str| println!("A received: {}", msg));
    b.subscribe(|&mut B, msg: &&'static str| println!("B received: {}", msg));

    nuts::publish("Hello World");

}

pub fn example1() {
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
}
