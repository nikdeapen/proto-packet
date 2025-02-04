use code_gen::rust::ImplBlock;
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Enum, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Encode

    /// Generates the encoding trait implementations for the `enum`.
    pub fn gen_enum_encode(&self, enom: &Enum) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_encoded_len(enom));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_encode_to_slice(enom));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_encode_to_write(enom));

        source
    }
}

impl GenRust {
    //! `enc::EncodedLen`

    fn gen_enum_encoded_len(&self, enom: &Enum) -> ImplBlock {
        self.gen_encoded_len_impl(
            enom.type_name(),
            Source::default().with_literal(format!(
                "{}::from({}).encoded_len()",
                "enc::var_int::VarInt32", "self.tag_number().tag_number()"
            )),
        )
    }
}

impl GenRust {
    //! `enc::EncodeToSlice`

    fn gen_enum_encode_to_slice(&self, enom: &Enum) -> ImplBlock {
        self.gen_encode_to_slice_impl(
            enom.type_name(),
            Source::default().with_literal(format!(
                "{}::from({}).encode_to_slice_unchecked(target)",
                "enc::var_int::VarInt32", "self.tag_number().tag_number()"
            )),
        )
    }
}

impl GenRust {
    //! `enc::EncodeToWrite`

    fn gen_enum_encode_to_write(&self, enom: &Enum) -> ImplBlock {
        self.gen_encode_to_write_impl(
            enom.type_name(),
            Source::default().with_literal(format!(
                "{}::from({}).encode_to_write(w)",
                "enc::var_int::VarInt32", "self.tag_number().tag_number()"
            )),
        )
    }
}
