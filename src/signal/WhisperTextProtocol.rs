// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct SignalMessage {
    // message fields
    ratchetKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    counter: ::std::option::Option<u32>,
    previousCounter: ::std::option::Option<u32>,
    ciphertext: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignalMessage {}

impl SignalMessage {
    pub fn new() -> SignalMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignalMessage {
        static mut instance: ::protobuf::lazy::Lazy<SignalMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignalMessage,
        };
        unsafe {
            instance.get(SignalMessage::new)
        }
    }

    // optional bytes ratchetKey = 1;

    pub fn clear_ratchetKey(&mut self) {
        self.ratchetKey.clear();
    }

    pub fn has_ratchetKey(&self) -> bool {
        self.ratchetKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ratchetKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.ratchetKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ratchetKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ratchetKey.is_none() {
            self.ratchetKey.set_default();
        };
        self.ratchetKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_ratchetKey(&mut self) -> ::std::vec::Vec<u8> {
        self.ratchetKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ratchetKey(&self) -> &[u8] {
        match self.ratchetKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ratchetKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ratchetKey
    }

    fn mut_ratchetKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ratchetKey
    }

    // optional uint32 counter = 2;

    pub fn clear_counter(&mut self) {
        self.counter = ::std::option::Option::None;
    }

    pub fn has_counter(&self) -> bool {
        self.counter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter(&mut self, v: u32) {
        self.counter = ::std::option::Option::Some(v);
    }

    pub fn get_counter(&self) -> u32 {
        self.counter.unwrap_or(0)
    }

    fn get_counter_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.counter
    }

    fn mut_counter_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.counter
    }

    // optional uint32 previousCounter = 3;

    pub fn clear_previousCounter(&mut self) {
        self.previousCounter = ::std::option::Option::None;
    }

    pub fn has_previousCounter(&self) -> bool {
        self.previousCounter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previousCounter(&mut self, v: u32) {
        self.previousCounter = ::std::option::Option::Some(v);
    }

    pub fn get_previousCounter(&self) -> u32 {
        self.previousCounter.unwrap_or(0)
    }

    fn get_previousCounter_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.previousCounter
    }

    fn mut_previousCounter_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.previousCounter
    }

    // optional bytes ciphertext = 4;

    pub fn clear_ciphertext(&mut self) {
        self.ciphertext.clear();
    }

    pub fn has_ciphertext(&self) -> bool {
        self.ciphertext.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ciphertext(&mut self, v: ::std::vec::Vec<u8>) {
        self.ciphertext = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphertext(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ciphertext.is_none() {
            self.ciphertext.set_default();
        };
        self.ciphertext.as_mut().unwrap()
    }

    // Take field
    pub fn take_ciphertext(&mut self) -> ::std::vec::Vec<u8> {
        self.ciphertext.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ciphertext(&self) -> &[u8] {
        match self.ciphertext.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ciphertext_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ciphertext
    }

    fn mut_ciphertext_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ciphertext
    }
}

impl ::protobuf::Message for SignalMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ratchetKey)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.counter = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.previousCounter = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ciphertext)?;
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
        if let Some(v) = self.ratchetKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.counter {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.previousCounter {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ciphertext.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ratchetKey.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.counter {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.previousCounter {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.ciphertext.as_ref() {
            os.write_bytes(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SignalMessage {
    fn new() -> SignalMessage {
        SignalMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignalMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ratchetKey",
                    SignalMessage::get_ratchetKey_for_reflect,
                    SignalMessage::mut_ratchetKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "counter",
                    SignalMessage::get_counter_for_reflect,
                    SignalMessage::mut_counter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "previousCounter",
                    SignalMessage::get_previousCounter_for_reflect,
                    SignalMessage::mut_previousCounter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ciphertext",
                    SignalMessage::get_ciphertext_for_reflect,
                    SignalMessage::mut_ciphertext_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignalMessage>(
                    "SignalMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignalMessage {
    fn clear(&mut self) {
        self.clear_ratchetKey();
        self.clear_counter();
        self.clear_previousCounter();
        self.clear_ciphertext();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignalMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignalMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PreKeySignalMessage {
    // message fields
    registrationId: ::std::option::Option<u32>,
    preKeyId: ::std::option::Option<u32>,
    signedPreKeyId: ::std::option::Option<u32>,
    baseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    identityKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    message: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PreKeySignalMessage {}

impl PreKeySignalMessage {
    pub fn new() -> PreKeySignalMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PreKeySignalMessage {
        static mut instance: ::protobuf::lazy::Lazy<PreKeySignalMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PreKeySignalMessage,
        };
        unsafe {
            instance.get(PreKeySignalMessage::new)
        }
    }

    // optional uint32 registrationId = 5;

    pub fn clear_registrationId(&mut self) {
        self.registrationId = ::std::option::Option::None;
    }

    pub fn has_registrationId(&self) -> bool {
        self.registrationId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registrationId(&mut self, v: u32) {
        self.registrationId = ::std::option::Option::Some(v);
    }

    pub fn get_registrationId(&self) -> u32 {
        self.registrationId.unwrap_or(0)
    }

    fn get_registrationId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.registrationId
    }

    fn mut_registrationId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.registrationId
    }

    // optional uint32 preKeyId = 1;

    pub fn clear_preKeyId(&mut self) {
        self.preKeyId = ::std::option::Option::None;
    }

    pub fn has_preKeyId(&self) -> bool {
        self.preKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preKeyId(&mut self, v: u32) {
        self.preKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_preKeyId(&self) -> u32 {
        self.preKeyId.unwrap_or(0)
    }

    fn get_preKeyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.preKeyId
    }

    fn mut_preKeyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.preKeyId
    }

    // optional uint32 signedPreKeyId = 6;

    pub fn clear_signedPreKeyId(&mut self) {
        self.signedPreKeyId = ::std::option::Option::None;
    }

    pub fn has_signedPreKeyId(&self) -> bool {
        self.signedPreKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signedPreKeyId(&mut self, v: u32) {
        self.signedPreKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_signedPreKeyId(&self) -> u32 {
        self.signedPreKeyId.unwrap_or(0)
    }

    fn get_signedPreKeyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.signedPreKeyId
    }

    fn mut_signedPreKeyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.signedPreKeyId
    }

    // optional bytes baseKey = 2;

    pub fn clear_baseKey(&mut self) {
        self.baseKey.clear();
    }

    pub fn has_baseKey(&self) -> bool {
        self.baseKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.baseKey.is_none() {
            self.baseKey.set_default();
        };
        self.baseKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseKey(&mut self) -> ::std::vec::Vec<u8> {
        self.baseKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseKey(&self) -> &[u8] {
        match self.baseKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_baseKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.baseKey
    }

    fn mut_baseKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.baseKey
    }

    // optional bytes identityKey = 3;

    pub fn clear_identityKey(&mut self) {
        self.identityKey.clear();
    }

    pub fn has_identityKey(&self) -> bool {
        self.identityKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identityKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.identityKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identityKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.identityKey.is_none() {
            self.identityKey.set_default();
        };
        self.identityKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_identityKey(&mut self) -> ::std::vec::Vec<u8> {
        self.identityKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_identityKey(&self) -> &[u8] {
        match self.identityKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_identityKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.identityKey
    }

    fn mut_identityKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.identityKey
    }

    // optional bytes message = 4;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::vec::Vec<u8> {
        self.message.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_message(&self) -> &[u8] {
        match self.message.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.message
    }
}

impl ::protobuf::Message for PreKeySignalMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.registrationId = ::std::option::Option::Some(tmp);
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.preKeyId = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.signedPreKeyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.baseKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.identityKey)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.message)?;
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
        if let Some(v) = self.registrationId {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.preKeyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.signedPreKeyId {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.baseKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.identityKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.message.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.registrationId {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.preKeyId {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.signedPreKeyId {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.baseKey.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.identityKey.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.message.as_ref() {
            os.write_bytes(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PreKeySignalMessage {
    fn new() -> PreKeySignalMessage {
        PreKeySignalMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<PreKeySignalMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "registrationId",
                    PreKeySignalMessage::get_registrationId_for_reflect,
                    PreKeySignalMessage::mut_registrationId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "preKeyId",
                    PreKeySignalMessage::get_preKeyId_for_reflect,
                    PreKeySignalMessage::mut_preKeyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signedPreKeyId",
                    PreKeySignalMessage::get_signedPreKeyId_for_reflect,
                    PreKeySignalMessage::mut_signedPreKeyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "baseKey",
                    PreKeySignalMessage::get_baseKey_for_reflect,
                    PreKeySignalMessage::mut_baseKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "identityKey",
                    PreKeySignalMessage::get_identityKey_for_reflect,
                    PreKeySignalMessage::mut_identityKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message",
                    PreKeySignalMessage::get_message_for_reflect,
                    PreKeySignalMessage::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PreKeySignalMessage>(
                    "PreKeySignalMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PreKeySignalMessage {
    fn clear(&mut self) {
        self.clear_registrationId();
        self.clear_preKeyId();
        self.clear_signedPreKeyId();
        self.clear_baseKey();
        self.clear_identityKey();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PreKeySignalMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PreKeySignalMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeyExchangeMessage {
    // message fields
    id: ::std::option::Option<u32>,
    baseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ratchetKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    identityKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    baseKeySignature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyExchangeMessage {}

impl KeyExchangeMessage {
    pub fn new() -> KeyExchangeMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyExchangeMessage {
        static mut instance: ::protobuf::lazy::Lazy<KeyExchangeMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyExchangeMessage,
        };
        unsafe {
            instance.get(KeyExchangeMessage::new)
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional bytes baseKey = 2;

    pub fn clear_baseKey(&mut self) {
        self.baseKey.clear();
    }

    pub fn has_baseKey(&self) -> bool {
        self.baseKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.baseKey.is_none() {
            self.baseKey.set_default();
        };
        self.baseKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseKey(&mut self) -> ::std::vec::Vec<u8> {
        self.baseKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseKey(&self) -> &[u8] {
        match self.baseKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_baseKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.baseKey
    }

    fn mut_baseKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.baseKey
    }

    // optional bytes ratchetKey = 3;

    pub fn clear_ratchetKey(&mut self) {
        self.ratchetKey.clear();
    }

    pub fn has_ratchetKey(&self) -> bool {
        self.ratchetKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ratchetKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.ratchetKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ratchetKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ratchetKey.is_none() {
            self.ratchetKey.set_default();
        };
        self.ratchetKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_ratchetKey(&mut self) -> ::std::vec::Vec<u8> {
        self.ratchetKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ratchetKey(&self) -> &[u8] {
        match self.ratchetKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ratchetKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ratchetKey
    }

    fn mut_ratchetKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ratchetKey
    }

    // optional bytes identityKey = 4;

    pub fn clear_identityKey(&mut self) {
        self.identityKey.clear();
    }

    pub fn has_identityKey(&self) -> bool {
        self.identityKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identityKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.identityKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identityKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.identityKey.is_none() {
            self.identityKey.set_default();
        };
        self.identityKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_identityKey(&mut self) -> ::std::vec::Vec<u8> {
        self.identityKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_identityKey(&self) -> &[u8] {
        match self.identityKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_identityKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.identityKey
    }

    fn mut_identityKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.identityKey
    }

    // optional bytes baseKeySignature = 5;

    pub fn clear_baseKeySignature(&mut self) {
        self.baseKeySignature.clear();
    }

    pub fn has_baseKeySignature(&self) -> bool {
        self.baseKeySignature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseKeySignature(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseKeySignature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseKeySignature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.baseKeySignature.is_none() {
            self.baseKeySignature.set_default();
        };
        self.baseKeySignature.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseKeySignature(&mut self) -> ::std::vec::Vec<u8> {
        self.baseKeySignature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseKeySignature(&self) -> &[u8] {
        match self.baseKeySignature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_baseKeySignature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.baseKeySignature
    }

    fn mut_baseKeySignature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.baseKeySignature
    }
}

impl ::protobuf::Message for KeyExchangeMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.baseKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ratchetKey)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.identityKey)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.baseKeySignature)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.baseKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.ratchetKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.identityKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.baseKeySignature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.baseKey.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.ratchetKey.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.identityKey.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.baseKeySignature.as_ref() {
            os.write_bytes(5, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyExchangeMessage {
    fn new() -> KeyExchangeMessage {
        KeyExchangeMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyExchangeMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    KeyExchangeMessage::get_id_for_reflect,
                    KeyExchangeMessage::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "baseKey",
                    KeyExchangeMessage::get_baseKey_for_reflect,
                    KeyExchangeMessage::mut_baseKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ratchetKey",
                    KeyExchangeMessage::get_ratchetKey_for_reflect,
                    KeyExchangeMessage::mut_ratchetKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "identityKey",
                    KeyExchangeMessage::get_identityKey_for_reflect,
                    KeyExchangeMessage::mut_identityKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "baseKeySignature",
                    KeyExchangeMessage::get_baseKeySignature_for_reflect,
                    KeyExchangeMessage::mut_baseKeySignature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyExchangeMessage>(
                    "KeyExchangeMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyExchangeMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_baseKey();
        self.clear_ratchetKey();
        self.clear_identityKey();
        self.clear_baseKeySignature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyExchangeMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyExchangeMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyMessage {
    // message fields
    id: ::std::option::Option<u32>,
    iteration: ::std::option::Option<u32>,
    ciphertext: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyMessage {}

impl SenderKeyMessage {
    pub fn new() -> SenderKeyMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyMessage {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyMessage,
        };
        unsafe {
            instance.get(SenderKeyMessage::new)
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional uint32 iteration = 2;

    pub fn clear_iteration(&mut self) {
        self.iteration = ::std::option::Option::None;
    }

    pub fn has_iteration(&self) -> bool {
        self.iteration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iteration(&mut self, v: u32) {
        self.iteration = ::std::option::Option::Some(v);
    }

    pub fn get_iteration(&self) -> u32 {
        self.iteration.unwrap_or(0)
    }

    fn get_iteration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.iteration
    }

    fn mut_iteration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.iteration
    }

    // optional bytes ciphertext = 3;

    pub fn clear_ciphertext(&mut self) {
        self.ciphertext.clear();
    }

    pub fn has_ciphertext(&self) -> bool {
        self.ciphertext.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ciphertext(&mut self, v: ::std::vec::Vec<u8>) {
        self.ciphertext = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphertext(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ciphertext.is_none() {
            self.ciphertext.set_default();
        };
        self.ciphertext.as_mut().unwrap()
    }

    // Take field
    pub fn take_ciphertext(&mut self) -> ::std::vec::Vec<u8> {
        self.ciphertext.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ciphertext(&self) -> &[u8] {
        match self.ciphertext.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ciphertext_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ciphertext
    }

    fn mut_ciphertext_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ciphertext
    }
}

impl ::protobuf::Message for SenderKeyMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.iteration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ciphertext)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.iteration {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ciphertext.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.iteration {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.ciphertext.as_ref() {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SenderKeyMessage {
    fn new() -> SenderKeyMessage {
        SenderKeyMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    SenderKeyMessage::get_id_for_reflect,
                    SenderKeyMessage::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "iteration",
                    SenderKeyMessage::get_iteration_for_reflect,
                    SenderKeyMessage::mut_iteration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ciphertext",
                    SenderKeyMessage::get_ciphertext_for_reflect,
                    SenderKeyMessage::mut_ciphertext_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyMessage>(
                    "SenderKeyMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_iteration();
        self.clear_ciphertext();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyDistributionMessage {
    // message fields
    id: ::std::option::Option<u32>,
    iteration: ::std::option::Option<u32>,
    chainKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signingKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyDistributionMessage {}

impl SenderKeyDistributionMessage {
    pub fn new() -> SenderKeyDistributionMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyDistributionMessage {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyDistributionMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyDistributionMessage,
        };
        unsafe {
            instance.get(SenderKeyDistributionMessage::new)
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional uint32 iteration = 2;

    pub fn clear_iteration(&mut self) {
        self.iteration = ::std::option::Option::None;
    }

    pub fn has_iteration(&self) -> bool {
        self.iteration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iteration(&mut self, v: u32) {
        self.iteration = ::std::option::Option::Some(v);
    }

    pub fn get_iteration(&self) -> u32 {
        self.iteration.unwrap_or(0)
    }

    fn get_iteration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.iteration
    }

    fn mut_iteration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.iteration
    }

    // optional bytes chainKey = 3;

    pub fn clear_chainKey(&mut self) {
        self.chainKey.clear();
    }

    pub fn has_chainKey(&self) -> bool {
        self.chainKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chainKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.chainKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chainKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.chainKey.is_none() {
            self.chainKey.set_default();
        };
        self.chainKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_chainKey(&mut self) -> ::std::vec::Vec<u8> {
        self.chainKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_chainKey(&self) -> &[u8] {
        match self.chainKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_chainKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.chainKey
    }

    fn mut_chainKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.chainKey
    }

    // optional bytes signingKey = 4;

    pub fn clear_signingKey(&mut self) {
        self.signingKey.clear();
    }

    pub fn has_signingKey(&self) -> bool {
        self.signingKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signingKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.signingKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signingKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signingKey.is_none() {
            self.signingKey.set_default();
        };
        self.signingKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_signingKey(&mut self) -> ::std::vec::Vec<u8> {
        self.signingKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signingKey(&self) -> &[u8] {
        match self.signingKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signingKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signingKey
    }

    fn mut_signingKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signingKey
    }
}

impl ::protobuf::Message for SenderKeyDistributionMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.iteration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.chainKey)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signingKey)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.iteration {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.chainKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.signingKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.iteration {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.chainKey.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.signingKey.as_ref() {
            os.write_bytes(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SenderKeyDistributionMessage {
    fn new() -> SenderKeyDistributionMessage {
        SenderKeyDistributionMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyDistributionMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    SenderKeyDistributionMessage::get_id_for_reflect,
                    SenderKeyDistributionMessage::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "iteration",
                    SenderKeyDistributionMessage::get_iteration_for_reflect,
                    SenderKeyDistributionMessage::mut_iteration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "chainKey",
                    SenderKeyDistributionMessage::get_chainKey_for_reflect,
                    SenderKeyDistributionMessage::mut_chainKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signingKey",
                    SenderKeyDistributionMessage::get_signingKey_for_reflect,
                    SenderKeyDistributionMessage::mut_signingKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyDistributionMessage>(
                    "SenderKeyDistributionMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyDistributionMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_iteration();
        self.clear_chainKey();
        self.clear_signingKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyDistributionMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyDistributionMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeviceConsistencyCodeMessage {
    // message fields
    generation: ::std::option::Option<u32>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceConsistencyCodeMessage {}

impl DeviceConsistencyCodeMessage {
    pub fn new() -> DeviceConsistencyCodeMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceConsistencyCodeMessage {
        static mut instance: ::protobuf::lazy::Lazy<DeviceConsistencyCodeMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceConsistencyCodeMessage,
        };
        unsafe {
            instance.get(DeviceConsistencyCodeMessage::new)
        }
    }

    // optional uint32 generation = 1;

    pub fn clear_generation(&mut self) {
        self.generation = ::std::option::Option::None;
    }

    pub fn has_generation(&self) -> bool {
        self.generation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generation(&mut self, v: u32) {
        self.generation = ::std::option::Option::Some(v);
    }

    pub fn get_generation(&self) -> u32 {
        self.generation.unwrap_or(0)
    }

    fn get_generation_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.generation
    }

    fn mut_generation_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.generation
    }

    // optional bytes signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        };
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }
}

impl ::protobuf::Message for DeviceConsistencyCodeMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.generation = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
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
        if let Some(v) = self.generation {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.generation {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.signature.as_ref() {
            os.write_bytes(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeviceConsistencyCodeMessage {
    fn new() -> DeviceConsistencyCodeMessage {
        DeviceConsistencyCodeMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeviceConsistencyCodeMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "generation",
                    DeviceConsistencyCodeMessage::get_generation_for_reflect,
                    DeviceConsistencyCodeMessage::mut_generation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    DeviceConsistencyCodeMessage::get_signature_for_reflect,
                    DeviceConsistencyCodeMessage::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeviceConsistencyCodeMessage>(
                    "DeviceConsistencyCodeMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeviceConsistencyCodeMessage {
    fn clear(&mut self) {
        self.clear_generation();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceConsistencyCodeMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceConsistencyCodeMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x22, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x57, 0x68, 0x69, 0x73, 0x70,
    0x65, 0x72, 0x54, 0x65, 0x78, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63, 0x75, 0x72, 0x65,
    0x22, 0x93, 0x01, 0x0a, 0x0d, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x72, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x72, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b,
    0x65, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x28, 0x0a, 0x0f,
    0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72,
    0x74, 0x65, 0x78, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x63, 0x69, 0x70, 0x68,
    0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x22, 0xd7, 0x01, 0x0a, 0x13, 0x50, 0x72, 0x65, 0x4b, 0x65,
    0x79, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x26,
    0x0a, 0x0e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x4b, 0x65, 0x79,
    0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x70, 0x72, 0x65, 0x4b, 0x65, 0x79,
    0x49, 0x64, 0x12, 0x26, 0x0a, 0x0e, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x72, 0x65, 0x4b,
    0x65, 0x79, 0x49, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x73, 0x69, 0x67, 0x6e,
    0x65, 0x64, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x62, 0x61,
    0x73, 0x65, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x62, 0x61, 0x73,
    0x65, 0x4b, 0x65, 0x79, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x69, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x74, 0x79, 0x4b, 0x65, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x22, 0xac, 0x01, 0x0a, 0x12, 0x4b, 0x65, 0x79, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x62, 0x61, 0x73, 0x65, 0x4b,
    0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x62, 0x61, 0x73, 0x65, 0x4b, 0x65,
    0x79, 0x12, 0x1e, 0x0a, 0x0a, 0x72, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x72, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65,
    0x79, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4b, 0x65, 0x79,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x4b, 0x65, 0x79, 0x12, 0x2a, 0x0a, 0x10, 0x62, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x53, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x10, 0x62,
    0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x22,
    0x60, 0x0a, 0x10, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x02, 0x69, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x74, 0x65, 0x78,
    0x74, 0x22, 0x88, 0x01, 0x0a, 0x1c, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x44,
    0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x02,
    0x69, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x1a, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x12, 0x1e, 0x0a, 0x0a,
    0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x22, 0x5c, 0x0a, 0x1c,
    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63,
    0x79, 0x43, 0x6f, 0x64, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1e, 0x0a, 0x0a,
    0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1c, 0x0a, 0x09,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x42, 0x35, 0x0a, 0x25, 0x6f, 0x72,
    0x67, 0x2e, 0x77, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x73,
    0x2e, 0x6c, 0x69, 0x62, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x42, 0x0c, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x4a, 0xcf, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2d, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x00, 0x08, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x02, 0x00, 0x3e,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x02, 0x00, 0x3e, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x02, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x02, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x02, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x07, 0x12, 0x03, 0x02, 0x16, 0x3d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x03,
    0x00, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x03, 0x00, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x03, 0x07, 0x1b, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x03, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x03, 0x1e, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x05, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x06, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x07, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x07,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x12, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x08, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x09, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00,
    0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0d, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x0f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x23, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x10, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x10, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10,
    0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x11, 0x02, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x11, 0x23, 0x24, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12,
    0x03, 0x12, 0x02, 0x25, 0x22, 0x0f, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x12, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x12, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x12, 0x23, 0x24, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x15, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x15, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x16,
    0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x17, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x12,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x18, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x18, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x02,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x19, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x19, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x1a, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1a,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1a, 0x25, 0x26, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1d, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x1d, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x1e, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x1f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x20, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x20, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x20, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x23, 0x00, 0x28,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x23, 0x08, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x24, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x25, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12,
    0x03, 0x26, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x27, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x27, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x1f,
    0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2a, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x2b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2c, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2c, 0x1f, 0x20,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
