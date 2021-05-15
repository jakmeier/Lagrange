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

/* "Library" code */
mod my_library {
    #![allow(non_camel_case_types)]

    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    pub struct Nut {
        objects: HashMap<TypeId, Box<dyn Any>>,
        methods: HashMap<(TypeId, TypeId), Box<dyn FnMut(&mut Box<dyn Any>, Box<dyn Any>)>>,
    }
    impl Nut {
        fn register_object<OBJECT>(&mut self, obj: OBJECT)
        where
            OBJECT: Any,
        {
            let key = TypeId::of::<OBJECT>();
            let boxed_obj = Box::new(obj);
            self.objects.insert(key, boxed_obj);
        }

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
    }

    // The real nuts code has absolutely no unsafe code.
    // But just for readability, global data is stored as mutable static in this example.
    static mut NUT: Option<Nut> = None;
    fn get_nut() -> &'static mut Nut {
        unsafe {
            NUT.get_or_insert_with(|| Nut {
                objects: HashMap::new(),
                methods: HashMap::new(),
            })
        }
    }

    pub fn register_object(obj: impl Any) {
        get_nut().register_object(obj);
    }
    pub fn register_method<OBJECT, ARGUMENT, FUNCTION>(method: FUNCTION)
    where
        FUNCTION: FnMut(&mut OBJECT, ARGUMENT) + 'static,
        ARGUMENT: Any,
        OBJECT: Any,
    {
        get_nut().register_method(method);
    }
    pub fn invoke<OBJECT, ARGUMENT>(method_call: ARGUMENT)
    where
        OBJECT: Any,
        ARGUMENT: Any,
    {
        get_nut().invoke::<OBJECT, ARGUMENT>(method_call);
    }
}
