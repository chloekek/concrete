use std::collections::HashMap;
use std::ffi::CStr;
use std::io;
use utility::zmq;
use protocol::cap::CapSet;

/// The identifier of a slave is
/// the identity of the connection with the slave as
/// reported by the command socket of the master.
/// It can be directly used as the identity
/// in the envelope of a reply to the slave.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SlaveId(pub [u8; 5]);

/// The global state of a master includes
/// all state that is necessary for correct operation of
/// the master state machine.
pub struct Master<'a>
{
    pub command: zmq::Socket<'a>,

    /// A slave is considered idle immediately after
    /// an IDLE message has been received from it over the command socket,
    /// and is added to this set.
    /// Once a COMMAND message has been sent to a slave
    /// the slave is no longer considered idle,
    /// and is removed from this set.
    pub idleSlaves: HashMap<SlaveId, CapSet>,
}

impl<'a> Master<'a>
{
    pub fn new(command: zmq::Socket<'a>) -> Self
    {
        Master{command, idleSlaves: HashMap::new()}
    }

    pub fn bind(
        context: &'a zmq::Context, commandsEndpoint: &CStr,
    ) -> io::Result<Self>
    {
        let commands = zmq::socket(context, zmq::ROUTER)?;
        zmq::bind(&commands, commandsEndpoint)?;
        Ok(Master::new(commands))
    }

    /// Find an idle slave with
    /// the smallest number of capabilities that
    /// has at least the given capabilities.
    pub fn findIdleSlave(&self, requiredCaps: &CapSet) -> Option<SlaveId>
    {
        self.idleSlaves.iter()
            .filter(|(_, caps)| caps.is_superset(requiredCaps))
            .min_by_key(|(_, caps)| caps.len())
            .map(|(&id, _)| id)
    }
}

// TODO: Receive request from slave. The request contains the caps of
// TODO: the slave. Insert the slave into the collection of idle slaves and
// TODO: advance the state machine.
