use crate::rust::{EncodeOp, GenRust, Naming};
use code_gen::rust::{MatchCase, MatchStatement};
use code_gen::{Source, WithStatements};
use proto_packet_tree::{TypeTag, Variant, WithTagNumber, WithTypeTag};

impl GenRust {
    //! Gen Variant: Encode

    pub(in crate::rust::gen_packet::gen_variant) fn gen_variant_encode(
        &self,
        v: &Variant,
    ) -> Source {
        self.gen_encode(v, v.cases().is_empty(), |v, op| {
            self.gen_encode_variant_source(v, op)
        })
    }

    fn gen_encode_variant_source(&self, v: &Variant, op: EncodeOp) -> Source {
        let mut source: Source = Source::default();
        if v.cases()
            .iter()
            .any(|c| matches!(c.type_tag(), TypeTag::Named(_)))
        {
            source.add_semi("use proto_packet::Packet;");
        }
        source.add_empty_line();
        source.add_semi("let mut encoded_len: usize = 0;");
        source.add_empty_line();

        let mut match_statement: MatchStatement = MatchStatement::from("self");
        let mut unrecognized: MatchCase =
            MatchCase::from(format!("Self::{}(u)", Naming::UNRECOGNIZED_CASE_NAME));
        match op {
            EncodeOp::EncodedLen => {}
            EncodeOp::EncodeToSlice => {
                unrecognized.add_semi(
                    "(&mut target[..u.serial.len()]).copy_from_slice(u.serial.as_slice())",
                );
            }
            EncodeOp::EncodeToWrite => unrecognized.add_semi("w.write_all(u.serial.as_slice())?"),
        };
        unrecognized.add_semi("encoded_len += u.serial.len()");
        match_statement.add_match_case(unrecognized);
        for case in v.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!("Self::{}(value)", self.naming.case_name(case)))
                    .with_statement(self.gen_encode_field(
                        "value",
                        case.type_tag(),
                        false,
                        case.tag(),
                        op,
                    )),
            )
        }
        source.add_statement(match_statement);

        source.add_empty_line();
        source.add_literal("Ok(encoded_len)");
        source
    }
}
