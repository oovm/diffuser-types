#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;   #[allow(unused_macros)] macro_rules! try { ( $   __expr   :   expr   ) =>   { match   $   __expr   { _serde   ::   __private   ::   Ok   ( __val   ) =>   __val   ,   _serde   ::   __private   ::   Err   ( __err   ) =>   { return   _serde   ::   __private   ::   Err   ( __err   ) ;   } } } } #[automatically_derived]
    impl _serde::Serialize for DiffuserPrompts {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error> where __S: _serde::Serializer, {
            let mut __serde_state = match (_serde::Serializer::serialize_struct(__serializer, "DiffuserPrompts", false as usize + 1)) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
            };
            match (_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "tags", &self.tags)) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
            }
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};