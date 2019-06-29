use protocol::cap::CapSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request
{
    /// Sent by a slave to the master when
    /// the slave is ready to receive a command.
    IDLE(CapSet),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response
{
    /// Sent by the master to a slave when
    /// the slave must execute a command.
    COMMAND,
}
