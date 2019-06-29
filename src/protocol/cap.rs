use std::collections::BTreeSet;
use std::rc::Rc;

/// Capabilities allow filtering slaves
/// so that they can be sent commands that
/// they can actually execute.
///
/// For example, consider a slave running on an AMD64 CPU.
/// It would have the capability "amd64".
/// Commands that require the capability "arm" would
/// not be assigned to this slave.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cap(pub Rc<str>);

/// Each slave has a set of capabilities.
/// Such a set acts as a whitelist:
/// for a command to be assigned to a slave,
/// the set of required capabilities for the command
/// must be a subset of the capabilities of the slave.
pub type CapSet = BTreeSet<Cap>;

