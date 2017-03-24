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
pub struct Envelope {
    // message fields
    field_type: ::std::option::Option<Envelope_Type>,
    source: ::protobuf::SingularField<::std::string::String>,
    sourceDevice: ::std::option::Option<u32>,
    relay: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<u64>,
    legacyMessage: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Envelope {}

impl Envelope {
    pub fn new() -> Envelope {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Envelope {
        static mut instance: ::protobuf::lazy::Lazy<Envelope> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Envelope,
        };
        unsafe {
            instance.get(Envelope::new)
        }
    }

    // optional .signalservice.Envelope.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Envelope_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Envelope_Type {
        self.field_type.unwrap_or(Envelope_Type::UNKNOWN)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<Envelope_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<Envelope_Type> {
        &mut self.field_type
    }

    // optional string source = 2;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        if self.source.is_none() {
            self.source.set_default();
        };
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        self.source.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_source(&self) -> &str {
        match self.source.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.source
    }

    // optional uint32 sourceDevice = 7;

    pub fn clear_sourceDevice(&mut self) {
        self.sourceDevice = ::std::option::Option::None;
    }

    pub fn has_sourceDevice(&self) -> bool {
        self.sourceDevice.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sourceDevice(&mut self, v: u32) {
        self.sourceDevice = ::std::option::Option::Some(v);
    }

    pub fn get_sourceDevice(&self) -> u32 {
        self.sourceDevice.unwrap_or(0)
    }

    fn get_sourceDevice_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sourceDevice
    }

