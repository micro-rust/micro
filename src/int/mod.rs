//! Interrupt handling module.
//! Contains abstactions that facilitate interrupt handling for libraries and users.
//! This allows only for static functions to be used as handlers. These functions
//! can use an optional context to receive data from user code.



mod user;



pub use user::UserHandler;
