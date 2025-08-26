use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::{
    Enum as RustEnum, RustType, Struct as RustStruct, StructField, WithAccess, WithStructFields,
};
use proto_packet::PacketType;
use proto_packet_tree::{Enum, Message, Struct, Variant, WithTypeName};

impl GenRust {
    //! Gen Type Dec

    /// Generates the struct declaration for the struct `s`.
    pub(in crate::rust) fn gen_type_dec_struct(&self, s: &Struct) -> RustStruct {
        let mut result: RustStruct = RustStruct::from(self.naming.type_name(s.type_name()));

        self.gen_type_dec_comments_struct(&mut result, s);
        self.gen_derives(&mut result, PacketType::Struct);
        result.set_access(Public);

        for field in s.fields() {
            result.add_field(self.gen_type_dec_struct_field(field));
        }

        result
    }

    /// Generates the struct declaration for the message `m`.
    pub(in crate::rust) fn gen_type_dec_message(&self, m: &Message) -> RustStruct {
        let mut result: RustStruct = RustStruct::from(self.naming.type_name(m.type_name()));

        self.gen_type_dec_comments_message(&mut result, m);
        self.gen_derives(&mut result, PacketType::Message);
        result.set_access(Public);

        result.add_field(StructField::from((
            self.naming.unrecognized_fields_name.as_str(),
            RustType::from("Vec<u8>"),
        )));
        for field in m.fields() {
            result.add_field(self.gen_type_dec_message_field(field));
        }

        result
    }

    /// Generates the enum declaration for the enum `e`.
    pub(in crate::rust) fn gen_type_dec_enum(&self, e: &Enum) -> RustEnum {
        let mut result: RustEnum = RustEnum::from(self.naming.type_name(e.type_name()));

        self.gen_type_dec_comments_enum(&mut result, e);
        self.gen_derives(&mut result, PacketType::Enum);
        result.set_access(Public);

        for case in e.cases() {
            result.add_case(self.gen_type_dec_enum_case(case));
        }

        result
    }

    /// Generates the enum declaration for the variant `v`.
    pub(in crate::rust) fn gen_type_dec_variant(&self, v: &Variant) -> RustEnum {
        let mut result: RustEnum = RustEnum::from(self.naming.type_name(v.type_name()));

        self.gen_type_dec_comments_variant(&mut result, v);
        self.gen_derives(&mut result, PacketType::Variant);
        result.set_access(Public);

        for case in v.cases() {
            result.add_case(self.gen_type_dec_variant_case(case));
        }

        result
    }
}
