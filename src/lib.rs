//! The continuous integration system consists of
//! two major components:
//! the master component and
//! the slave component.
//! Within an instance of the
//! continous integration system,
//! there is exactly one instance of the master component
//! (referred to as <em>the master</em>) and
//! any number of instances of the slave component
//! (referred to as <em>the slaves</em>).
//!
//! Masters communicate with slaves over [ØMQ][ømq] sockets.
//! Within an instance of the continuous integration system,
//! the master sends commands to the slaves over ROUTER–REQ.
//! The slaves execute the commands and
//! stream status updates to the master over PUSH–PULL.
//! A typical command would be: <em>run this build script</em>.
//! The corresponding status updates would then include
//! the captured output of the build script.
//!
//! To ensure secure communication,
//! all messages sent between masters and slaves are
//! encrypted and signed using asymmetric cryptography.
//! It is important to note that slaves do no authorization checks;
//! masters must be trusted not to distribute malware.
//!
//! [ømq]: http://zeromq.org/
