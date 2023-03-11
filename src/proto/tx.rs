// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct TxRaw {
    // message fields
    pub body_bytes: ::std::vec::Vec<u8>,
    pub auth_info_bytes: ::std::vec::Vec<u8>,
    pub signatures: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TxRaw {
    fn default() -> &'a TxRaw {
        <TxRaw as ::protobuf::Message>::default_instance()
    }
}

impl TxRaw {
    pub fn new() -> TxRaw {
        ::std::default::Default::default()
    }

    // bytes body_bytes = 1;


    pub fn get_body_bytes(&self) -> &[u8] {
        &self.body_bytes
    }
    pub fn clear_body_bytes(&mut self) {
        self.body_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_body_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.body_bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.body_bytes
    }

    // Take field
    pub fn take_body_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.body_bytes, ::std::vec::Vec::new())
    }

    // bytes auth_info_bytes = 2;


    pub fn get_auth_info_bytes(&self) -> &[u8] {
        &self.auth_info_bytes
    }
    pub fn clear_auth_info_bytes(&mut self) {
        self.auth_info_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_auth_info_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.auth_info_bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_info_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.auth_info_bytes
    }

    // Take field
    pub fn take_auth_info_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.auth_info_bytes, ::std::vec::Vec::new())
    }

    // repeated bytes signatures = 3;


    pub fn get_signatures(&self) -> &[::std::vec::Vec<u8>] {
        &self.signatures
    }
    pub fn clear_signatures(&mut self) {
        self.signatures.clear();
    }

    // Param is passed by value, moved
    pub fn set_signatures(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.signatures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signatures(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.signatures
    }

    // Take field
    pub fn take_signatures(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.signatures, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TxRaw {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.body_bytes)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.auth_info_bytes)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.signatures)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.body_bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.body_bytes);
        }
        if !self.auth_info_bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.auth_info_bytes);
        }
        for value in &self.signatures {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.body_bytes.is_empty() {
            os.write_bytes(1, &self.body_bytes)?;
        }
        if !self.auth_info_bytes.is_empty() {
            os.write_bytes(2, &self.auth_info_bytes)?;
        }
        for v in &self.signatures {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TxRaw {
        TxRaw::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "body_bytes",
                |m: &TxRaw| { &m.body_bytes },
                |m: &mut TxRaw| { &mut m.body_bytes },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "auth_info_bytes",
                |m: &TxRaw| { &m.auth_info_bytes },
                |m: &mut TxRaw| { &mut m.auth_info_bytes },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "signatures",
                |m: &TxRaw| { &m.signatures },
                |m: &mut TxRaw| { &mut m.signatures },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TxRaw>(
                "TxRaw",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TxRaw {
        static instance: ::protobuf::rt::LazyV2<TxRaw> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TxRaw::new)
    }
}

impl ::protobuf::Clear for TxRaw {
    fn clear(&mut self) {
        self.body_bytes.clear();
        self.auth_info_bytes.clear();
        self.signatures.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxRaw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxRaw {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TxBody {
    // message fields
    pub messages: ::protobuf::RepeatedField<Message>,
    pub memo: ::std::string::String,
    pub timeout_height: u64,
    pub extension_options: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    pub non_critical_extension_options: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TxBody {
    fn default() -> &'a TxBody {
        <TxBody as ::protobuf::Message>::default_instance()
    }
}

impl TxBody {
    pub fn new() -> TxBody {
        ::std::default::Default::default()
    }

    // repeated .metro.pb.Message messages = 1;


    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages(&mut self, v: ::protobuf::RepeatedField<Message>) {
        self.messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages(&mut self) -> &mut ::protobuf::RepeatedField<Message> {
        &mut self.messages
    }

    // Take field
    pub fn take_messages(&mut self) -> ::protobuf::RepeatedField<Message> {
        ::std::mem::replace(&mut self.messages, ::protobuf::RepeatedField::new())
    }

    // string memo = 2;


    pub fn get_memo(&self) -> &str {
        &self.memo
    }
    pub fn clear_memo(&mut self) {
        self.memo.clear();
    }

    // Param is passed by value, moved
    pub fn set_memo(&mut self, v: ::std::string::String) {
        self.memo = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_memo(&mut self) -> &mut ::std::string::String {
        &mut self.memo
    }

    // Take field
    pub fn take_memo(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.memo, ::std::string::String::new())
    }

    // uint64 timeout_height = 3;


    pub fn get_timeout_height(&self) -> u64 {
        self.timeout_height
    }
    pub fn clear_timeout_height(&mut self) {
        self.timeout_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeout_height(&mut self, v: u64) {
        self.timeout_height = v;
    }

    // repeated .google.protobuf.Any extension_options = 1023;


    pub fn get_extension_options(&self) -> &[::protobuf::well_known_types::Any] {
        &self.extension_options
    }
    pub fn clear_extension_options(&mut self) {
        self.extension_options.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension_options(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.extension_options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension_options(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.extension_options
    }

    // Take field
    pub fn take_extension_options(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.extension_options, ::protobuf::RepeatedField::new())
    }

    // repeated .google.protobuf.Any non_critical_extension_options = 2047;


    pub fn get_non_critical_extension_options(&self) -> &[::protobuf::well_known_types::Any] {
        &self.non_critical_extension_options
    }
    pub fn clear_non_critical_extension_options(&mut self) {
        self.non_critical_extension_options.clear();
    }

    // Param is passed by value, moved
    pub fn set_non_critical_extension_options(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.non_critical_extension_options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_non_critical_extension_options(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.non_critical_extension_options
    }

    // Take field
    pub fn take_non_critical_extension_options(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.non_critical_extension_options, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TxBody {
    fn is_initialized(&self) -> bool {
        for v in &self.messages {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.extension_options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.non_critical_extension_options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.memo)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timeout_height = tmp;
                },
                1023 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extension_options)?;
                },
                2047 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.non_critical_extension_options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.messages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.memo.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.memo);
        }
        if self.timeout_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.timeout_height, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.extension_options {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.non_critical_extension_options {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.messages {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.memo.is_empty() {
            os.write_string(2, &self.memo)?;
        }
        if self.timeout_height != 0 {
            os.write_uint64(3, self.timeout_height)?;
        }
        for v in &self.extension_options {
            os.write_tag(1023, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.non_critical_extension_options {
            os.write_tag(2047, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TxBody {
        TxBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message>>(
                "messages",
                |m: &TxBody| { &m.messages },
                |m: &mut TxBody| { &mut m.messages },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "memo",
                |m: &TxBody| { &m.memo },
                |m: &mut TxBody| { &mut m.memo },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "timeout_height",
                |m: &TxBody| { &m.timeout_height },
                |m: &mut TxBody| { &mut m.timeout_height },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "extension_options",
                |m: &TxBody| { &m.extension_options },
                |m: &mut TxBody| { &mut m.extension_options },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "non_critical_extension_options",
                |m: &TxBody| { &m.non_critical_extension_options },
                |m: &mut TxBody| { &mut m.non_critical_extension_options },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TxBody>(
                "TxBody",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TxBody {
        static instance: ::protobuf::rt::LazyV2<TxBody> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TxBody::new)
    }
}

impl ::protobuf::Clear for TxBody {
    fn clear(&mut self) {
        self.messages.clear();
        self.memo.clear();
        self.timeout_height = 0;
        self.extension_options.clear();
        self.non_critical_extension_options.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxBody {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub type_url: ::std::string::String,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // string type_url = 1;


    pub fn get_type_url(&self) -> &str {
        &self.type_url
    }
    pub fn clear_type_url(&mut self) {
        self.type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_url(&mut self, v: ::std::string::String) {
        self.type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // Take field
    pub fn take_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_url, ::std::string::String::new())
    }

    // bytes value = 2;


    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_url)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.type_url);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.type_url.is_empty() {
            os.write_string(1, &self.type_url)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type_url",
                |m: &Message| { &m.type_url },
                |m: &mut Message| { &mut m.type_url },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "value",
                |m: &Message| { &m.value },
                |m: &mut Message| { &mut m.value },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message>(
                "Message",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message {
        static instance: ::protobuf::rt::LazyV2<Message> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message::new)
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.type_url.clear();
        self.value.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08tx.proto\x12\x08metro.pb\x1a\x19google/protobuf/any.proto\"v\n\x05\
    TxRaw\x12\x1f\n\nbody_bytes\x18\x01\x20\x01(\x0cR\tbodyBytesB\0\x12(\n\
    \x0fauth_info_bytes\x18\x02\x20\x01(\x0cR\rauthInfoBytesB\0\x12\x20\n\ns\
    ignatures\x18\x03\x20\x03(\x0cR\nsignaturesB\0:\0\"\x9e\x02\n\x06TxBody\
    \x12/\n\x08messages\x18\x01\x20\x03(\x0b2\x11.metro.pb.MessageR\x08messa\
    gesB\0\x12\x14\n\x04memo\x18\x02\x20\x01(\tR\x04memoB\0\x12'\n\x0etimeou\
    t_height\x18\x03\x20\x01(\x04R\rtimeoutHeightB\0\x12D\n\x11extension_opt\
    ions\x18\xff\x07\x20\x03(\x0b2\x14.google.protobuf.AnyR\x10extensionOpti\
    onsB\0\x12\\\n\x1enon_critical_extension_options\x18\xff\x0f\x20\x03(\
    \x0b2\x14.google.protobuf.AnyR\x1bnonCriticalExtensionOptionsB\0:\0\"@\n\
    \x07Message\x12\x1b\n\x08type_url\x18\x01\x20\x01(\tR\x07typeUrlB\0\x12\
    \x16\n\x05value\x18\x02\x20\x01(\x0cR\x05valueB\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}