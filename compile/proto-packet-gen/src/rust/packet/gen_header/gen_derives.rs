use crate::rust::GenRust;
use code_gen::rust::WithDerives;
use proto_packet::PacketType;
use proto_packet::PacketType::{Enum, Message};

impl GenRust {
    //! Gen Derives

    /// Generates the derives for the `element` of the `packet_type`.
    pub(in crate::rust) fn gen_derives<T>(&self, element: &mut T, packet_type: PacketType)
    where
        T: WithDerives,
    {
        if packet_type == Enum {
            element.add_derive("Copy");
        }
        element.add_derive("Clone");
        element.add_derive("Ord");
        element.add_derive("PartialOrd");
        element.add_derive("Eq");
        element.add_derive("PartialEq");
        element.add_derive("Hash");
        element.add_derive("Debug");
        if packet_type == Message {
            element.add_derive("Default");
        }
        element.add_derive("Serialize");
        element.add_derive("Deserialize");
    }
}
