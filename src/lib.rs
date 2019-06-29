#![allow(non_snake_case)]

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
//! <h2>Master–slave communication</h2>
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
//! <h3>The command sockets</h3>
//!
//! The command sockets form a socket pair.
//! The master binds a ROUTER socket.
//! Each slave connects a REQ socket to
//! the ROUTER socket of the master.
//! Once connected,
//! each slave sends an IDLE message
//! that contains the capabilities of the slave.
//! After receiving the IDLE message,
//! the master is aware that the slave is
//! ready to run a command.
//! Once a command is assigned to a slave,
//! the master sends a COMMAND message
//! to the slave.
//! The slave will execute the command and
//! will send another IDLE message when done,
//! such that the cycle repeats.
//!
//! <h3>Security measures</h3>
//!
//! To ensure secure communication,
//! all messages sent between masters and slaves are
//! encrypted and signed using asymmetric cryptography.
//! It is important to note that slaves do no authorization checks;
//! masters must be trusted not to distribute malware.
//!
//! [ømq]: http://zeromq.org/

pub mod master;
pub mod protocol;
pub mod utility;
