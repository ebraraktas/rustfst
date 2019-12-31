use failure::Fallible;

use crate::algorithms::fst_convert;
use crate::fst_impls::{ConstFst, VectorFst};
use crate::fst_traits::{Fst, MutableFst};
use crate::semirings::{Semiring, WeightQuantize};
use crate::tests_openfst::FstTestData;
use crate::SymbolTable;
use std::rc::Rc;

pub fn test_const_fst_convert_convert<W>(test_data: &FstTestData<VectorFst<W>>) -> Fallible<()>
where
    W: Semiring<Type = f32> + WeightQuantize,
{
    let mut raw_fst = test_data.raw.clone();

    {
        let mut isymt = SymbolTable::new();
        isymt.add_symbol("a");
        let mut osymt = SymbolTable::new();
        osymt.add_symbol("b");
        osymt.add_symbol("c");

        raw_fst.set_input_symbols(Rc::new(isymt));
        raw_fst.set_output_symbols(Rc::new(osymt));
    }

    let const_fst: ConstFst<_> = raw_fst.clone().into();
    let pred_fst: VectorFst<_> = fst_convert(&const_fst);
    assert_eq!(
        raw_fst,
        pred_fst,
        "{}",
        error_message_fst!(
            raw_fst,
            pred_fst,
            "Convert VectorFst -> ConstFst -> VectorFst"
        )
    );

    // Check symbol tables are still attached
    {
        let isymt = const_fst.input_symbols();
        assert!(isymt.is_some());
        let isymt = isymt.unwrap();
        assert_eq!(isymt.len(), 2);

        let osymt = const_fst.output_symbols();
        assert!(osymt.is_some());
        let osymt = osymt.unwrap();
        assert_eq!(osymt.len(), 3);
    }
    {
        let isymt = pred_fst.input_symbols();
        assert!(isymt.is_some());
        let isymt = isymt.unwrap();
        assert_eq!(isymt.len(), 2);

        let osymt = pred_fst.output_symbols();
        assert!(osymt.is_some());
        let osymt = osymt.unwrap();
        assert_eq!(osymt.len(), 3);
    }
    Ok(())
}
