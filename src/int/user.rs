//! `USerHandler` is an abstraction that allows user to define Interrupt Requests
//! with an optional user provided context.



pub struct UserHandler {
    /// Pointer to the function.
    handler: *const (),

    /// Pointer to the context of this function.
    context: Option<*mut ()>,

    /// Initialization flag.
    init: bool,
}

impl UserHandler {
    /// Static intializer.
    pub const fn empty() -> UserHandler {
        UserHandler { handler: 0 as *mut (), context: None, init: false }
    }

    /// Calls the given handler function if it is initialized.
    pub fn call(&self) {
        if self.init {
            match self.context {
                Some(cx) => {
                    // Empty reference to the context.
                    let cxref: &mut () = unsafe { &mut *(cx) };

                    // Empty reference to the function.
                    let fnref: fn(&mut ()) = unsafe { core::mem::transmute::<*const (), fn(&mut ())>(self.handler) };

                    fnref( cxref );
                },
                _ => {
                    // Empty reference to the function.
                    let fnref: fn() = unsafe { core::mem::transmute::<*const (), fn()>(self.handler) };

                    fnref();
                },
            }
        }
    }

    /// Sets the handler of the `UserContext`.
    pub const fn isolated(handler: fn()) -> Self {
        // Set the handler pointer.
        handler = handler as *const ();

        // Set no context.
        context = None;

        // Indicate as initialized.
        init = true;

        UserHandler { handler, context, init }
    }

    /// Initializes a `UserHandler` to the given 
    pub const fn contextualized<T: Sized>(handler: fn(&mut T), context: &'static mut T, init: bool) -> Self {
        // Set the handler pointer.
        handler = handler as *const ();

        // Set the context.
        context = Some( context as *mut T as *mut () );

        // inherit initialization.
        init = init;

        UserHandler { handler, context, init }
    }
}
