var ALIASES = {};
ALIASES["backtrace"] = {};
ALIASES["backtrace_sys"] = {};
ALIASES["bimap"] = {};
ALIASES["binary_heap_plus"] = {};
ALIASES["bit_set"] = {};
ALIASES["bit_vec"] = {};
ALIASES["bitflags"] = {};
ALIASES["cfg_if"] = {};
ALIASES["compare"] = {};
ALIASES["doc_comment"] = {};
ALIASES["either"] = {};
ALIASES["failure"] = {};
ALIASES["failure_derive"] = {};
ALIASES["generic_array"] = {};
ALIASES["itertools"] = {};
ALIASES["lexical_core"] = {};
ALIASES["libc"] = {};
ALIASES["memchr"] = {};
ALIASES["nom"] = {"-=":[{'crate':'nom','ty':8,'name':'SubAssign','desc':'The subtraction assignment operator `-=`.','p':'nom::lib::std::ops'}],"*":[{'crate':'nom','ty':8,'name':'DerefMut','desc':'Used for mutable dereferencing operations, like in `*v =…','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Deref','desc':'Used for immutable dereferencing operations, like `*v`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Mul','desc':'The multiplication operator `*`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'MulAssign','desc':'The multiplication assignment operator `*=`.','p':'nom::lib::std::ops'}],">=":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'}],"..=":[{'crate':'nom','ty':3,'name':'RangeToInclusive','desc':'A range only bounded inclusively above (`..=end`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'RangeInclusive','desc':'A range bounded inclusively below and above (`start..=end`).','p':'nom::lib::std::ops'}],"^=":[{'crate':'nom','ty':8,'name':'BitXorAssign','desc':'The bitwise XOR assignment operator `^=`.','p':'nom::lib::std::ops'}],">":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'}],"<=":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'}],"^":[{'crate':'nom','ty':8,'name':'BitXor','desc':'The bitwise XOR operator `^`.','p':'nom::lib::std::ops'}],"&=":[{'crate':'nom','ty':8,'name':'BitAndAssign','desc':'The bitwise AND assignment operator `&=`.','p':'nom::lib::std::ops'}],"[]":[{'crate':'nom','ty':8,'name':'IndexMut','desc':'Used for indexing operations (`container[index]`) in…','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Index','desc':'Used for indexing operations (`container[index]`) in…','p':'nom::lib::std::ops'}],">>=":[{'crate':'nom','ty':8,'name':'ShrAssign','desc':'The right shift assignment operator `>>=`.','p':'nom::lib::std::ops'}],"%":[{'crate':'nom','ty':8,'name':'RemAssign','desc':'The remainder assignment operator `%=`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Rem','desc':'The remainder operator `%`.','p':'nom::lib::std::ops'}],"+":[{'crate':'nom','ty':8,'name':'AddAssign','desc':'The addition assignment operator `+=`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Add','desc':'The addition operator `+`.','p':'nom::lib::std::ops'}],"[":[{'crate':'nom','ty':8,'name':'IndexMut','desc':'Used for indexing operations (`container[index]`) in…','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Index','desc':'Used for indexing operations (`container[index]`) in…','p':'nom::lib::std::ops'}],"..":[{'crate':'nom','ty':3,'name':'RangeTo','desc':'A range only bounded exclusively above (`..end`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'RangeFrom','desc':'A range only bounded inclusively below (`start..`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'Range','desc':'A (half-open) range bounded inclusively below and…','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'RangeFull','desc':'An unbounded range (`..`).','p':'nom::lib::std::ops'}],"/":[{'crate':'nom','ty':8,'name':'Div','desc':'The division operator `/`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'DivAssign','desc':'The division assignment operator `/=`.','p':'nom::lib::std::ops'}],"<<":[{'crate':'nom','ty':8,'name':'Shl','desc':'The left shift operator `<<`. Note that because this trait…','p':'nom::lib::std::ops'}],"{}":[{'crate':'nom','ty':8,'name':'Display','desc':'Format trait for an empty format, `{}`.','p':'nom::lib::std::fmt'}],"<":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'}],"&":[{'crate':'nom','ty':8,'name':'BitAnd','desc':'The bitwise AND operator `&`.','p':'nom::lib::std::ops'}],"*=":[{'crate':'nom','ty':8,'name':'MulAssign','desc':'The multiplication assignment operator `*=`.','p':'nom::lib::std::ops'}],"|=":[{'crate':'nom','ty':8,'name':'BitOrAssign','desc':'The bitwise OR assignment operator `|=`.','p':'nom::lib::std::ops'}],"|":[{'crate':'nom','ty':8,'name':'BitOr','desc':'The bitwise OR operator `|`.','p':'nom::lib::std::ops'}],"&*":[{'crate':'nom','ty':8,'name':'Deref','desc':'Used for immutable dereferencing operations, like `*v`.','p':'nom::lib::std::ops'}],">>":[{'crate':'nom','ty':8,'name':'Shr','desc':'The right shift operator `>>`. Note that because this…','p':'nom::lib::std::ops'}],"?":[{'crate':'nom','ty':8,'name':'Try','desc':'A trait for customizing the behavior of the `?` operator.','p':'nom::lib::std::ops'}],"==":[{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial…','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence…','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence…','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial…','p':'nom::lib::std::prelude::v1::v1'}],"!=":[{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial…','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence…','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence…','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial…','p':'nom::lib::std::prelude::v1::v1'}],"-":[{'crate':'nom','ty':8,'name':'Sub','desc':'The subtraction operator `-`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Neg','desc':'The unary negation operator `-`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'SubAssign','desc':'The subtraction assignment operator `-=`.','p':'nom::lib::std::ops'}],"+=":[{'crate':'nom','ty':8,'name':'AddAssign','desc':'The addition assignment operator `+=`.','p':'nom::lib::std::ops'}],"{:?}":[{'crate':'nom','ty':8,'name':'Debug','desc':'`?` formatting.','p':'nom::lib::std::fmt'}],"]":[{'crate':'nom','ty':8,'name':'IndexMut','desc':'Used for indexing operations (`container[index]`) in…','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Index','desc':'Used for indexing operations (`container[index]`) in…','p':'nom::lib::std::ops'}],"<<=":[{'crate':'nom','ty':8,'name':'ShlAssign','desc':'The left shift assignment operator `<<=`.','p':'nom::lib::std::ops'}],"%=":[{'crate':'nom','ty':8,'name':'RemAssign','desc':'The remainder assignment operator `%=`.','p':'nom::lib::std::ops'}],"/=":[{'crate':'nom','ty':8,'name':'DivAssign','desc':'The division assignment operator `/=`.','p':'nom::lib::std::ops'}],};
ALIASES["num_traits"] = {};
ALIASES["ordered_float"] = {};
ALIASES["ordered_iter"] = {};
ALIASES["proc_macro2"] = {};
ALIASES["quote"] = {};
ALIASES["rustc_demangle"] = {};
ALIASES["rustfst"] = {};
ALIASES["ryu"] = {};
ALIASES["stable_bst"] = {};
ALIASES["stackvector"] = {};
ALIASES["static_assertions"] = {};
ALIASES["syn"] = {};
ALIASES["synstructure"] = {};
ALIASES["typenum"] = {};
ALIASES["unicode_xid"] = {};
ALIASES["unreachable"] = {};
ALIASES["unsafe_unwrap"] = {};
ALIASES["vec_map"] = {};
ALIASES["void"] = {};