    fn mut_sourceDevice_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sourceDevice
    }

    // optional string relay = 3;

    pub fn clear_relay(&mut self) {
        self.relay.clear();
    }

    pub fn has_relay(&self) -> bool {
        self.relay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay(&mut self, v: ::std::string::String) {
        self.relay = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_relay(&mut self) -> &mut ::std::string::String {
        if self.relay.is_none() {
            self.relay.set_default();
        };
        self.relay.as_mut().unwrap()
    }

    // Take field
    pub fn take_relay(&mut self) -> ::std::string::String {
        self.relay.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_relay(&self) -> &str {
        match self.relay.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_relay_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.relay
    }

    fn mut_relay_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.relay
    }

    // optional uint64 timestamp = 5;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }

    // optional bytes legacyMessage = 6;

    pub fn clear_legacyMessage(&mut self) {
        self.legacyMessage.clear();
    }

    pub fn has_legacyMessage(&self) -> bool {
        self.legacyMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacyMessage(&mut self, v: ::std::vec::Vec<u8>) {
        self.legacyMessage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_legacyMessage(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.legacyMessage.is_none() {
            self.legacyMessage.set_default();
        };
        self.legacyMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_legacyMessage(&mut self) -> ::std::vec::Vec<u8> {
        self.legacyMessage.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_legacyMessage(&self) -> &[u8] {
        match self.legacyMessage.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_legacyMessage_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.legacyMessage
    }

    fn mut_legacyMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.legacyMessage
    }

    // optional bytes content = 8;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        self.content.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        match self.content.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.content
    }
}

impl ::protobuf::Message for Envelope {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.source)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sourceDevice = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.relay)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.legacyMessage)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.source.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.sourceDevice {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.relay.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.legacyMessage.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        if let Some(v) = self.content.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.source.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.sourceDevice {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.relay.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.legacyMessage.as_ref() {
            os.write_bytes(6, &v)?;
        };
        if let Some(v) = self.content.as_ref() {
            os.write_bytes(8, &v)?;
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

impl ::protobuf::MessageStatic for Envelope {
    fn new() -> Envelope {
        Envelope::new()
    }

    fn descriptor_static(_: ::std::option::Option<Envelope>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Envelope_Type>>(
                    "type",
                    Envelope::get_field_type_for_reflect,
                    Envelope::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source",
                    Envelope::get_source_for_reflect,
                    Envelope::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sourceDevice",
                    Envelope::get_sourceDevice_for_reflect,
                    Envelope::mut_sourceDevice_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "relay",
                    Envelope::get_relay_for_reflect,
                    Envelope::mut_relay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    Envelope::get_timestamp_for_reflect,
                    Envelope::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "legacyMessage",
                    Envelope::get_legacyMessage_for_reflect,
                    Envelope::mut_legacyMessage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    Envelope::get_content_for_reflect,
                    Envelope::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Envelope>(
                    "Envelope",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Envelope {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_source();
        self.clear_sourceDevice();
        self.clear_relay();
        self.clear_timestamp();
        self.clear_legacyMessage();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Envelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Envelope {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Envelope_Type {
    UNKNOWN = 0,
    CIPHERTEXT = 1,
    KEY_EXCHANGE = 2,
    PREKEY_BUNDLE = 3,
    RECEIPT = 5,
}

impl ::protobuf::ProtobufEnum for Envelope_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Envelope_Type> {
        match value {
            0 => ::std::option::Option::Some(Envelope_Type::UNKNOWN),
            1 => ::std::option::Option::Some(Envelope_Type::CIPHERTEXT),
            2 => ::std::option::Option::Some(Envelope_Type::KEY_EXCHANGE),
            3 => ::std::option::Option::Some(Envelope_Type::PREKEY_BUNDLE),
            5 => ::std::option::Option::Some(Envelope_Type::RECEIPT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Envelope_Type] = &[
            Envelope_Type::UNKNOWN,
            Envelope_Type::CIPHERTEXT,
            Envelope_Type::KEY_EXCHANGE,
            Envelope_Type::PREKEY_BUNDLE,
            Envelope_Type::RECEIPT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Envelope_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Envelope_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Envelope_Type {
}

impl ::protobuf::reflect::ProtobufValue for Envelope_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Content {
    // message fields
    dataMessage: ::protobuf::SingularPtrField<DataMessage>,
    syncMessage: ::protobuf::SingularPtrField<SyncMessage>,
    callMessage: ::protobuf::SingularPtrField<CallMessage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Content {}

impl Content {
    pub fn new() -> Content {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Content {
        static mut instance: ::protobuf::lazy::Lazy<Content> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Content,
        };
        unsafe {
            instance.get(Content::new)
        }
    }

    // optional .signalservice.DataMessage dataMessage = 1;

    pub fn clear_dataMessage(&mut self) {
        self.dataMessage.clear();
    }

    pub fn has_dataMessage(&self) -> bool {
        self.dataMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dataMessage(&mut self, v: DataMessage) {
        self.dataMessage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dataMessage(&mut self) -> &mut DataMessage {
        if self.dataMessage.is_none() {
            self.dataMessage.set_default();
        };
        self.dataMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_dataMessage(&mut self) -> DataMessage {
        self.dataMessage.take().unwrap_or_else(|| DataMessage::new())
    }

    pub fn get_dataMessage(&self) -> &DataMessage {
        self.dataMessage.as_ref().unwrap_or_else(|| DataMessage::default_instance())
    }

    fn get_dataMessage_for_reflect(&self) -> &::protobuf::SingularPtrField<DataMessage> {
        &self.dataMessage
    }

    fn mut_dataMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataMessage> {
        &mut self.dataMessage
    }

    // optional .signalservice.SyncMessage syncMessage = 2;

    pub fn clear_syncMessage(&mut self) {
        self.syncMessage.clear();
    }

    pub fn has_syncMessage(&self) -> bool {
        self.syncMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncMessage(&mut self, v: SyncMessage) {
        self.syncMessage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_syncMessage(&mut self) -> &mut SyncMessage {
        if self.syncMessage.is_none() {
            self.syncMessage.set_default();
        };
        self.syncMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_syncMessage(&mut self) -> SyncMessage {
        self.syncMessage.take().unwrap_or_else(|| SyncMessage::new())
    }

    pub fn get_syncMessage(&self) -> &SyncMessage {
        self.syncMessage.as_ref().unwrap_or_else(|| SyncMessage::default_instance())
    }

    fn get_syncMessage_for_reflect(&self) -> &::protobuf::SingularPtrField<SyncMessage> {
        &self.syncMessage
    }

    fn mut_syncMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SyncMessage> {
        &mut self.syncMessage
    }

    // optional .signalservice.CallMessage callMessage = 3;

    pub fn clear_callMessage(&mut self) {
        self.callMessage.clear();
    }

    pub fn has_callMessage(&self) -> bool {
        self.callMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_callMessage(&mut self, v: CallMessage) {
        self.callMessage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_callMessage(&mut self) -> &mut CallMessage {
        if self.callMessage.is_none() {
            self.callMessage.set_default();
        };
        self.callMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_callMessage(&mut self) -> CallMessage {
        self.callMessage.take().unwrap_or_else(|| CallMessage::new())
    }

    pub fn get_callMessage(&self) -> &CallMessage {
        self.callMessage.as_ref().unwrap_or_else(|| CallMessage::default_instance())
    }

    fn get_callMessage_for_reflect(&self) -> &::protobuf::SingularPtrField<CallMessage> {
        &self.callMessage
    }

    fn mut_callMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CallMessage> {
        &mut self.callMessage
    }
}

impl ::protobuf::Message for Content {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dataMessage)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.syncMessage)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.callMessage)?;
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
        if let Some(v) = self.dataMessage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.syncMessage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.callMessage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dataMessage.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.syncMessage.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.callMessage.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Content {
    fn new() -> Content {
        Content::new()
    }

    fn descriptor_static(_: ::std::option::Option<Content>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataMessage>>(
                    "dataMessage",
                    Content::get_dataMessage_for_reflect,
                    Content::mut_dataMessage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage>>(
                    "syncMessage",
                    Content::get_syncMessage_for_reflect,
                    Content::mut_syncMessage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CallMessage>>(
                    "callMessage",
                    Content::get_callMessage_for_reflect,
                    Content::mut_callMessage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Content>(
                    "Content",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Content {
    fn clear(&mut self) {
        self.clear_dataMessage();
        self.clear_syncMessage();
        self.clear_callMessage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Content {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Content {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallMessage {
    // message fields
    offer: ::protobuf::SingularPtrField<CallMessage_Offer>,
    answer: ::protobuf::SingularPtrField<CallMessage_Answer>,
    iceUpdate: ::protobuf::RepeatedField<CallMessage_IceUpdate>,
    hangup: ::protobuf::SingularPtrField<CallMessage_Hangup>,
    busy: ::protobuf::SingularPtrField<CallMessage_Busy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CallMessage {}

impl CallMessage {
    pub fn new() -> CallMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CallMessage {
        static mut instance: ::protobuf::lazy::Lazy<CallMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CallMessage,
        };
        unsafe {
            instance.get(CallMessage::new)
        }
    }

    // optional .signalservice.CallMessage.Offer offer = 1;

    pub fn clear_offer(&mut self) {
        self.offer.clear();
    }

    pub fn has_offer(&self) -> bool {
        self.offer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offer(&mut self, v: CallMessage_Offer) {
        self.offer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_offer(&mut self) -> &mut CallMessage_Offer {
        if self.offer.is_none() {
            self.offer.set_default();
        };
        self.offer.as_mut().unwrap()
    }

    // Take field
    pub fn take_offer(&mut self) -> CallMessage_Offer {
        self.offer.take().unwrap_or_else(|| CallMessage_Offer::new())
    }

    pub fn get_offer(&self) -> &CallMessage_Offer {
        self.offer.as_ref().unwrap_or_else(|| CallMessage_Offer::default_instance())
    }

    fn get_offer_for_reflect(&self) -> &::protobuf::SingularPtrField<CallMessage_Offer> {
        &self.offer
    }

    fn mut_offer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CallMessage_Offer> {
        &mut self.offer
    }

    // optional .signalservice.CallMessage.Answer answer = 2;

    pub fn clear_answer(&mut self) {
        self.answer.clear();
    }

    pub fn has_answer(&self) -> bool {
        self.answer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_answer(&mut self, v: CallMessage_Answer) {
        self.answer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_answer(&mut self) -> &mut CallMessage_Answer {
        if self.answer.is_none() {
            self.answer.set_default();
        };
        self.answer.as_mut().unwrap()
    }

    // Take field
    pub fn take_answer(&mut self) -> CallMessage_Answer {
        self.answer.take().unwrap_or_else(|| CallMessage_Answer::new())
    }

    pub fn get_answer(&self) -> &CallMessage_Answer {
        self.answer.as_ref().unwrap_or_else(|| CallMessage_Answer::default_instance())
    }

    fn get_answer_for_reflect(&self) -> &::protobuf::SingularPtrField<CallMessage_Answer> {
        &self.answer
    }

    fn mut_answer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CallMessage_Answer> {
        &mut self.answer
    }

    // repeated .signalservice.CallMessage.IceUpdate iceUpdate = 3;

    pub fn clear_iceUpdate(&mut self) {
        self.iceUpdate.clear();
    }

    // Param is passed by value, moved
    pub fn set_iceUpdate(&mut self, v: ::protobuf::RepeatedField<CallMessage_IceUpdate>) {
        self.iceUpdate = v;
    }

    // Mutable pointer to the field.
    pub fn mut_iceUpdate(&mut self) -> &mut ::protobuf::RepeatedField<CallMessage_IceUpdate> {
        &mut self.iceUpdate
    }

    // Take field
    pub fn take_iceUpdate(&mut self) -> ::protobuf::RepeatedField<CallMessage_IceUpdate> {
        ::std::mem::replace(&mut self.iceUpdate, ::protobuf::RepeatedField::new())
    }

    pub fn get_iceUpdate(&self) -> &[CallMessage_IceUpdate] {
        &self.iceUpdate
    }

    fn get_iceUpdate_for_reflect(&self) -> &::protobuf::RepeatedField<CallMessage_IceUpdate> {
        &self.iceUpdate
    }

    fn mut_iceUpdate_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CallMessage_IceUpdate> {
        &mut self.iceUpdate
    }

    // optional .signalservice.CallMessage.Hangup hangup = 4;

    pub fn clear_hangup(&mut self) {
        self.hangup.clear();
    }

    pub fn has_hangup(&self) -> bool {
        self.hangup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hangup(&mut self, v: CallMessage_Hangup) {
        self.hangup = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hangup(&mut self) -> &mut CallMessage_Hangup {
        if self.hangup.is_none() {
            self.hangup.set_default();
        };
        self.hangup.as_mut().unwrap()
    }

    // Take field
    pub fn take_hangup(&mut self) -> CallMessage_Hangup {
        self.hangup.take().unwrap_or_else(|| CallMessage_Hangup::new())
    }

    pub fn get_hangup(&self) -> &CallMessage_Hangup {
        self.hangup.as_ref().unwrap_or_else(|| CallMessage_Hangup::default_instance())
    }

    fn get_hangup_for_reflect(&self) -> &::protobuf::SingularPtrField<CallMessage_Hangup> {
        &self.hangup
    }

    fn mut_hangup_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CallMessage_Hangup> {
        &mut self.hangup
    }

    // optional .signalservice.CallMessage.Busy busy = 5;

    pub fn clear_busy(&mut self) {
        self.busy.clear();
    }

    pub fn has_busy(&self) -> bool {
        self.busy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_busy(&mut self, v: CallMessage_Busy) {
        self.busy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_busy(&mut self) -> &mut CallMessage_Busy {
        if self.busy.is_none() {
            self.busy.set_default();
        };
        self.busy.as_mut().unwrap()
    }

    // Take field
    pub fn take_busy(&mut self) -> CallMessage_Busy {
        self.busy.take().unwrap_or_else(|| CallMessage_Busy::new())
    }

    pub fn get_busy(&self) -> &CallMessage_Busy {
        self.busy.as_ref().unwrap_or_else(|| CallMessage_Busy::default_instance())
    }

    fn get_busy_for_reflect(&self) -> &::protobuf::SingularPtrField<CallMessage_Busy> {
        &self.busy
    }

    fn mut_busy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CallMessage_Busy> {
        &mut self.busy
    }
}

impl ::protobuf::Message for CallMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.offer)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.answer)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.iceUpdate)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hangup)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.busy)?;
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
        if let Some(v) = self.offer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.answer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.iceUpdate {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hangup.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.busy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offer.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.answer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.iceUpdate {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hangup.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.busy.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CallMessage {
    fn new() -> CallMessage {
        CallMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CallMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CallMessage_Offer>>(
                    "offer",
                    CallMessage::get_offer_for_reflect,
                    CallMessage::mut_offer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CallMessage_Answer>>(
                    "answer",
                    CallMessage::get_answer_for_reflect,
                    CallMessage::mut_answer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CallMessage_IceUpdate>>(
                    "iceUpdate",
                    CallMessage::get_iceUpdate_for_reflect,
                    CallMessage::mut_iceUpdate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CallMessage_Hangup>>(
                    "hangup",
                    CallMessage::get_hangup_for_reflect,
                    CallMessage::mut_hangup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CallMessage_Busy>>(
                    "busy",
                    CallMessage::get_busy_for_reflect,
                    CallMessage::mut_busy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CallMessage>(
                    "CallMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CallMessage {
    fn clear(&mut self) {
        self.clear_offer();
        self.clear_answer();
        self.clear_iceUpdate();
        self.clear_hangup();
        self.clear_busy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallMessage_Offer {
    // message fields
    id: ::std::option::Option<u64>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CallMessage_Offer {}

impl CallMessage_Offer {
    pub fn new() -> CallMessage_Offer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CallMessage_Offer {
        static mut instance: ::protobuf::lazy::Lazy<CallMessage_Offer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CallMessage_Offer,
        };
        unsafe {
            instance.get(CallMessage_Offer::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }
}

impl ::protobuf::Message for CallMessage_Offer {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
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
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.description.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CallMessage_Offer {
    fn new() -> CallMessage_Offer {
        CallMessage_Offer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CallMessage_Offer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CallMessage_Offer::get_id_for_reflect,
                    CallMessage_Offer::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CallMessage_Offer::get_description_for_reflect,
                    CallMessage_Offer::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CallMessage_Offer>(
                    "CallMessage_Offer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CallMessage_Offer {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallMessage_Offer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallMessage_Offer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallMessage_Answer {
    // message fields
    id: ::std::option::Option<u64>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CallMessage_Answer {}

impl CallMessage_Answer {
    pub fn new() -> CallMessage_Answer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CallMessage_Answer {
        static mut instance: ::protobuf::lazy::Lazy<CallMessage_Answer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CallMessage_Answer,
        };
        unsafe {
            instance.get(CallMessage_Answer::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }
}

impl ::protobuf::Message for CallMessage_Answer {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
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
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.description.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CallMessage_Answer {
    fn new() -> CallMessage_Answer {
        CallMessage_Answer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CallMessage_Answer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CallMessage_Answer::get_id_for_reflect,
                    CallMessage_Answer::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CallMessage_Answer::get_description_for_reflect,
                    CallMessage_Answer::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CallMessage_Answer>(
                    "CallMessage_Answer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CallMessage_Answer {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallMessage_Answer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallMessage_Answer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallMessage_IceUpdate {
    // message fields
    id: ::std::option::Option<u64>,
    sdpMid: ::protobuf::SingularField<::std::string::String>,
    sdpMLineIndex: ::std::option::Option<u32>,
    sdp: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CallMessage_IceUpdate {}

impl CallMessage_IceUpdate {
    pub fn new() -> CallMessage_IceUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CallMessage_IceUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CallMessage_IceUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CallMessage_IceUpdate,
        };
        unsafe {
            instance.get(CallMessage_IceUpdate::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string sdpMid = 2;

    pub fn clear_sdpMid(&mut self) {
        self.sdpMid.clear();
    }

    pub fn has_sdpMid(&self) -> bool {
        self.sdpMid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdpMid(&mut self, v: ::std::string::String) {
        self.sdpMid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sdpMid(&mut self) -> &mut ::std::string::String {
        if self.sdpMid.is_none() {
            self.sdpMid.set_default();
        };
        self.sdpMid.as_mut().unwrap()
    }

    // Take field
    pub fn take_sdpMid(&mut self) -> ::std::string::String {
        self.sdpMid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sdpMid(&self) -> &str {
        match self.sdpMid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sdpMid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sdpMid
    }

    fn mut_sdpMid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sdpMid
    }

    // optional uint32 sdpMLineIndex = 3;

    pub fn clear_sdpMLineIndex(&mut self) {
        self.sdpMLineIndex = ::std::option::Option::None;
    }

    pub fn has_sdpMLineIndex(&self) -> bool {
        self.sdpMLineIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdpMLineIndex(&mut self, v: u32) {
        self.sdpMLineIndex = ::std::option::Option::Some(v);
    }

    pub fn get_sdpMLineIndex(&self) -> u32 {
        self.sdpMLineIndex.unwrap_or(0)
    }

    fn get_sdpMLineIndex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sdpMLineIndex
    }

    fn mut_sdpMLineIndex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sdpMLineIndex
    }

    // optional string sdp = 4;

    pub fn clear_sdp(&mut self) {
        self.sdp.clear();
    }

    pub fn has_sdp(&self) -> bool {
        self.sdp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdp(&mut self, v: ::std::string::String) {
        self.sdp = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sdp(&mut self) -> &mut ::std::string::String {
        if self.sdp.is_none() {
            self.sdp.set_default();
        };
        self.sdp.as_mut().unwrap()
    }

    // Take field
    pub fn take_sdp(&mut self) -> ::std::string::String {
        self.sdp.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sdp(&self) -> &str {
        match self.sdp.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sdp_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sdp
    }

    fn mut_sdp_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sdp
    }
}

impl ::protobuf::Message for CallMessage_IceUpdate {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sdpMid)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sdpMLineIndex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sdp)?;
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
        if let Some(v) = self.sdpMid.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.sdpMLineIndex {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sdp.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.sdpMid.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.sdpMLineIndex {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.sdp.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for CallMessage_IceUpdate {
    fn new() -> CallMessage_IceUpdate {
        CallMessage_IceUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CallMessage_IceUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CallMessage_IceUpdate::get_id_for_reflect,
                    CallMessage_IceUpdate::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sdpMid",
                    CallMessage_IceUpdate::get_sdpMid_for_reflect,
                    CallMessage_IceUpdate::mut_sdpMid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sdpMLineIndex",
                    CallMessage_IceUpdate::get_sdpMLineIndex_for_reflect,
                    CallMessage_IceUpdate::mut_sdpMLineIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sdp",
                    CallMessage_IceUpdate::get_sdp_for_reflect,
                    CallMessage_IceUpdate::mut_sdp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CallMessage_IceUpdate>(
                    "CallMessage_IceUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CallMessage_IceUpdate {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_sdpMid();
        self.clear_sdpMLineIndex();
        self.clear_sdp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallMessage_IceUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallMessage_IceUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallMessage_Busy {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CallMessage_Busy {}

impl CallMessage_Busy {
    pub fn new() -> CallMessage_Busy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CallMessage_Busy {
        static mut instance: ::protobuf::lazy::Lazy<CallMessage_Busy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CallMessage_Busy,
        };
        unsafe {
            instance.get(CallMessage_Busy::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }
}

impl ::protobuf::Message for CallMessage_Busy {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for CallMessage_Busy {
    fn new() -> CallMessage_Busy {
        CallMessage_Busy::new()
    }

    fn descriptor_static(_: ::std::option::Option<CallMessage_Busy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CallMessage_Busy::get_id_for_reflect,
                    CallMessage_Busy::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CallMessage_Busy>(
                    "CallMessage_Busy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CallMessage_Busy {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallMessage_Busy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallMessage_Busy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallMessage_Hangup {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CallMessage_Hangup {}

impl CallMessage_Hangup {
    pub fn new() -> CallMessage_Hangup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CallMessage_Hangup {
        static mut instance: ::protobuf::lazy::Lazy<CallMessage_Hangup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CallMessage_Hangup,
        };
        unsafe {
            instance.get(CallMessage_Hangup::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }
}

impl ::protobuf::Message for CallMessage_Hangup {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for CallMessage_Hangup {
    fn new() -> CallMessage_Hangup {
        CallMessage_Hangup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CallMessage_Hangup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CallMessage_Hangup::get_id_for_reflect,
                    CallMessage_Hangup::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CallMessage_Hangup>(
                    "CallMessage_Hangup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CallMessage_Hangup {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallMessage_Hangup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallMessage_Hangup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataMessage {
    // message fields
    body: ::protobuf::SingularField<::std::string::String>,
    attachments: ::protobuf::RepeatedField<AttachmentPointer>,
    group: ::protobuf::SingularPtrField<GroupContext>,
    flags: ::std::option::Option<u32>,
    expireTimer: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataMessage {}

impl DataMessage {
    pub fn new() -> DataMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataMessage {
        static mut instance: ::protobuf::lazy::Lazy<DataMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataMessage,
        };
        unsafe {
            instance.get(DataMessage::new)
        }
    }

    // optional string body = 1;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::string::String) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::string::String {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::string::String {
        self.body.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_body(&self) -> &str {
        match self.body.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.body
    }

    // repeated .signalservice.AttachmentPointer attachments = 2;

    pub fn clear_attachments(&mut self) {
        self.attachments.clear();
    }

    // Param is passed by value, moved
    pub fn set_attachments(&mut self, v: ::protobuf::RepeatedField<AttachmentPointer>) {
        self.attachments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attachments(&mut self) -> &mut ::protobuf::RepeatedField<AttachmentPointer> {
        &mut self.attachments
    }

    // Take field
    pub fn take_attachments(&mut self) -> ::protobuf::RepeatedField<AttachmentPointer> {
        ::std::mem::replace(&mut self.attachments, ::protobuf::RepeatedField::new())
    }

    pub fn get_attachments(&self) -> &[AttachmentPointer] {
        &self.attachments
    }

    fn get_attachments_for_reflect(&self) -> &::protobuf::RepeatedField<AttachmentPointer> {
        &self.attachments
    }

    fn mut_attachments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AttachmentPointer> {
        &mut self.attachments
    }

    // optional .signalservice.GroupContext group = 3;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: GroupContext) {
        self.group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut GroupContext {
        if self.group.is_none() {
            self.group.set_default();
        };
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> GroupContext {
        self.group.take().unwrap_or_else(|| GroupContext::new())
    }

    pub fn get_group(&self) -> &GroupContext {
        self.group.as_ref().unwrap_or_else(|| GroupContext::default_instance())
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularPtrField<GroupContext> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GroupContext> {
        &mut self.group
    }

    // optional uint32 flags = 4;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional uint32 expireTimer = 5;

    pub fn clear_expireTimer(&mut self) {
        self.expireTimer = ::std::option::Option::None;
    }

    pub fn has_expireTimer(&self) -> bool {
        self.expireTimer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expireTimer(&mut self, v: u32) {
        self.expireTimer = ::std::option::Option::Some(v);
    }

    pub fn get_expireTimer(&self) -> u32 {
        self.expireTimer.unwrap_or(0)
    }

    fn get_expireTimer_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.expireTimer
    }

    fn mut_expireTimer_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.expireTimer
    }
}

impl ::protobuf::Message for DataMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.body)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attachments)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.group)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.expireTimer = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.attachments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.group.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.expireTimer {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.body.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.attachments {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.group.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.expireTimer {
            os.write_uint32(5, v)?;
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

impl ::protobuf::MessageStatic for DataMessage {
    fn new() -> DataMessage {
        DataMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "body",
                    DataMessage::get_body_for_reflect,
                    DataMessage::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AttachmentPointer>>(
                    "attachments",
                    DataMessage::get_attachments_for_reflect,
                    DataMessage::mut_attachments_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GroupContext>>(
                    "group",
                    DataMessage::get_group_for_reflect,
                    DataMessage::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    DataMessage::get_flags_for_reflect,
                    DataMessage::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "expireTimer",
                    DataMessage::get_expireTimer_for_reflect,
                    DataMessage::mut_expireTimer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataMessage>(
                    "DataMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataMessage {
    fn clear(&mut self) {
        self.clear_body();
        self.clear_attachments();
        self.clear_group();
        self.clear_flags();
        self.clear_expireTimer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataMessage_Flags {
    END_SESSION = 1,
    EXPIRATION_TIMER_UPDATE = 2,
}

impl ::protobuf::ProtobufEnum for DataMessage_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataMessage_Flags> {
        match value {
            1 => ::std::option::Option::Some(DataMessage_Flags::END_SESSION),
            2 => ::std::option::Option::Some(DataMessage_Flags::EXPIRATION_TIMER_UPDATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataMessage_Flags] = &[
            DataMessage_Flags::END_SESSION,
            DataMessage_Flags::EXPIRATION_TIMER_UPDATE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DataMessage_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataMessage_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataMessage_Flags {
}

impl ::protobuf::reflect::ProtobufValue for DataMessage_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage {
    // message fields
    sent: ::protobuf::SingularPtrField<SyncMessage_Sent>,
    contacts: ::protobuf::SingularPtrField<SyncMessage_Contacts>,
    groups: ::protobuf::SingularPtrField<SyncMessage_Groups>,
    request: ::protobuf::SingularPtrField<SyncMessage_Request>,
    read: ::protobuf::RepeatedField<SyncMessage_Read>,
    blocked: ::protobuf::SingularPtrField<SyncMessage_Blocked>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage {}

impl SyncMessage {
    pub fn new() -> SyncMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage,
        };
        unsafe {
            instance.get(SyncMessage::new)
        }
    }

    // optional .signalservice.SyncMessage.Sent sent = 1;

    pub fn clear_sent(&mut self) {
        self.sent.clear();
    }

    pub fn has_sent(&self) -> bool {
        self.sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sent(&mut self, v: SyncMessage_Sent) {
        self.sent = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sent(&mut self) -> &mut SyncMessage_Sent {
        if self.sent.is_none() {
            self.sent.set_default();
        };
        self.sent.as_mut().unwrap()
    }

    // Take field
    pub fn take_sent(&mut self) -> SyncMessage_Sent {
        self.sent.take().unwrap_or_else(|| SyncMessage_Sent::new())
    }

    pub fn get_sent(&self) -> &SyncMessage_Sent {
        self.sent.as_ref().unwrap_or_else(|| SyncMessage_Sent::default_instance())
    }

    fn get_sent_for_reflect(&self) -> &::protobuf::SingularPtrField<SyncMessage_Sent> {
        &self.sent
    }

    fn mut_sent_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SyncMessage_Sent> {
        &mut self.sent
    }

    // optional .signalservice.SyncMessage.Contacts contacts = 2;

    pub fn clear_contacts(&mut self) {
        self.contacts.clear();
    }

    pub fn has_contacts(&self) -> bool {
        self.contacts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contacts(&mut self, v: SyncMessage_Contacts) {
        self.contacts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contacts(&mut self) -> &mut SyncMessage_Contacts {
        if self.contacts.is_none() {
            self.contacts.set_default();
        };
        self.contacts.as_mut().unwrap()
    }

    // Take field
    pub fn take_contacts(&mut self) -> SyncMessage_Contacts {
        self.contacts.take().unwrap_or_else(|| SyncMessage_Contacts::new())
    }

    pub fn get_contacts(&self) -> &SyncMessage_Contacts {
        self.contacts.as_ref().unwrap_or_else(|| SyncMessage_Contacts::default_instance())
    }

    fn get_contacts_for_reflect(&self) -> &::protobuf::SingularPtrField<SyncMessage_Contacts> {
        &self.contacts
    }

    fn mut_contacts_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SyncMessage_Contacts> {
        &mut self.contacts
    }

    // optional .signalservice.SyncMessage.Groups groups = 3;

    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }

    pub fn has_groups(&self) -> bool {
        self.groups.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groups(&mut self, v: SyncMessage_Groups) {
        self.groups = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groups(&mut self) -> &mut SyncMessage_Groups {
        if self.groups.is_none() {
            self.groups.set_default();
        };
        self.groups.as_mut().unwrap()
    }

    // Take field
    pub fn take_groups(&mut self) -> SyncMessage_Groups {
        self.groups.take().unwrap_or_else(|| SyncMessage_Groups::new())
    }

    pub fn get_groups(&self) -> &SyncMessage_Groups {
        self.groups.as_ref().unwrap_or_else(|| SyncMessage_Groups::default_instance())
    }

    fn get_groups_for_reflect(&self) -> &::protobuf::SingularPtrField<SyncMessage_Groups> {
        &self.groups
    }

    fn mut_groups_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SyncMessage_Groups> {
        &mut self.groups
    }

    // optional .signalservice.SyncMessage.Request request = 4;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: SyncMessage_Request) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut SyncMessage_Request {
        if self.request.is_none() {
            self.request.set_default();
        };
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> SyncMessage_Request {
        self.request.take().unwrap_or_else(|| SyncMessage_Request::new())
    }

    pub fn get_request(&self) -> &SyncMessage_Request {
        self.request.as_ref().unwrap_or_else(|| SyncMessage_Request::default_instance())
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularPtrField<SyncMessage_Request> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SyncMessage_Request> {
        &mut self.request
    }

    // repeated .signalservice.SyncMessage.Read read = 5;

    pub fn clear_read(&mut self) {
        self.read.clear();
    }

    // Param is passed by value, moved
    pub fn set_read(&mut self, v: ::protobuf::RepeatedField<SyncMessage_Read>) {
        self.read = v;
    }

    // Mutable pointer to the field.
    pub fn mut_read(&mut self) -> &mut ::protobuf::RepeatedField<SyncMessage_Read> {
        &mut self.read
    }

    // Take field
    pub fn take_read(&mut self) -> ::protobuf::RepeatedField<SyncMessage_Read> {
        ::std::mem::replace(&mut self.read, ::protobuf::RepeatedField::new())
    }

    pub fn get_read(&self) -> &[SyncMessage_Read] {
        &self.read
    }

    fn get_read_for_reflect(&self) -> &::protobuf::RepeatedField<SyncMessage_Read> {
        &self.read
    }

    fn mut_read_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SyncMessage_Read> {
        &mut self.read
    }

    // optional .signalservice.SyncMessage.Blocked blocked = 6;

    pub fn clear_blocked(&mut self) {
        self.blocked.clear();
    }

    pub fn has_blocked(&self) -> bool {
        self.blocked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blocked(&mut self, v: SyncMessage_Blocked) {
        self.blocked = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blocked(&mut self) -> &mut SyncMessage_Blocked {
        if self.blocked.is_none() {
            self.blocked.set_default();
        };
        self.blocked.as_mut().unwrap()
    }

    // Take field
    pub fn take_blocked(&mut self) -> SyncMessage_Blocked {
        self.blocked.take().unwrap_or_else(|| SyncMessage_Blocked::new())
    }

    pub fn get_blocked(&self) -> &SyncMessage_Blocked {
        self.blocked.as_ref().unwrap_or_else(|| SyncMessage_Blocked::default_instance())
    }

    fn get_blocked_for_reflect(&self) -> &::protobuf::SingularPtrField<SyncMessage_Blocked> {
        &self.blocked
    }

    fn mut_blocked_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SyncMessage_Blocked> {
        &mut self.blocked
    }
}

impl ::protobuf::Message for SyncMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sent)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contacts)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.groups)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.read)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blocked)?;
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
        if let Some(v) = self.sent.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.contacts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.groups.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.read {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.blocked.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sent.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.contacts.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.groups.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.request.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.read {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.blocked.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SyncMessage {
    fn new() -> SyncMessage {
        SyncMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage_Sent>>(
                    "sent",
                    SyncMessage::get_sent_for_reflect,
                    SyncMessage::mut_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage_Contacts>>(
                    "contacts",
                    SyncMessage::get_contacts_for_reflect,
                    SyncMessage::mut_contacts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage_Groups>>(
                    "groups",
                    SyncMessage::get_groups_for_reflect,
                    SyncMessage::mut_groups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage_Request>>(
                    "request",
                    SyncMessage::get_request_for_reflect,
                    SyncMessage::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage_Read>>(
                    "read",
                    SyncMessage::get_read_for_reflect,
                    SyncMessage::mut_read_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SyncMessage_Blocked>>(
                    "blocked",
                    SyncMessage::get_blocked_for_reflect,
                    SyncMessage::mut_blocked_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage>(
                    "SyncMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage {
    fn clear(&mut self) {
        self.clear_sent();
        self.clear_contacts();
        self.clear_groups();
        self.clear_request();
        self.clear_read();
        self.clear_blocked();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage_Sent {
    // message fields
    destination: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<u64>,
    message: ::protobuf::SingularPtrField<DataMessage>,
    expirationStartTimestamp: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage_Sent {}

impl SyncMessage_Sent {
    pub fn new() -> SyncMessage_Sent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage_Sent {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage_Sent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage_Sent,
        };
        unsafe {
            instance.get(SyncMessage_Sent::new)
        }
    }

    // optional string destination = 1;

    pub fn clear_destination(&mut self) {
        self.destination.clear();
    }

    pub fn has_destination(&self) -> bool {
        self.destination.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destination(&mut self, v: ::std::string::String) {
        self.destination = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination(&mut self) -> &mut ::std::string::String {
        if self.destination.is_none() {
            self.destination.set_default();
        };
        self.destination.as_mut().unwrap()
    }

    // Take field
    pub fn take_destination(&mut self) -> ::std::string::String {
        self.destination.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_destination(&self) -> &str {
        match self.destination.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_destination_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.destination
    }

    fn mut_destination_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.destination
    }

    // optional uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }

    // optional .signalservice.DataMessage message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: DataMessage) {
        self.message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut DataMessage {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> DataMessage {
        self.message.take().unwrap_or_else(|| DataMessage::new())
    }

    pub fn get_message(&self) -> &DataMessage {
        self.message.as_ref().unwrap_or_else(|| DataMessage::default_instance())
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularPtrField<DataMessage> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataMessage> {
        &mut self.message
    }

    // optional uint64 expirationStartTimestamp = 4;

    pub fn clear_expirationStartTimestamp(&mut self) {
        self.expirationStartTimestamp = ::std::option::Option::None;
    }

    pub fn has_expirationStartTimestamp(&self) -> bool {
        self.expirationStartTimestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expirationStartTimestamp(&mut self, v: u64) {
        self.expirationStartTimestamp = ::std::option::Option::Some(v);
    }

    pub fn get_expirationStartTimestamp(&self) -> u64 {
        self.expirationStartTimestamp.unwrap_or(0)
    }

    fn get_expirationStartTimestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.expirationStartTimestamp
    }

    fn mut_expirationStartTimestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.expirationStartTimestamp
    }
}

impl ::protobuf::Message for SyncMessage_Sent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.destination)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.expirationStartTimestamp = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.destination.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.expirationStartTimestamp {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.destination.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.message.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.expirationStartTimestamp {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for SyncMessage_Sent {
    fn new() -> SyncMessage_Sent {
        SyncMessage_Sent::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage_Sent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "destination",
                    SyncMessage_Sent::get_destination_for_reflect,
                    SyncMessage_Sent::mut_destination_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    SyncMessage_Sent::get_timestamp_for_reflect,
                    SyncMessage_Sent::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataMessage>>(
                    "message",
                    SyncMessage_Sent::get_message_for_reflect,
                    SyncMessage_Sent::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expirationStartTimestamp",
                    SyncMessage_Sent::get_expirationStartTimestamp_for_reflect,
                    SyncMessage_Sent::mut_expirationStartTimestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage_Sent>(
                    "SyncMessage_Sent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage_Sent {
    fn clear(&mut self) {
        self.clear_destination();
        self.clear_timestamp();
        self.clear_message();
        self.clear_expirationStartTimestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage_Sent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Sent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage_Contacts {
    // message fields
    blob: ::protobuf::SingularPtrField<AttachmentPointer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage_Contacts {}

impl SyncMessage_Contacts {
    pub fn new() -> SyncMessage_Contacts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage_Contacts {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage_Contacts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage_Contacts,
        };
        unsafe {
            instance.get(SyncMessage_Contacts::new)
        }
    }

    // optional .signalservice.AttachmentPointer blob = 1;

    pub fn clear_blob(&mut self) {
        self.blob.clear();
    }

    pub fn has_blob(&self) -> bool {
        self.blob.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blob(&mut self, v: AttachmentPointer) {
        self.blob = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob(&mut self) -> &mut AttachmentPointer {
        if self.blob.is_none() {
            self.blob.set_default();
        };
        self.blob.as_mut().unwrap()
    }

    // Take field
    pub fn take_blob(&mut self) -> AttachmentPointer {
        self.blob.take().unwrap_or_else(|| AttachmentPointer::new())
    }

    pub fn get_blob(&self) -> &AttachmentPointer {
        self.blob.as_ref().unwrap_or_else(|| AttachmentPointer::default_instance())
    }

    fn get_blob_for_reflect(&self) -> &::protobuf::SingularPtrField<AttachmentPointer> {
        &self.blob
    }

    fn mut_blob_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AttachmentPointer> {
        &mut self.blob
    }
}

impl ::protobuf::Message for SyncMessage_Contacts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blob)?;
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
        if let Some(v) = self.blob.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blob.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SyncMessage_Contacts {
    fn new() -> SyncMessage_Contacts {
        SyncMessage_Contacts::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage_Contacts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AttachmentPointer>>(
                    "blob",
                    SyncMessage_Contacts::get_blob_for_reflect,
                    SyncMessage_Contacts::mut_blob_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage_Contacts>(
                    "SyncMessage_Contacts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage_Contacts {
    fn clear(&mut self) {
        self.clear_blob();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage_Contacts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Contacts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage_Groups {
    // message fields
    blob: ::protobuf::SingularPtrField<AttachmentPointer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage_Groups {}

impl SyncMessage_Groups {
    pub fn new() -> SyncMessage_Groups {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage_Groups {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage_Groups> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage_Groups,
        };
        unsafe {
            instance.get(SyncMessage_Groups::new)
        }
    }

    // optional .signalservice.AttachmentPointer blob = 1;

    pub fn clear_blob(&mut self) {
        self.blob.clear();
    }

    pub fn has_blob(&self) -> bool {
        self.blob.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blob(&mut self, v: AttachmentPointer) {
        self.blob = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob(&mut self) -> &mut AttachmentPointer {
        if self.blob.is_none() {
            self.blob.set_default();
        };
        self.blob.as_mut().unwrap()
    }

    // Take field
    pub fn take_blob(&mut self) -> AttachmentPointer {
        self.blob.take().unwrap_or_else(|| AttachmentPointer::new())
    }

    pub fn get_blob(&self) -> &AttachmentPointer {
        self.blob.as_ref().unwrap_or_else(|| AttachmentPointer::default_instance())
    }

    fn get_blob_for_reflect(&self) -> &::protobuf::SingularPtrField<AttachmentPointer> {
        &self.blob
    }

    fn mut_blob_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AttachmentPointer> {
        &mut self.blob
    }
}

impl ::protobuf::Message for SyncMessage_Groups {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blob)?;
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
        if let Some(v) = self.blob.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blob.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SyncMessage_Groups {
    fn new() -> SyncMessage_Groups {
        SyncMessage_Groups::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage_Groups>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AttachmentPointer>>(
                    "blob",
                    SyncMessage_Groups::get_blob_for_reflect,
                    SyncMessage_Groups::mut_blob_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage_Groups>(
                    "SyncMessage_Groups",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage_Groups {
    fn clear(&mut self) {
        self.clear_blob();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage_Groups {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Groups {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage_Blocked {
    // message fields
    numbers: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage_Blocked {}

impl SyncMessage_Blocked {
    pub fn new() -> SyncMessage_Blocked {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage_Blocked {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage_Blocked> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage_Blocked,
        };
        unsafe {
            instance.get(SyncMessage_Blocked::new)
        }
    }

    // repeated string numbers = 1;

    pub fn clear_numbers(&mut self) {
        self.numbers.clear();
    }

    // Param is passed by value, moved
    pub fn set_numbers(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.numbers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_numbers(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.numbers
    }

    // Take field
    pub fn take_numbers(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.numbers, ::protobuf::RepeatedField::new())
    }

    pub fn get_numbers(&self) -> &[::std::string::String] {
        &self.numbers
    }

    fn get_numbers_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.numbers
    }

    fn mut_numbers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.numbers
    }
}

impl ::protobuf::Message for SyncMessage_Blocked {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.numbers)?;
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
        for value in &self.numbers {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.numbers {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for SyncMessage_Blocked {
    fn new() -> SyncMessage_Blocked {
        SyncMessage_Blocked::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage_Blocked>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "numbers",
                    SyncMessage_Blocked::get_numbers_for_reflect,
                    SyncMessage_Blocked::mut_numbers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage_Blocked>(
                    "SyncMessage_Blocked",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage_Blocked {
    fn clear(&mut self) {
        self.clear_numbers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage_Blocked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Blocked {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage_Request {
    // message fields
    field_type: ::std::option::Option<SyncMessage_Request_Type>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage_Request {}

impl SyncMessage_Request {
    pub fn new() -> SyncMessage_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage_Request {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage_Request,
        };
        unsafe {
            instance.get(SyncMessage_Request::new)
        }
    }

    // optional .signalservice.SyncMessage.Request.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: SyncMessage_Request_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> SyncMessage_Request_Type {
        self.field_type.unwrap_or(SyncMessage_Request_Type::UNKNOWN)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<SyncMessage_Request_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<SyncMessage_Request_Type> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for SyncMessage_Request {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for SyncMessage_Request {
    fn new() -> SyncMessage_Request {
        SyncMessage_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SyncMessage_Request_Type>>(
                    "type",
                    SyncMessage_Request::get_field_type_for_reflect,
                    SyncMessage_Request::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage_Request>(
                    "SyncMessage_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage_Request {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SyncMessage_Request_Type {
    UNKNOWN = 0,
    CONTACTS = 1,
    GROUPS = 2,
    BLOCKED = 3,
}

impl ::protobuf::ProtobufEnum for SyncMessage_Request_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SyncMessage_Request_Type> {
        match value {
            0 => ::std::option::Option::Some(SyncMessage_Request_Type::UNKNOWN),
            1 => ::std::option::Option::Some(SyncMessage_Request_Type::CONTACTS),
            2 => ::std::option::Option::Some(SyncMessage_Request_Type::GROUPS),
            3 => ::std::option::Option::Some(SyncMessage_Request_Type::BLOCKED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SyncMessage_Request_Type] = &[
            SyncMessage_Request_Type::UNKNOWN,
            SyncMessage_Request_Type::CONTACTS,
            SyncMessage_Request_Type::GROUPS,
            SyncMessage_Request_Type::BLOCKED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SyncMessage_Request_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SyncMessage_Request_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SyncMessage_Request_Type {
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Request_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncMessage_Read {
    // message fields
    sender: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncMessage_Read {}

impl SyncMessage_Read {
    pub fn new() -> SyncMessage_Read {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncMessage_Read {
        static mut instance: ::protobuf::lazy::Lazy<SyncMessage_Read> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncMessage_Read,
        };
        unsafe {
            instance.get(SyncMessage_Read::new)
        }
    }

    // optional string sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender(&mut self) -> &mut ::std::string::String {
        if self.sender.is_none() {
            self.sender.set_default();
        };
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        self.sender.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender(&self) -> &str {
        match self.sender.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sender_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sender
    }

    fn mut_sender_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sender
    }

    // optional uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for SyncMessage_Read {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sender)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sender.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sender.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for SyncMessage_Read {
    fn new() -> SyncMessage_Read {
        SyncMessage_Read::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncMessage_Read>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sender",
                    SyncMessage_Read::get_sender_for_reflect,
                    SyncMessage_Read::mut_sender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    SyncMessage_Read::get_timestamp_for_reflect,
                    SyncMessage_Read::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncMessage_Read>(
                    "SyncMessage_Read",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncMessage_Read {
    fn clear(&mut self) {
        self.clear_sender();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncMessage_Read {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncMessage_Read {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AttachmentPointer {
    // message fields
    id: ::std::option::Option<u64>,
    contentType: ::protobuf::SingularField<::std::string::String>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    size: ::std::option::Option<u32>,
    thumbnail: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    digest: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AttachmentPointer {}

impl AttachmentPointer {
    pub fn new() -> AttachmentPointer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttachmentPointer {
        static mut instance: ::protobuf::lazy::Lazy<AttachmentPointer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttachmentPointer,
        };
        unsafe {
            instance.get(AttachmentPointer::new)
        }
    }

    // optional fixed64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string contentType = 2;

    pub fn clear_contentType(&mut self) {
        self.contentType.clear();
    }

    pub fn has_contentType(&self) -> bool {
        self.contentType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentType(&mut self, v: ::std::string::String) {
        self.contentType = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contentType(&mut self) -> &mut ::std::string::String {
        if self.contentType.is_none() {
            self.contentType.set_default();
        };
        self.contentType.as_mut().unwrap()
    }

    // Take field
    pub fn take_contentType(&mut self) -> ::std::string::String {
        self.contentType.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_contentType(&self) -> &str {
        match self.contentType.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_contentType_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.contentType
    }

    fn mut_contentType_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.contentType
    }

    // optional bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint32 size = 4;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u32 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.size
    }

    // optional bytes thumbnail = 5;

    pub fn clear_thumbnail(&mut self) {
        self.thumbnail.clear();
    }

    pub fn has_thumbnail(&self) -> bool {
        self.thumbnail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thumbnail(&mut self, v: ::std::vec::Vec<u8>) {
        self.thumbnail = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_thumbnail(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.thumbnail.is_none() {
            self.thumbnail.set_default();
        };
        self.thumbnail.as_mut().unwrap()
    }

    // Take field
    pub fn take_thumbnail(&mut self) -> ::std::vec::Vec<u8> {
        self.thumbnail.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_thumbnail(&self) -> &[u8] {
        match self.thumbnail.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_thumbnail_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.thumbnail
    }

    fn mut_thumbnail_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.thumbnail
    }

    // optional bytes digest = 6;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    pub fn has_digest(&self) -> bool {
        self.digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: ::std::vec::Vec<u8>) {
        self.digest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_digest(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.digest.is_none() {
            self.digest.set_default();
        };
        self.digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_digest(&mut self) -> ::std::vec::Vec<u8> {
        self.digest.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_digest(&self) -> &[u8] {
        match self.digest.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_digest_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.digest
    }

    fn mut_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.digest
    }
}

impl ::protobuf::Message for AttachmentPointer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.contentType)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.thumbnail)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.digest)?;
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
            my_size += 9;
        };
        if let Some(v) = self.contentType.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.thumbnail.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.digest.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.contentType.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.size {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.thumbnail.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.digest.as_ref() {
            os.write_bytes(6, &v)?;
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

impl ::protobuf::MessageStatic for AttachmentPointer {
    fn new() -> AttachmentPointer {
        AttachmentPointer::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttachmentPointer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "id",
                    AttachmentPointer::get_id_for_reflect,
                    AttachmentPointer::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "contentType",
                    AttachmentPointer::get_contentType_for_reflect,
                    AttachmentPointer::mut_contentType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    AttachmentPointer::get_key_for_reflect,
                    AttachmentPointer::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "size",
                    AttachmentPointer::get_size_for_reflect,
                    AttachmentPointer::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "thumbnail",
                    AttachmentPointer::get_thumbnail_for_reflect,
                    AttachmentPointer::mut_thumbnail_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "digest",
                    AttachmentPointer::get_digest_for_reflect,
                    AttachmentPointer::mut_digest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttachmentPointer>(
                    "AttachmentPointer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttachmentPointer {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_contentType();
        self.clear_key();
        self.clear_size();
        self.clear_thumbnail();
        self.clear_digest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AttachmentPointer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AttachmentPointer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupContext {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::std::option::Option<GroupContext_Type>,
    name: ::protobuf::SingularField<::std::string::String>,
    members: ::protobuf::RepeatedField<::std::string::String>,
    avatar: ::protobuf::SingularPtrField<AttachmentPointer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GroupContext {}

impl GroupContext {
    pub fn new() -> GroupContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GroupContext {
        static mut instance: ::protobuf::lazy::Lazy<GroupContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupContext,
        };
        unsafe {
            instance.get(GroupContext::new)
        }
    }

    // optional bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.id
    }

    // optional .signalservice.GroupContext.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: GroupContext_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> GroupContext_Type {
        self.field_type.unwrap_or(GroupContext_Type::UNKNOWN)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<GroupContext_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<GroupContext_Type> {
        &mut self.field_type
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated string members = 4;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[::std::string::String] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // optional .signalservice.AttachmentPointer avatar = 5;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: AttachmentPointer) {
        self.avatar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut AttachmentPointer {
        if self.avatar.is_none() {
            self.avatar.set_default();
        };
        self.avatar.as_mut().unwrap()
    }

    // Take field
    pub fn take_avatar(&mut self) -> AttachmentPointer {
        self.avatar.take().unwrap_or_else(|| AttachmentPointer::new())
    }

    pub fn get_avatar(&self) -> &AttachmentPointer {
        self.avatar.as_ref().unwrap_or_else(|| AttachmentPointer::default_instance())
    }

    fn get_avatar_for_reflect(&self) -> &::protobuf::SingularPtrField<AttachmentPointer> {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AttachmentPointer> {
        &mut self.avatar
    }
}

impl ::protobuf::Message for GroupContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.members)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.avatar)?;
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
        if let Some(v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        for value in &self.members {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(v) = self.avatar.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        for v in &self.members {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.avatar.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GroupContext {
    fn new() -> GroupContext {
        GroupContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<GroupContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    GroupContext::get_id_for_reflect,
                    GroupContext::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GroupContext_Type>>(
                    "type",
                    GroupContext::get_field_type_for_reflect,
                    GroupContext::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GroupContext::get_name_for_reflect,
                    GroupContext::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "members",
                    GroupContext::get_members_for_reflect,
                    GroupContext::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AttachmentPointer>>(
                    "avatar",
                    GroupContext::get_avatar_for_reflect,
                    GroupContext::mut_avatar_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GroupContext>(
                    "GroupContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GroupContext {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_field_type();
        self.clear_name();
        self.clear_members();
        self.clear_avatar();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GroupContext_Type {
    UNKNOWN = 0,
    UPDATE = 1,
    DELIVER = 2,
    QUIT = 3,
    REQUEST_INFO = 4,
}

impl ::protobuf::ProtobufEnum for GroupContext_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GroupContext_Type> {
        match value {
            0 => ::std::option::Option::Some(GroupContext_Type::UNKNOWN),
            1 => ::std::option::Option::Some(GroupContext_Type::UPDATE),
            2 => ::std::option::Option::Some(GroupContext_Type::DELIVER),
            3 => ::std::option::Option::Some(GroupContext_Type::QUIT),
            4 => ::std::option::Option::Some(GroupContext_Type::REQUEST_INFO),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GroupContext_Type] = &[
            GroupContext_Type::UNKNOWN,
            GroupContext_Type::UPDATE,
            GroupContext_Type::DELIVER,
            GroupContext_Type::QUIT,
            GroupContext_Type::REQUEST_INFO,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<GroupContext_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GroupContext_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GroupContext_Type {
}

impl ::protobuf::reflect::ProtobufValue for GroupContext_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContactDetails {
    // message fields
    number: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    avatar: ::protobuf::SingularPtrField<ContactDetails_Avatar>,
    color: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContactDetails {}

impl ContactDetails {
    pub fn new() -> ContactDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContactDetails {
        static mut instance: ::protobuf::lazy::Lazy<ContactDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContactDetails,
        };
        unsafe {
            instance.get(ContactDetails::new)
        }
    }

    // optional string number = 1;

    pub fn clear_number(&mut self) {
        self.number.clear();
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: ::std::string::String) {
        self.number = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number(&mut self) -> &mut ::std::string::String {
        if self.number.is_none() {
            self.number.set_default();
        };
        self.number.as_mut().unwrap()
    }

    // Take field
    pub fn take_number(&mut self) -> ::std::string::String {
        self.number.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_number(&self) -> &str {
        match self.number.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_number_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.number
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .signalservice.ContactDetails.Avatar avatar = 3;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: ContactDetails_Avatar) {
        self.avatar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut ContactDetails_Avatar {
        if self.avatar.is_none() {
            self.avatar.set_default();
        };
        self.avatar.as_mut().unwrap()
    }

    // Take field
    pub fn take_avatar(&mut self) -> ContactDetails_Avatar {
        self.avatar.take().unwrap_or_else(|| ContactDetails_Avatar::new())
    }

    pub fn get_avatar(&self) -> &ContactDetails_Avatar {
        self.avatar.as_ref().unwrap_or_else(|| ContactDetails_Avatar::default_instance())
    }

    fn get_avatar_for_reflect(&self) -> &::protobuf::SingularPtrField<ContactDetails_Avatar> {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ContactDetails_Avatar> {
        &mut self.avatar
    }

    // optional string color = 4;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: ::std::string::String) {
        self.color = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut ::std::string::String {
        if self.color.is_none() {
            self.color.set_default();
        };
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> ::std::string::String {
        self.color.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_color(&self) -> &str {
        match self.color.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_color_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.color
    }
}

impl ::protobuf::Message for ContactDetails {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.number)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.avatar)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.color)?;
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
        if let Some(v) = self.number.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.avatar.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.color.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.number.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.avatar.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.color.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for ContactDetails {
    fn new() -> ContactDetails {
        ContactDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContactDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "number",
                    ContactDetails::get_number_for_reflect,
                    ContactDetails::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ContactDetails::get_name_for_reflect,
                    ContactDetails::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ContactDetails_Avatar>>(
                    "avatar",
                    ContactDetails::get_avatar_for_reflect,
                    ContactDetails::mut_avatar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "color",
                    ContactDetails::get_color_for_reflect,
                    ContactDetails::mut_color_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContactDetails>(
                    "ContactDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContactDetails {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_name();
        self.clear_avatar();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContactDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContactDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContactDetails_Avatar {
    // message fields
    contentType: ::protobuf::SingularField<::std::string::String>,
    length: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContactDetails_Avatar {}

impl ContactDetails_Avatar {
    pub fn new() -> ContactDetails_Avatar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContactDetails_Avatar {
        static mut instance: ::protobuf::lazy::Lazy<ContactDetails_Avatar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContactDetails_Avatar,
        };
        unsafe {
            instance.get(ContactDetails_Avatar::new)
        }
    }

    // optional string contentType = 1;

    pub fn clear_contentType(&mut self) {
        self.contentType.clear();
    }

    pub fn has_contentType(&self) -> bool {
        self.contentType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentType(&mut self, v: ::std::string::String) {
        self.contentType = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contentType(&mut self) -> &mut ::std::string::String {
        if self.contentType.is_none() {
            self.contentType.set_default();
        };
        self.contentType.as_mut().unwrap()
    }

    // Take field
    pub fn take_contentType(&mut self) -> ::std::string::String {
        self.contentType.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_contentType(&self) -> &str {
        match self.contentType.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_contentType_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.contentType
    }

    fn mut_contentType_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.contentType
    }

    // optional uint32 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.length
    }
}

impl ::protobuf::Message for ContactDetails_Avatar {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.contentType)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.length = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.contentType.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.contentType.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.length {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for ContactDetails_Avatar {
    fn new() -> ContactDetails_Avatar {
        ContactDetails_Avatar::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContactDetails_Avatar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "contentType",
                    ContactDetails_Avatar::get_contentType_for_reflect,
                    ContactDetails_Avatar::mut_contentType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "length",
                    ContactDetails_Avatar::get_length_for_reflect,
                    ContactDetails_Avatar::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContactDetails_Avatar>(
                    "ContactDetails_Avatar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContactDetails_Avatar {
    fn clear(&mut self) {
        self.clear_contentType();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContactDetails_Avatar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContactDetails_Avatar {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupDetails {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    members: ::protobuf::RepeatedField<::std::string::String>,
    avatar: ::protobuf::SingularPtrField<GroupDetails_Avatar>,
    active: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GroupDetails {}

impl GroupDetails {
    pub fn new() -> GroupDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GroupDetails {
        static mut instance: ::protobuf::lazy::Lazy<GroupDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupDetails,
        };
        unsafe {
            instance.get(GroupDetails::new)
        }
    }

    // optional bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.id
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated string members = 3;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[::std::string::String] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // optional .signalservice.GroupDetails.Avatar avatar = 4;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: GroupDetails_Avatar) {
        self.avatar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut GroupDetails_Avatar {
        if self.avatar.is_none() {
            self.avatar.set_default();
        };
        self.avatar.as_mut().unwrap()
    }

    // Take field
    pub fn take_avatar(&mut self) -> GroupDetails_Avatar {
        self.avatar.take().unwrap_or_else(|| GroupDetails_Avatar::new())
    }

    pub fn get_avatar(&self) -> &GroupDetails_Avatar {
        self.avatar.as_ref().unwrap_or_else(|| GroupDetails_Avatar::default_instance())
    }

    fn get_avatar_for_reflect(&self) -> &::protobuf::SingularPtrField<GroupDetails_Avatar> {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GroupDetails_Avatar> {
        &mut self.avatar
    }

    // optional bool active = 5;

    pub fn clear_active(&mut self) {
        self.active = ::std::option::Option::None;
    }

    pub fn has_active(&self) -> bool {
        self.active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: bool) {
        self.active = ::std::option::Option::Some(v);
    }

    pub fn get_active(&self) -> bool {
        self.active.unwrap_or(true)
    }

    fn get_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.active
    }

    fn mut_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.active
    }
}

impl ::protobuf::Message for GroupDetails {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.members)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.avatar)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.active = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        for value in &self.members {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(v) = self.avatar.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.active {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        for v in &self.members {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.avatar.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.active {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for GroupDetails {
    fn new() -> GroupDetails {
        GroupDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<GroupDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    GroupDetails::get_id_for_reflect,
                    GroupDetails::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GroupDetails::get_name_for_reflect,
                    GroupDetails::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "members",
                    GroupDetails::get_members_for_reflect,
                    GroupDetails::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GroupDetails_Avatar>>(
                    "avatar",
                    GroupDetails::get_avatar_for_reflect,
                    GroupDetails::mut_avatar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "active",
                    GroupDetails::get_active_for_reflect,
                    GroupDetails::mut_active_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GroupDetails>(
                    "GroupDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GroupDetails {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_members();
        self.clear_avatar();
        self.clear_active();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupDetails_Avatar {
    // message fields
    contentType: ::protobuf::SingularField<::std::string::String>,
    length: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GroupDetails_Avatar {}

impl GroupDetails_Avatar {
    pub fn new() -> GroupDetails_Avatar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GroupDetails_Avatar {
        static mut instance: ::protobuf::lazy::Lazy<GroupDetails_Avatar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupDetails_Avatar,
        };
        unsafe {
            instance.get(GroupDetails_Avatar::new)
        }
    }

    // optional string contentType = 1;

    pub fn clear_contentType(&mut self) {
        self.contentType.clear();
    }

    pub fn has_contentType(&self) -> bool {
        self.contentType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentType(&mut self, v: ::std::string::String) {
        self.contentType = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contentType(&mut self) -> &mut ::std::string::String {
        if self.contentType.is_none() {
            self.contentType.set_default();
        };
        self.contentType.as_mut().unwrap()
    }

    // Take field
    pub fn take_contentType(&mut self) -> ::std::string::String {
        self.contentType.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_contentType(&self) -> &str {
        match self.contentType.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_contentType_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.contentType
    }

    fn mut_contentType_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.contentType
    }

    // optional uint32 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.length
    }
}

impl ::protobuf::Message for GroupDetails_Avatar {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.contentType)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.length = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.contentType.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.contentType.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.length {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for GroupDetails_Avatar {
    fn new() -> GroupDetails_Avatar {
        GroupDetails_Avatar::new()
    }

    fn descriptor_static(_: ::std::option::Option<GroupDetails_Avatar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "contentType",
                    GroupDetails_Avatar::get_contentType_for_reflect,
                    GroupDetails_Avatar::mut_contentType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "length",
                    GroupDetails_Avatar::get_length_for_reflect,
                    GroupDetails_Avatar::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GroupDetails_Avatar>(
                    "GroupDetails_Avatar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GroupDetails_Avatar {
    fn clear(&mut self) {
        self.clear_contentType();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupDetails_Avatar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupDetails_Avatar {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x53, 0x69, 0x67, 0x6e, 0x61,
    0x6c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x22, 0xc3, 0x02,
    0x0a, 0x08, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x12, 0x30, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61,
    0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x12, 0x22, 0x0a, 0x0c, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x65, 0x6c, 0x61,
    0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x72, 0x65, 0x6c, 0x61, 0x79, 0x12, 0x1c,
    0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x24, 0x0a, 0x0d,
    0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x0d, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22, 0x55, 0x0a, 0x04,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10,
    0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x49, 0x50, 0x48, 0x45, 0x52, 0x54, 0x45, 0x58, 0x54, 0x10,
    0x01, 0x12, 0x10, 0x0a, 0x0c, 0x4b, 0x45, 0x59, 0x5f, 0x45, 0x58, 0x43, 0x48, 0x41, 0x4e, 0x47,
    0x45, 0x10, 0x02, 0x12, 0x11, 0x0a, 0x0d, 0x50, 0x52, 0x45, 0x4b, 0x45, 0x59, 0x5f, 0x42, 0x55,
    0x4e, 0x44, 0x4c, 0x45, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x45, 0x43, 0x45, 0x49, 0x50,
    0x54, 0x10, 0x05, 0x22, 0xc3, 0x01, 0x0a, 0x07, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12,
    0x3c, 0x0a, 0x0b, 0x64, 0x61, 0x74, 0x61, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x52, 0x0b, 0x64, 0x61, 0x74, 0x61, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3c, 0x0a,
    0x0b, 0x73, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x0b,
    0x73, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3c, 0x0a, 0x0b, 0x63,
    0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1a, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x0b, 0x63, 0x61,
    0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0xca, 0x04, 0x0a, 0x0b, 0x43, 0x61,
    0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x36, 0x0a, 0x05, 0x6f, 0x66, 0x66,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61,
    0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x2e, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x52, 0x05, 0x6f, 0x66, 0x66, 0x65,
    0x72, 0x12, 0x39, 0x0a, 0x06, 0x61, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x21, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x41, 0x6e,
    0x73, 0x77, 0x65, 0x72, 0x52, 0x06, 0x61, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x12, 0x42, 0x0a, 0x09,
    0x69, 0x63, 0x65, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x24, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x49, 0x63, 0x65, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x09, 0x69, 0x63, 0x65, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x12, 0x39, 0x0a, 0x06, 0x68, 0x61, 0x6e, 0x67, 0x75, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x21, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x48, 0x61, 0x6e,
    0x67, 0x75, 0x70, 0x52, 0x06, 0x68, 0x61, 0x6e, 0x67, 0x75, 0x70, 0x12, 0x33, 0x0a, 0x04, 0x62,
    0x75, 0x73, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x73, 0x69, 0x67, 0x6e,
    0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x42, 0x75, 0x73, 0x79, 0x52, 0x04, 0x62, 0x75, 0x73, 0x79,
    0x1a, 0x39, 0x0a, 0x05, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0x3a, 0x0a, 0x06, 0x41,
    0x6e, 0x73, 0x77, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0x6b, 0x0a, 0x09, 0x49, 0x63, 0x65, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x02, 0x69, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x64, 0x70, 0x4d, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x64, 0x70, 0x4d, 0x69, 0x64, 0x12, 0x24, 0x0a, 0x0d,
    0x73, 0x64, 0x70, 0x4d, 0x4c, 0x69, 0x6e, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0d, 0x73, 0x64, 0x70, 0x4d, 0x4c, 0x69, 0x6e, 0x65, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x12, 0x10, 0x0a, 0x03, 0x73, 0x64, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x03, 0x73, 0x64, 0x70, 0x1a, 0x16, 0x0a, 0x04, 0x42, 0x75, 0x73, 0x79, 0x12, 0x0e, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x1a, 0x18, 0x0a, 0x06,
    0x48, 0x61, 0x6e, 0x67, 0x75, 0x70, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x22, 0x87, 0x02, 0x0a, 0x0b, 0x44, 0x61, 0x74, 0x61, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x12, 0x42, 0x0a, 0x0b, 0x61, 0x74,
    0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x52, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x31,
    0x0a, 0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x47, 0x72,
    0x6f, 0x75, 0x70, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x52, 0x05, 0x67, 0x72, 0x6f, 0x75,
    0x70, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x20, 0x0a, 0x0b, 0x65, 0x78, 0x70, 0x69, 0x72,
    0x65, 0x54, 0x69, 0x6d, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x65, 0x78,
    0x70, 0x69, 0x72, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x72, 0x22, 0x35, 0x0a, 0x05, 0x46, 0x6c, 0x61,
    0x67, 0x73, 0x12, 0x0f, 0x0a, 0x0b, 0x45, 0x4e, 0x44, 0x5f, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f,
    0x4e, 0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x58, 0x50, 0x49, 0x52, 0x41, 0x54, 0x49, 0x4f,
    0x4e, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x52, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x10, 0x02,
    0x22, 0x94, 0x07, 0x0a, 0x0b, 0x53, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x33, 0x0a, 0x04, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x53,
    0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x74, 0x52,
    0x04, 0x73, 0x65, 0x6e, 0x74, 0x12, 0x3f, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x73, 0x52, 0x08, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x63, 0x74, 0x73, 0x12, 0x39, 0x0a, 0x06, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x2e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x52, 0x06, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x73, 0x12, 0x3c, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x22, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x33, 0x0a, 0x04, 0x72, 0x65, 0x61, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x53, 0x79,
    0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x52, 0x04,
    0x72, 0x65, 0x61, 0x64, 0x12, 0x3c, 0x0a, 0x07, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x52, 0x07, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x65, 0x64, 0x1a, 0xb8, 0x01, 0x0a, 0x04, 0x53, 0x65, 0x6e, 0x74, 0x12, 0x20, 0x0a, 0x0b, 0x64,
    0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0b, 0x64, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1c, 0x0a,
    0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x34, 0x0a, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73,
    0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x3a, 0x0a, 0x18, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x18, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x1a, 0x40, 0x0a,
    0x08, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x73, 0x12, 0x34, 0x0a, 0x04, 0x62, 0x6c, 0x6f,
    0x62, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x04, 0x62, 0x6c, 0x6f, 0x62, 0x1a,
    0x3e, 0x0a, 0x06, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x12, 0x34, 0x0a, 0x04, 0x62, 0x6c, 0x6f,
    0x62, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x04, 0x62, 0x6c, 0x6f, 0x62, 0x1a,
    0x23, 0x0a, 0x07, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x1a, 0x82, 0x01, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x3b, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x27,
    0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x53,
    0x79, 0x6e, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x0a,
    0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e,
    0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x4f, 0x4e, 0x54, 0x41, 0x43, 0x54, 0x53, 0x10, 0x01,
    0x12, 0x0a, 0x0a, 0x06, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x53, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07,
    0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x45, 0x44, 0x10, 0x03, 0x1a, 0x3c, 0x0a, 0x04, 0x52, 0x65, 0x61,
    0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0xa1, 0x01, 0x0a, 0x11, 0x41, 0x74, 0x74, 0x61,
    0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x0e, 0x0a,
    0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a,
    0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65,
    0x79, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x68, 0x75, 0x6d, 0x62, 0x6e, 0x61,
    0x69, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x74, 0x68, 0x75, 0x6d, 0x62, 0x6e,
    0x61, 0x69, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x06, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x22, 0x86, 0x02, 0x0a, 0x0c,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x0e, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x02, 0x69, 0x64, 0x12, 0x34, 0x0a, 0x04,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x20, 0x2e, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x47, 0x72, 0x6f, 0x75, 0x70,
    0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x12, 0x38, 0x0a, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x20, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x52, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x22, 0x48, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12,
    0x0a, 0x0a, 0x06, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x44,
    0x45, 0x4c, 0x49, 0x56, 0x45, 0x52, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x51, 0x55, 0x49, 0x54,
    0x10, 0x03, 0x12, 0x10, 0x0a, 0x0c, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x5f, 0x49, 0x4e,
    0x46, 0x4f, 0x10, 0x04, 0x22, 0xd4, 0x01, 0x0a, 0x0e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74,
    0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x3c, 0x0a, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x73, 0x2e, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x52, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61,
    0x72, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x1a, 0x42, 0x0a, 0x06, 0x41, 0x76, 0x61, 0x74, 0x61,
    0x72, 0x12, 0x20, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x22, 0xea, 0x01, 0x0a, 0x0c,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x0e, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x3a, 0x0a, 0x06, 0x61, 0x76,
    0x61, 0x74, 0x61, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x47, 0x72, 0x6f, 0x75, 0x70,
    0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x2e, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x52, 0x06,
    0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x12, 0x1c, 0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x52, 0x06, 0x61, 0x63,
    0x74, 0x69, 0x76, 0x65, 0x1a, 0x42, 0x0a, 0x06, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x12, 0x20,
    0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x42, 0x45, 0x0a, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    0x77, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x73, 0x2e, 0x73,
    0x69, 0x67, 0x6e, 0x61, 0x6c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x70, 0x75, 0x73, 0x68, 0x42, 0x13, 0x53, 0x69, 0x67, 0x6e,
    0x61, 0x6c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x4a,
    0xad, 0x37, 0x0a, 0x07, 0x12, 0x05, 0x06, 0x00, 0xad, 0x01, 0x01, 0x0a, 0x77, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x06, 0x08, 0x15, 0x32, 0x6d, 0x2a, 0x0a, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69,
    0x67, 0x68, 0x74, 0x20, 0x28, 0x43, 0x29, 0x20, 0x32, 0x30, 0x31, 0x34, 0x2d, 0x32, 0x30, 0x31,
    0x36, 0x20, 0x4f, 0x70, 0x65, 0x6e, 0x20, 0x57, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x20, 0x53,
    0x79, 0x73, 0x74, 0x65, 0x6d, 0x73, 0x0a, 0x0a, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x64, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x4c, 0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x6f,
    0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x47, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x08, 0x00, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x07, 0x12, 0x03, 0x08, 0x16, 0x46, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x34,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x09, 0x00, 0x34, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x09, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x01, 0x07, 0x12, 0x03, 0x09, 0x1e, 0x33, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x0b, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x02, 0x12, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x10, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x11, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x11, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x11, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x14, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x14, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x15, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x16, 0x02, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x16, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x17, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x17, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x18, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x18, 0x22, 0x23,
    0x0a, 0x30, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x19, 0x02, 0x24, 0x22, 0x23, 0x20,
    0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x44, 0x61, 0x74, 0x61, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x19, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x19, 0x22, 0x23, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x1a, 0x02, 0x24, 0x22, 0x1f, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x20,
    0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x1a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1a,
    0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x23,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x1e, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1e, 0x0b,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x17, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x1f, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1f, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x20, 0x02, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x20, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x20, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23,
    0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x13, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x00, 0x12, 0x04, 0x24, 0x02, 0x27, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x03, 0x00, 0x01, 0x12, 0x03, 0x24, 0x0a, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25, 0x04, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x25, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x14, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x26, 0x04, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x26, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x14, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x29, 0x02, 0x2c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x01, 0x01,
    0x12, 0x03, 0x29, 0x0a, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x2a, 0x04, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2a, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x2a, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2a, 0x14, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2a, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x2b, 0x04, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x2b, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x2b, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x2b, 0x14, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2b, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x02, 0x12, 0x04, 0x2e, 0x02, 0x33,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x0a, 0x13, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x26, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x14, 0x16, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x24, 0x25, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x02, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30, 0x04, 0x26, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x14, 0x1a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x24, 0x25, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x02, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x31, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x31, 0x14, 0x21, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x02, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x32, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x32, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x32, 0x14, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x32, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x35, 0x02, 0x37, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03,
    0x03, 0x01, 0x12, 0x03, 0x35, 0x0a, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x36, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x36, 0x14, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x36, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x39, 0x02, 0x3b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x04, 0x01, 0x12, 0x03, 0x39,
    0x0a, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x04, 0x02, 0x00, 0x12, 0x03, 0x3a, 0x04,
    0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3a, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3a, 0x0d,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x14,
    0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x3f, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3f, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x15, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3f, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x40, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x40, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40,
    0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x41, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x41, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x41, 0x15, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x41, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x42,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x42, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x42, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x42, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x45, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x45,
    0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x46, 0x02, 0x49, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x46, 0x07, 0x0c, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x47, 0x04, 0x14, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x04, 0x0f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x47, 0x12, 0x13, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x48, 0x04, 0x20, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x48, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x4b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x4b, 0x1e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b, 0x2c,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x02, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4c, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x4c, 0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x4c, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03,
    0x4d, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4d, 0x0b, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x1e, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4d, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x4e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4e,
    0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4e, 0x2c, 0x2d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x4f, 0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x4f, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x52, 0x00, 0x7c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x52, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x53, 0x02, 0x58, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x53, 0x0a, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x54, 0x04, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x54, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x54, 0x19, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x54, 0x34, 0x35, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x55, 0x04, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x55, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x55, 0x19, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x55, 0x34, 0x35, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x56, 0x04, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x56, 0x0d, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x56, 0x19, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x56, 0x34, 0x35, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x57, 0x04, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x57, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x57, 0x19, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x57, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x01, 0x12, 0x04, 0x5a,
    0x02, 0x5c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x01, 0x01, 0x12, 0x03, 0x5a, 0x0a,
    0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x04, 0x28,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5b, 0x04, 0x0c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5b, 0x0d, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x1f, 0x23,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x02, 0x12, 0x04, 0x5e, 0x02, 0x60, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x0a, 0x10, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x04, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5f, 0x0d, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x1f, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x04, 0x03, 0x03, 0x12, 0x04, 0x62, 0x02, 0x64, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03,
    0x03, 0x01, 0x12, 0x03, 0x62, 0x0a, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x63, 0x04, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x63, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x63, 0x14, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x63, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x04, 0x12, 0x04,
    0x66, 0x02, 0x6f, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x04, 0x01, 0x12, 0x03, 0x66,
    0x0a, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x12, 0x04, 0x67, 0x04,
    0x6c, 0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x67,
    0x09, 0x0d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x68, 0x06, 0x13, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x68, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x68, 0x11, 0x12, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x04, 0x03, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x69, 0x06, 0x13, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x03, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x69, 0x06, 0x0e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04,
    0x03, 0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x69, 0x11, 0x12, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x06, 0x13, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x06, 0x0c, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x11,
    0x12, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x6b,
    0x06, 0x13, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x6b, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x03, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x6b, 0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x6e, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x6e, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6e, 0x12, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x04, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x6e, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x05, 0x12, 0x04, 0x71,
    0x02, 0x74, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x05, 0x01, 0x12, 0x03, 0x71, 0x0a,
    0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x05, 0x02, 0x00, 0x12, 0x03, 0x72, 0x04, 0x22,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x72, 0x0d, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x72, 0x14, 0x1a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x72, 0x20, 0x21,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x73, 0x04, 0x22, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x73, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x73, 0x0d, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x73, 0x14, 0x1d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x73, 0x20, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x76, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x76, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x76, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x76, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x76, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x77, 0x02,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x77, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x77, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x02, 0x12, 0x03, 0x78, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x78, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x78,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x14, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x78, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x79, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x79, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x79, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x79, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x06, 0x12, 0x03, 0x7a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7a, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x7a, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05,
    0x12, 0x03, 0x7b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x06, 0x12, 0x03, 0x7b, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7b, 0x14, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x7b, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x05, 0x7e, 0x00, 0x85, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x7e, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x7f, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7f, 0x0b, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7f, 0x13, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x80, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x80, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x80, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x80, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x04, 0x81,
    0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x81, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01, 0x13, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x04, 0x82, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x01, 0x12, 0x04, 0x82, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x82, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x04, 0x12, 0x04, 0x83, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x83, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x83, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x83, 0x01, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x04, 0x83,
    0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x04, 0x84, 0x01, 0x02,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x04, 0x84, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x04, 0x84, 0x01, 0x13, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x04, 0x84, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x06, 0x87, 0x01, 0x00, 0x94, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x06, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x06, 0x04, 0x00,
    0x12, 0x06, 0x88, 0x01, 0x02, 0x8e, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x04, 0x00,
    0x01, 0x12, 0x04, 0x88, 0x01, 0x07, 0x0b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x89, 0x01, 0x04, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x89, 0x01, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0x89, 0x01, 0x13, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x06, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x06, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x06,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x13, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x06, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x06, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x13, 0x14, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x15, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x08, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x13, 0x14, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x15, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x10,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x8d, 0x01, 0x13,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x1d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x90, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x06, 0x12, 0x04, 0x90, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x90, 0x01, 0x1d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x90, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x04, 0x91,
    0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x04, 0x91, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x04, 0x91, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x04, 0x91, 0x01, 0x1d, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x04, 0x91, 0x01, 0x27, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x04, 0x92, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x92, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x04, 0x92, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x01, 0x12, 0x04, 0x92, 0x01, 0x1d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x92, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x04, 0x12, 0x04, 0x93, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x93, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x93, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x93, 0x01, 0x1d, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x04, 0x93,
    0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0x96, 0x01, 0x00, 0xa0, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x16, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x07, 0x03, 0x00, 0x12, 0x06, 0x97, 0x01, 0x02, 0x9a, 0x01, 0x03, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x03, 0x00, 0x01, 0x12, 0x04, 0x97, 0x01, 0x0a, 0x10, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x98, 0x01, 0x04, 0x24, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x98, 0x01, 0x0d, 0x13, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x98, 0x01, 0x14, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98, 0x01, 0x22,
    0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x04,
    0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x99, 0x01,
    0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x99,
    0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x99, 0x01, 0x14, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x99, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0x9c,
    0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9c, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x12, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x02, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x9e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12,
    0x04, 0x9e, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x9e, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9e,
    0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x02,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x12, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x08, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xad, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x08, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x08, 0x03, 0x00,
    0x12, 0x06, 0xa3, 0x01, 0x02, 0xa6, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x03, 0x00,
    0x01, 0x12, 0x04, 0xa3, 0x01, 0x0a, 0x10, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xa4, 0x01, 0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x14, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa5, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x14, 0x1a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0xa8, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa8, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x01, 0x12, 0x04, 0xa9, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xa9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa9, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa9, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9,
    0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0xaa, 0x01, 0x02,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x12, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x04, 0xab, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0xab, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x06, 0x12, 0x04, 0xab, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xab, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xab, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12,
    0x04, 0xac, 0x01, 0x02, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xac, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x04, 0xac,
    0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x04, 0xac, 0x01,
    0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x04, 0xac, 0x01, 0x1d,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x08, 0x12, 0x04, 0xac, 0x01, 0x1f, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x07, 0x12, 0x04, 0xac, 0x01, 0x2a, 0x2e,
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
