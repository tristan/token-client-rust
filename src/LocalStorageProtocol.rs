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
pub struct SessionStructure {
    // message fields
    sessionVersion: ::std::option::Option<u32>,
    localIdentityPublic: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    remoteIdentityPublic: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rootKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    previousCounter: ::std::option::Option<u32>,
    senderChain: ::protobuf::SingularPtrField<SessionStructure_Chain>,
    receiverChains: ::protobuf::RepeatedField<SessionStructure_Chain>,
    pendingKeyExchange: ::protobuf::SingularPtrField<SessionStructure_PendingKeyExchange>,
    pendingPreKey: ::protobuf::SingularPtrField<SessionStructure_PendingPreKey>,
    remoteRegistrationId: ::std::option::Option<u32>,
    localRegistrationId: ::std::option::Option<u32>,
    needsRefresh: ::std::option::Option<bool>,
    aliceBaseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionStructure {}

impl SessionStructure {
    pub fn new() -> SessionStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionStructure {
        static mut instance: ::protobuf::lazy::Lazy<SessionStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionStructure,
        };
        unsafe {
            instance.get(SessionStructure::new)
        }
    }

    // optional uint32 sessionVersion = 1;

    pub fn clear_sessionVersion(&mut self) {
        self.sessionVersion = ::std::option::Option::None;
    }

    pub fn has_sessionVersion(&self) -> bool {
        self.sessionVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sessionVersion(&mut self, v: u32) {
        self.sessionVersion = ::std::option::Option::Some(v);
    }

    pub fn get_sessionVersion(&self) -> u32 {
        self.sessionVersion.unwrap_or(0)
    }

    fn get_sessionVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sessionVersion
    }

    fn mut_sessionVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sessionVersion
    }

    // optional bytes localIdentityPublic = 2;

    pub fn clear_localIdentityPublic(&mut self) {
        self.localIdentityPublic.clear();
    }

    pub fn has_localIdentityPublic(&self) -> bool {
        self.localIdentityPublic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localIdentityPublic(&mut self, v: ::std::vec::Vec<u8>) {
        self.localIdentityPublic = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localIdentityPublic(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localIdentityPublic.is_none() {
            self.localIdentityPublic.set_default();
        };
        self.localIdentityPublic.as_mut().unwrap()
    }

    // Take field
    pub fn take_localIdentityPublic(&mut self) -> ::std::vec::Vec<u8> {
        self.localIdentityPublic.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localIdentityPublic(&self) -> &[u8] {
        match self.localIdentityPublic.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localIdentityPublic_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localIdentityPublic
    }

    fn mut_localIdentityPublic_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localIdentityPublic
    }

    // optional bytes remoteIdentityPublic = 3;

    pub fn clear_remoteIdentityPublic(&mut self) {
        self.remoteIdentityPublic.clear();
    }

    pub fn has_remoteIdentityPublic(&self) -> bool {
        self.remoteIdentityPublic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remoteIdentityPublic(&mut self, v: ::std::vec::Vec<u8>) {
        self.remoteIdentityPublic = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remoteIdentityPublic(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.remoteIdentityPublic.is_none() {
            self.remoteIdentityPublic.set_default();
        };
        self.remoteIdentityPublic.as_mut().unwrap()
    }

    // Take field
    pub fn take_remoteIdentityPublic(&mut self) -> ::std::vec::Vec<u8> {
        self.remoteIdentityPublic.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_remoteIdentityPublic(&self) -> &[u8] {
        match self.remoteIdentityPublic.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_remoteIdentityPublic_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.remoteIdentityPublic
    }

    fn mut_remoteIdentityPublic_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.remoteIdentityPublic
    }

    // optional bytes rootKey = 4;

    pub fn clear_rootKey(&mut self) {
        self.rootKey.clear();
    }

    pub fn has_rootKey(&self) -> bool {
        self.rootKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rootKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.rootKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rootKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.rootKey.is_none() {
            self.rootKey.set_default();
        };
        self.rootKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_rootKey(&mut self) -> ::std::vec::Vec<u8> {
        self.rootKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_rootKey(&self) -> &[u8] {
        match self.rootKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_rootKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.rootKey
    }

    fn mut_rootKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.rootKey
    }

    // optional uint32 previousCounter = 5;

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

    // optional .textsecure.SessionStructure.Chain senderChain = 6;

    pub fn clear_senderChain(&mut self) {
        self.senderChain.clear();
    }

    pub fn has_senderChain(&self) -> bool {
        self.senderChain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderChain(&mut self, v: SessionStructure_Chain) {
        self.senderChain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_senderChain(&mut self) -> &mut SessionStructure_Chain {
        if self.senderChain.is_none() {
            self.senderChain.set_default();
        };
        self.senderChain.as_mut().unwrap()
    }

    // Take field
    pub fn take_senderChain(&mut self) -> SessionStructure_Chain {
        self.senderChain.take().unwrap_or_else(|| SessionStructure_Chain::new())
    }

    pub fn get_senderChain(&self) -> &SessionStructure_Chain {
        self.senderChain.as_ref().unwrap_or_else(|| SessionStructure_Chain::default_instance())
    }

    fn get_senderChain_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionStructure_Chain> {
        &self.senderChain
    }

    fn mut_senderChain_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionStructure_Chain> {
        &mut self.senderChain
    }

    // repeated .textsecure.SessionStructure.Chain receiverChains = 7;

    pub fn clear_receiverChains(&mut self) {
        self.receiverChains.clear();
    }

    // Param is passed by value, moved
    pub fn set_receiverChains(&mut self, v: ::protobuf::RepeatedField<SessionStructure_Chain>) {
        self.receiverChains = v;
    }

    // Mutable pointer to the field.
    pub fn mut_receiverChains(&mut self) -> &mut ::protobuf::RepeatedField<SessionStructure_Chain> {
        &mut self.receiverChains
    }

    // Take field
    pub fn take_receiverChains(&mut self) -> ::protobuf::RepeatedField<SessionStructure_Chain> {
        ::std::mem::replace(&mut self.receiverChains, ::protobuf::RepeatedField::new())
    }

    pub fn get_receiverChains(&self) -> &[SessionStructure_Chain] {
        &self.receiverChains
    }

    fn get_receiverChains_for_reflect(&self) -> &::protobuf::RepeatedField<SessionStructure_Chain> {
        &self.receiverChains
    }

    fn mut_receiverChains_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SessionStructure_Chain> {
        &mut self.receiverChains
    }

    // optional .textsecure.SessionStructure.PendingKeyExchange pendingKeyExchange = 8;

    pub fn clear_pendingKeyExchange(&mut self) {
        self.pendingKeyExchange.clear();
    }

    pub fn has_pendingKeyExchange(&self) -> bool {
        self.pendingKeyExchange.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pendingKeyExchange(&mut self, v: SessionStructure_PendingKeyExchange) {
        self.pendingKeyExchange = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pendingKeyExchange(&mut self) -> &mut SessionStructure_PendingKeyExchange {
        if self.pendingKeyExchange.is_none() {
            self.pendingKeyExchange.set_default();
        };
        self.pendingKeyExchange.as_mut().unwrap()
    }

    // Take field
    pub fn take_pendingKeyExchange(&mut self) -> SessionStructure_PendingKeyExchange {
        self.pendingKeyExchange.take().unwrap_or_else(|| SessionStructure_PendingKeyExchange::new())
    }

    pub fn get_pendingKeyExchange(&self) -> &SessionStructure_PendingKeyExchange {
        self.pendingKeyExchange.as_ref().unwrap_or_else(|| SessionStructure_PendingKeyExchange::default_instance())
    }

    fn get_pendingKeyExchange_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionStructure_PendingKeyExchange> {
        &self.pendingKeyExchange
    }

    fn mut_pendingKeyExchange_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionStructure_PendingKeyExchange> {
        &mut self.pendingKeyExchange
    }

    // optional .textsecure.SessionStructure.PendingPreKey pendingPreKey = 9;

    pub fn clear_pendingPreKey(&mut self) {
        self.pendingPreKey.clear();
    }

    pub fn has_pendingPreKey(&self) -> bool {
        self.pendingPreKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pendingPreKey(&mut self, v: SessionStructure_PendingPreKey) {
        self.pendingPreKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pendingPreKey(&mut self) -> &mut SessionStructure_PendingPreKey {
        if self.pendingPreKey.is_none() {
            self.pendingPreKey.set_default();
        };
        self.pendingPreKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_pendingPreKey(&mut self) -> SessionStructure_PendingPreKey {
        self.pendingPreKey.take().unwrap_or_else(|| SessionStructure_PendingPreKey::new())
    }

    pub fn get_pendingPreKey(&self) -> &SessionStructure_PendingPreKey {
        self.pendingPreKey.as_ref().unwrap_or_else(|| SessionStructure_PendingPreKey::default_instance())
    }

    fn get_pendingPreKey_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionStructure_PendingPreKey> {
        &self.pendingPreKey
    }

    fn mut_pendingPreKey_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionStructure_PendingPreKey> {
        &mut self.pendingPreKey
    }

    // optional uint32 remoteRegistrationId = 10;

    pub fn clear_remoteRegistrationId(&mut self) {
        self.remoteRegistrationId = ::std::option::Option::None;
    }

    pub fn has_remoteRegistrationId(&self) -> bool {
        self.remoteRegistrationId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remoteRegistrationId(&mut self, v: u32) {
        self.remoteRegistrationId = ::std::option::Option::Some(v);
    }

    pub fn get_remoteRegistrationId(&self) -> u32 {
        self.remoteRegistrationId.unwrap_or(0)
    }

    fn get_remoteRegistrationId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.remoteRegistrationId
    }

    fn mut_remoteRegistrationId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.remoteRegistrationId
    }

    // optional uint32 localRegistrationId = 11;

    pub fn clear_localRegistrationId(&mut self) {
        self.localRegistrationId = ::std::option::Option::None;
    }

    pub fn has_localRegistrationId(&self) -> bool {
        self.localRegistrationId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localRegistrationId(&mut self, v: u32) {
        self.localRegistrationId = ::std::option::Option::Some(v);
    }

    pub fn get_localRegistrationId(&self) -> u32 {
        self.localRegistrationId.unwrap_or(0)
    }

    fn get_localRegistrationId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.localRegistrationId
    }

    fn mut_localRegistrationId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.localRegistrationId
    }

    // optional bool needsRefresh = 12;

    pub fn clear_needsRefresh(&mut self) {
        self.needsRefresh = ::std::option::Option::None;
    }

    pub fn has_needsRefresh(&self) -> bool {
        self.needsRefresh.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needsRefresh(&mut self, v: bool) {
        self.needsRefresh = ::std::option::Option::Some(v);
    }

    pub fn get_needsRefresh(&self) -> bool {
        self.needsRefresh.unwrap_or(false)
    }

    fn get_needsRefresh_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needsRefresh
    }

    fn mut_needsRefresh_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needsRefresh
    }

    // optional bytes aliceBaseKey = 13;

    pub fn clear_aliceBaseKey(&mut self) {
        self.aliceBaseKey.clear();
    }

    pub fn has_aliceBaseKey(&self) -> bool {
        self.aliceBaseKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aliceBaseKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.aliceBaseKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_aliceBaseKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.aliceBaseKey.is_none() {
            self.aliceBaseKey.set_default();
        };
        self.aliceBaseKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_aliceBaseKey(&mut self) -> ::std::vec::Vec<u8> {
        self.aliceBaseKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_aliceBaseKey(&self) -> &[u8] {
        match self.aliceBaseKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_aliceBaseKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.aliceBaseKey
    }

    fn mut_aliceBaseKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.aliceBaseKey
    }
}

impl ::protobuf::Message for SessionStructure {
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
                    self.sessionVersion = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localIdentityPublic)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.remoteIdentityPublic)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.rootKey)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.previousCounter = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.senderChain)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.receiverChains)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pendingKeyExchange)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pendingPreKey)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.remoteRegistrationId = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.localRegistrationId = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.needsRefresh = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.aliceBaseKey)?;
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
        if let Some(v) = self.sessionVersion {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.localIdentityPublic.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.remoteIdentityPublic.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.rootKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.previousCounter {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.senderChain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.receiverChains {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.pendingKeyExchange.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.pendingPreKey.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.remoteRegistrationId {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.localRegistrationId {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.needsRefresh {
            my_size += 2;
        };
        if let Some(v) = self.aliceBaseKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(13, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sessionVersion {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.localIdentityPublic.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.remoteIdentityPublic.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.rootKey.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.previousCounter {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.senderChain.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.receiverChains {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.pendingKeyExchange.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.pendingPreKey.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.remoteRegistrationId {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.localRegistrationId {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.needsRefresh {
            os.write_bool(12, v)?;
        };
        if let Some(v) = self.aliceBaseKey.as_ref() {
            os.write_bytes(13, &v)?;
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

impl ::protobuf::MessageStatic for SessionStructure {
    fn new() -> SessionStructure {
        SessionStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sessionVersion",
                    SessionStructure::get_sessionVersion_for_reflect,
                    SessionStructure::mut_sessionVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localIdentityPublic",
                    SessionStructure::get_localIdentityPublic_for_reflect,
                    SessionStructure::mut_localIdentityPublic_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "remoteIdentityPublic",
                    SessionStructure::get_remoteIdentityPublic_for_reflect,
                    SessionStructure::mut_remoteIdentityPublic_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "rootKey",
                    SessionStructure::get_rootKey_for_reflect,
                    SessionStructure::mut_rootKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "previousCounter",
                    SessionStructure::get_previousCounter_for_reflect,
                    SessionStructure::mut_previousCounter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure_Chain>>(
                    "senderChain",
                    SessionStructure::get_senderChain_for_reflect,
                    SessionStructure::mut_senderChain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure_Chain>>(
                    "receiverChains",
                    SessionStructure::get_receiverChains_for_reflect,
                    SessionStructure::mut_receiverChains_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure_PendingKeyExchange>>(
                    "pendingKeyExchange",
                    SessionStructure::get_pendingKeyExchange_for_reflect,
                    SessionStructure::mut_pendingKeyExchange_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure_PendingPreKey>>(
                    "pendingPreKey",
                    SessionStructure::get_pendingPreKey_for_reflect,
                    SessionStructure::mut_pendingPreKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "remoteRegistrationId",
                    SessionStructure::get_remoteRegistrationId_for_reflect,
                    SessionStructure::mut_remoteRegistrationId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "localRegistrationId",
                    SessionStructure::get_localRegistrationId_for_reflect,
                    SessionStructure::mut_localRegistrationId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needsRefresh",
                    SessionStructure::get_needsRefresh_for_reflect,
                    SessionStructure::mut_needsRefresh_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "aliceBaseKey",
                    SessionStructure::get_aliceBaseKey_for_reflect,
                    SessionStructure::mut_aliceBaseKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionStructure>(
                    "SessionStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionStructure {
    fn clear(&mut self) {
        self.clear_sessionVersion();
        self.clear_localIdentityPublic();
        self.clear_remoteIdentityPublic();
        self.clear_rootKey();
        self.clear_previousCounter();
        self.clear_senderChain();
        self.clear_receiverChains();
        self.clear_pendingKeyExchange();
        self.clear_pendingPreKey();
        self.clear_remoteRegistrationId();
        self.clear_localRegistrationId();
        self.clear_needsRefresh();
        self.clear_aliceBaseKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionStructure_Chain {
    // message fields
    senderRatchetKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    senderRatchetKeyPrivate: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    chainKey: ::protobuf::SingularPtrField<SessionStructure_Chain_ChainKey>,
    messageKeys: ::protobuf::RepeatedField<SessionStructure_Chain_MessageKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionStructure_Chain {}

impl SessionStructure_Chain {
    pub fn new() -> SessionStructure_Chain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionStructure_Chain {
        static mut instance: ::protobuf::lazy::Lazy<SessionStructure_Chain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionStructure_Chain,
        };
        unsafe {
            instance.get(SessionStructure_Chain::new)
        }
    }

    // optional bytes senderRatchetKey = 1;

    pub fn clear_senderRatchetKey(&mut self) {
        self.senderRatchetKey.clear();
    }

    pub fn has_senderRatchetKey(&self) -> bool {
        self.senderRatchetKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderRatchetKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.senderRatchetKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_senderRatchetKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.senderRatchetKey.is_none() {
            self.senderRatchetKey.set_default();
        };
        self.senderRatchetKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_senderRatchetKey(&mut self) -> ::std::vec::Vec<u8> {
        self.senderRatchetKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_senderRatchetKey(&self) -> &[u8] {
        match self.senderRatchetKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_senderRatchetKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.senderRatchetKey
    }

    fn mut_senderRatchetKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.senderRatchetKey
    }

    // optional bytes senderRatchetKeyPrivate = 2;

    pub fn clear_senderRatchetKeyPrivate(&mut self) {
        self.senderRatchetKeyPrivate.clear();
    }

    pub fn has_senderRatchetKeyPrivate(&self) -> bool {
        self.senderRatchetKeyPrivate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderRatchetKeyPrivate(&mut self, v: ::std::vec::Vec<u8>) {
        self.senderRatchetKeyPrivate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_senderRatchetKeyPrivate(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.senderRatchetKeyPrivate.is_none() {
            self.senderRatchetKeyPrivate.set_default();
        };
        self.senderRatchetKeyPrivate.as_mut().unwrap()
    }

    // Take field
    pub fn take_senderRatchetKeyPrivate(&mut self) -> ::std::vec::Vec<u8> {
        self.senderRatchetKeyPrivate.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_senderRatchetKeyPrivate(&self) -> &[u8] {
        match self.senderRatchetKeyPrivate.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_senderRatchetKeyPrivate_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.senderRatchetKeyPrivate
    }

    fn mut_senderRatchetKeyPrivate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.senderRatchetKeyPrivate
    }

    // optional .textsecure.SessionStructure.Chain.ChainKey chainKey = 3;

    pub fn clear_chainKey(&mut self) {
        self.chainKey.clear();
    }

    pub fn has_chainKey(&self) -> bool {
        self.chainKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chainKey(&mut self, v: SessionStructure_Chain_ChainKey) {
        self.chainKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chainKey(&mut self) -> &mut SessionStructure_Chain_ChainKey {
        if self.chainKey.is_none() {
            self.chainKey.set_default();
        };
        self.chainKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_chainKey(&mut self) -> SessionStructure_Chain_ChainKey {
        self.chainKey.take().unwrap_or_else(|| SessionStructure_Chain_ChainKey::new())
    }

    pub fn get_chainKey(&self) -> &SessionStructure_Chain_ChainKey {
        self.chainKey.as_ref().unwrap_or_else(|| SessionStructure_Chain_ChainKey::default_instance())
    }

    fn get_chainKey_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionStructure_Chain_ChainKey> {
        &self.chainKey
    }

    fn mut_chainKey_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionStructure_Chain_ChainKey> {
        &mut self.chainKey
    }

    // repeated .textsecure.SessionStructure.Chain.MessageKey messageKeys = 4;

    pub fn clear_messageKeys(&mut self) {
        self.messageKeys.clear();
    }

    // Param is passed by value, moved
    pub fn set_messageKeys(&mut self, v: ::protobuf::RepeatedField<SessionStructure_Chain_MessageKey>) {
        self.messageKeys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messageKeys(&mut self) -> &mut ::protobuf::RepeatedField<SessionStructure_Chain_MessageKey> {
        &mut self.messageKeys
    }

    // Take field
    pub fn take_messageKeys(&mut self) -> ::protobuf::RepeatedField<SessionStructure_Chain_MessageKey> {
        ::std::mem::replace(&mut self.messageKeys, ::protobuf::RepeatedField::new())
    }

    pub fn get_messageKeys(&self) -> &[SessionStructure_Chain_MessageKey] {
        &self.messageKeys
    }

    fn get_messageKeys_for_reflect(&self) -> &::protobuf::RepeatedField<SessionStructure_Chain_MessageKey> {
        &self.messageKeys
    }

    fn mut_messageKeys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SessionStructure_Chain_MessageKey> {
        &mut self.messageKeys
    }
}

impl ::protobuf::Message for SessionStructure_Chain {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.senderRatchetKey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.senderRatchetKeyPrivate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.chainKey)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messageKeys)?;
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
        if let Some(v) = self.senderRatchetKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.senderRatchetKeyPrivate.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.chainKey.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.messageKeys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.senderRatchetKey.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.senderRatchetKeyPrivate.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.chainKey.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.messageKeys {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SessionStructure_Chain {
    fn new() -> SessionStructure_Chain {
        SessionStructure_Chain::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionStructure_Chain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "senderRatchetKey",
                    SessionStructure_Chain::get_senderRatchetKey_for_reflect,
                    SessionStructure_Chain::mut_senderRatchetKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "senderRatchetKeyPrivate",
                    SessionStructure_Chain::get_senderRatchetKeyPrivate_for_reflect,
                    SessionStructure_Chain::mut_senderRatchetKeyPrivate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure_Chain_ChainKey>>(
                    "chainKey",
                    SessionStructure_Chain::get_chainKey_for_reflect,
                    SessionStructure_Chain::mut_chainKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure_Chain_MessageKey>>(
                    "messageKeys",
                    SessionStructure_Chain::get_messageKeys_for_reflect,
                    SessionStructure_Chain::mut_messageKeys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionStructure_Chain>(
                    "SessionStructure_Chain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionStructure_Chain {
    fn clear(&mut self) {
        self.clear_senderRatchetKey();
        self.clear_senderRatchetKeyPrivate();
        self.clear_chainKey();
        self.clear_messageKeys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionStructure_Chain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionStructure_Chain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionStructure_Chain_ChainKey {
    // message fields
    index: ::std::option::Option<u32>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionStructure_Chain_ChainKey {}

impl SessionStructure_Chain_ChainKey {
    pub fn new() -> SessionStructure_Chain_ChainKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionStructure_Chain_ChainKey {
        static mut instance: ::protobuf::lazy::Lazy<SessionStructure_Chain_ChainKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionStructure_Chain_ChainKey,
        };
        unsafe {
            instance.get(SessionStructure_Chain_ChainKey::new)
        }
    }

    // optional uint32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }

    // optional bytes key = 2;

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
}

impl ::protobuf::Message for SessionStructure_Chain_ChainKey {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.key.as_ref() {
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

impl ::protobuf::MessageStatic for SessionStructure_Chain_ChainKey {
    fn new() -> SessionStructure_Chain_ChainKey {
        SessionStructure_Chain_ChainKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionStructure_Chain_ChainKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    SessionStructure_Chain_ChainKey::get_index_for_reflect,
                    SessionStructure_Chain_ChainKey::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    SessionStructure_Chain_ChainKey::get_key_for_reflect,
                    SessionStructure_Chain_ChainKey::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionStructure_Chain_ChainKey>(
                    "SessionStructure_Chain_ChainKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionStructure_Chain_ChainKey {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionStructure_Chain_ChainKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionStructure_Chain_ChainKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionStructure_Chain_MessageKey {
    // message fields
    index: ::std::option::Option<u32>,
    cipherKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    macKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionStructure_Chain_MessageKey {}

impl SessionStructure_Chain_MessageKey {
    pub fn new() -> SessionStructure_Chain_MessageKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionStructure_Chain_MessageKey {
        static mut instance: ::protobuf::lazy::Lazy<SessionStructure_Chain_MessageKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionStructure_Chain_MessageKey,
        };
        unsafe {
            instance.get(SessionStructure_Chain_MessageKey::new)
        }
    }

    // optional uint32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }

    // optional bytes cipherKey = 2;

    pub fn clear_cipherKey(&mut self) {
        self.cipherKey.clear();
    }

    pub fn has_cipherKey(&self) -> bool {
        self.cipherKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cipherKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.cipherKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cipherKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cipherKey.is_none() {
            self.cipherKey.set_default();
        };
        self.cipherKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_cipherKey(&mut self) -> ::std::vec::Vec<u8> {
        self.cipherKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cipherKey(&self) -> &[u8] {
        match self.cipherKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cipherKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cipherKey
    }

    fn mut_cipherKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cipherKey
    }

    // optional bytes macKey = 3;

    pub fn clear_macKey(&mut self) {
        self.macKey.clear();
    }

    pub fn has_macKey(&self) -> bool {
        self.macKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_macKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.macKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_macKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.macKey.is_none() {
            self.macKey.set_default();
        };
        self.macKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_macKey(&mut self) -> ::std::vec::Vec<u8> {
        self.macKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_macKey(&self) -> &[u8] {
        match self.macKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_macKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.macKey
    }

    fn mut_macKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.macKey
    }

    // optional bytes iv = 4;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        };
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv(&self) -> &[u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_iv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.iv
    }

    fn mut_iv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.iv
    }
}

impl ::protobuf::Message for SessionStructure_Chain_MessageKey {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cipherKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.macKey)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.iv)?;
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cipherKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.macKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.iv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.cipherKey.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.macKey.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.iv.as_ref() {
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

impl ::protobuf::MessageStatic for SessionStructure_Chain_MessageKey {
    fn new() -> SessionStructure_Chain_MessageKey {
        SessionStructure_Chain_MessageKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionStructure_Chain_MessageKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    SessionStructure_Chain_MessageKey::get_index_for_reflect,
                    SessionStructure_Chain_MessageKey::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cipherKey",
                    SessionStructure_Chain_MessageKey::get_cipherKey_for_reflect,
                    SessionStructure_Chain_MessageKey::mut_cipherKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "macKey",
                    SessionStructure_Chain_MessageKey::get_macKey_for_reflect,
                    SessionStructure_Chain_MessageKey::mut_macKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "iv",
                    SessionStructure_Chain_MessageKey::get_iv_for_reflect,
                    SessionStructure_Chain_MessageKey::mut_iv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionStructure_Chain_MessageKey>(
                    "SessionStructure_Chain_MessageKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionStructure_Chain_MessageKey {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_cipherKey();
        self.clear_macKey();
        self.clear_iv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionStructure_Chain_MessageKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionStructure_Chain_MessageKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionStructure_PendingKeyExchange {
    // message fields
    sequence: ::std::option::Option<u32>,
    localBaseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    localBaseKeyPrivate: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    localRatchetKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    localRatchetKeyPrivate: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    localIdentityKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    localIdentityKeyPrivate: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionStructure_PendingKeyExchange {}

impl SessionStructure_PendingKeyExchange {
    pub fn new() -> SessionStructure_PendingKeyExchange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionStructure_PendingKeyExchange {
        static mut instance: ::protobuf::lazy::Lazy<SessionStructure_PendingKeyExchange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionStructure_PendingKeyExchange,
        };
        unsafe {
            instance.get(SessionStructure_PendingKeyExchange::new)
        }
    }

    // optional uint32 sequence = 1;

    pub fn clear_sequence(&mut self) {
        self.sequence = ::std::option::Option::None;
    }

    pub fn has_sequence(&self) -> bool {
        self.sequence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence(&mut self, v: u32) {
        self.sequence = ::std::option::Option::Some(v);
    }

    pub fn get_sequence(&self) -> u32 {
        self.sequence.unwrap_or(0)
    }

    fn get_sequence_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sequence
    }

    fn mut_sequence_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sequence
    }

    // optional bytes localBaseKey = 2;

    pub fn clear_localBaseKey(&mut self) {
        self.localBaseKey.clear();
    }

    pub fn has_localBaseKey(&self) -> bool {
        self.localBaseKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localBaseKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.localBaseKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localBaseKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localBaseKey.is_none() {
            self.localBaseKey.set_default();
        };
        self.localBaseKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_localBaseKey(&mut self) -> ::std::vec::Vec<u8> {
        self.localBaseKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localBaseKey(&self) -> &[u8] {
        match self.localBaseKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localBaseKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localBaseKey
    }

    fn mut_localBaseKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localBaseKey
    }

    // optional bytes localBaseKeyPrivate = 3;

    pub fn clear_localBaseKeyPrivate(&mut self) {
        self.localBaseKeyPrivate.clear();
    }

    pub fn has_localBaseKeyPrivate(&self) -> bool {
        self.localBaseKeyPrivate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localBaseKeyPrivate(&mut self, v: ::std::vec::Vec<u8>) {
        self.localBaseKeyPrivate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localBaseKeyPrivate(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localBaseKeyPrivate.is_none() {
            self.localBaseKeyPrivate.set_default();
        };
        self.localBaseKeyPrivate.as_mut().unwrap()
    }

    // Take field
    pub fn take_localBaseKeyPrivate(&mut self) -> ::std::vec::Vec<u8> {
        self.localBaseKeyPrivate.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localBaseKeyPrivate(&self) -> &[u8] {
        match self.localBaseKeyPrivate.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localBaseKeyPrivate_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localBaseKeyPrivate
    }

    fn mut_localBaseKeyPrivate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localBaseKeyPrivate
    }

    // optional bytes localRatchetKey = 4;

    pub fn clear_localRatchetKey(&mut self) {
        self.localRatchetKey.clear();
    }

    pub fn has_localRatchetKey(&self) -> bool {
        self.localRatchetKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localRatchetKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.localRatchetKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localRatchetKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localRatchetKey.is_none() {
            self.localRatchetKey.set_default();
        };
        self.localRatchetKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_localRatchetKey(&mut self) -> ::std::vec::Vec<u8> {
        self.localRatchetKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localRatchetKey(&self) -> &[u8] {
        match self.localRatchetKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localRatchetKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localRatchetKey
    }

    fn mut_localRatchetKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localRatchetKey
    }

    // optional bytes localRatchetKeyPrivate = 5;

    pub fn clear_localRatchetKeyPrivate(&mut self) {
        self.localRatchetKeyPrivate.clear();
    }

    pub fn has_localRatchetKeyPrivate(&self) -> bool {
        self.localRatchetKeyPrivate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localRatchetKeyPrivate(&mut self, v: ::std::vec::Vec<u8>) {
        self.localRatchetKeyPrivate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localRatchetKeyPrivate(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localRatchetKeyPrivate.is_none() {
            self.localRatchetKeyPrivate.set_default();
        };
        self.localRatchetKeyPrivate.as_mut().unwrap()
    }

    // Take field
    pub fn take_localRatchetKeyPrivate(&mut self) -> ::std::vec::Vec<u8> {
        self.localRatchetKeyPrivate.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localRatchetKeyPrivate(&self) -> &[u8] {
        match self.localRatchetKeyPrivate.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localRatchetKeyPrivate_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localRatchetKeyPrivate
    }

    fn mut_localRatchetKeyPrivate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localRatchetKeyPrivate
    }

    // optional bytes localIdentityKey = 7;

    pub fn clear_localIdentityKey(&mut self) {
        self.localIdentityKey.clear();
    }

    pub fn has_localIdentityKey(&self) -> bool {
        self.localIdentityKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localIdentityKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.localIdentityKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localIdentityKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localIdentityKey.is_none() {
            self.localIdentityKey.set_default();
        };
        self.localIdentityKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_localIdentityKey(&mut self) -> ::std::vec::Vec<u8> {
        self.localIdentityKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localIdentityKey(&self) -> &[u8] {
        match self.localIdentityKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localIdentityKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localIdentityKey
    }

    fn mut_localIdentityKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localIdentityKey
    }

    // optional bytes localIdentityKeyPrivate = 8;

    pub fn clear_localIdentityKeyPrivate(&mut self) {
        self.localIdentityKeyPrivate.clear();
    }

    pub fn has_localIdentityKeyPrivate(&self) -> bool {
        self.localIdentityKeyPrivate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localIdentityKeyPrivate(&mut self, v: ::std::vec::Vec<u8>) {
        self.localIdentityKeyPrivate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localIdentityKeyPrivate(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.localIdentityKeyPrivate.is_none() {
            self.localIdentityKeyPrivate.set_default();
        };
        self.localIdentityKeyPrivate.as_mut().unwrap()
    }

    // Take field
    pub fn take_localIdentityKeyPrivate(&mut self) -> ::std::vec::Vec<u8> {
        self.localIdentityKeyPrivate.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_localIdentityKeyPrivate(&self) -> &[u8] {
        match self.localIdentityKeyPrivate.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_localIdentityKeyPrivate_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.localIdentityKeyPrivate
    }

    fn mut_localIdentityKeyPrivate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.localIdentityKeyPrivate
    }
}

impl ::protobuf::Message for SessionStructure_PendingKeyExchange {
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
                    self.sequence = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localBaseKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localBaseKeyPrivate)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localRatchetKey)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localRatchetKeyPrivate)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localIdentityKey)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.localIdentityKeyPrivate)?;
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
        if let Some(v) = self.sequence {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.localBaseKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.localBaseKeyPrivate.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.localRatchetKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.localRatchetKeyPrivate.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.localIdentityKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        if let Some(v) = self.localIdentityKeyPrivate.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sequence {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.localBaseKey.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.localBaseKeyPrivate.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.localRatchetKey.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.localRatchetKeyPrivate.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.localIdentityKey.as_ref() {
            os.write_bytes(7, &v)?;
        };
        if let Some(v) = self.localIdentityKeyPrivate.as_ref() {
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

impl ::protobuf::MessageStatic for SessionStructure_PendingKeyExchange {
    fn new() -> SessionStructure_PendingKeyExchange {
        SessionStructure_PendingKeyExchange::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionStructure_PendingKeyExchange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sequence",
                    SessionStructure_PendingKeyExchange::get_sequence_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_sequence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localBaseKey",
                    SessionStructure_PendingKeyExchange::get_localBaseKey_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_localBaseKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localBaseKeyPrivate",
                    SessionStructure_PendingKeyExchange::get_localBaseKeyPrivate_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_localBaseKeyPrivate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localRatchetKey",
                    SessionStructure_PendingKeyExchange::get_localRatchetKey_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_localRatchetKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localRatchetKeyPrivate",
                    SessionStructure_PendingKeyExchange::get_localRatchetKeyPrivate_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_localRatchetKeyPrivate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localIdentityKey",
                    SessionStructure_PendingKeyExchange::get_localIdentityKey_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_localIdentityKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "localIdentityKeyPrivate",
                    SessionStructure_PendingKeyExchange::get_localIdentityKeyPrivate_for_reflect,
                    SessionStructure_PendingKeyExchange::mut_localIdentityKeyPrivate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionStructure_PendingKeyExchange>(
                    "SessionStructure_PendingKeyExchange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionStructure_PendingKeyExchange {
    fn clear(&mut self) {
        self.clear_sequence();
        self.clear_localBaseKey();
        self.clear_localBaseKeyPrivate();
        self.clear_localRatchetKey();
        self.clear_localRatchetKeyPrivate();
        self.clear_localIdentityKey();
        self.clear_localIdentityKeyPrivate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionStructure_PendingKeyExchange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionStructure_PendingKeyExchange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionStructure_PendingPreKey {
    // message fields
    preKeyId: ::std::option::Option<u32>,
    signedPreKeyId: ::std::option::Option<i32>,
    baseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionStructure_PendingPreKey {}

impl SessionStructure_PendingPreKey {
    pub fn new() -> SessionStructure_PendingPreKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionStructure_PendingPreKey {
        static mut instance: ::protobuf::lazy::Lazy<SessionStructure_PendingPreKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionStructure_PendingPreKey,
        };
        unsafe {
            instance.get(SessionStructure_PendingPreKey::new)
        }
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

    // optional int32 signedPreKeyId = 3;

    pub fn clear_signedPreKeyId(&mut self) {
        self.signedPreKeyId = ::std::option::Option::None;
    }

    pub fn has_signedPreKeyId(&self) -> bool {
        self.signedPreKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signedPreKeyId(&mut self, v: i32) {
        self.signedPreKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_signedPreKeyId(&self) -> i32 {
        self.signedPreKeyId.unwrap_or(0)
    }

    fn get_signedPreKeyId_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.signedPreKeyId
    }

    fn mut_signedPreKeyId_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
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
}

impl ::protobuf::Message for SessionStructure_PendingPreKey {
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
                    self.preKeyId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.signedPreKeyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.baseKey)?;
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
        if let Some(v) = self.preKeyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.signedPreKeyId {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.baseKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.preKeyId {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.signedPreKeyId {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.baseKey.as_ref() {
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

impl ::protobuf::MessageStatic for SessionStructure_PendingPreKey {
    fn new() -> SessionStructure_PendingPreKey {
        SessionStructure_PendingPreKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionStructure_PendingPreKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "preKeyId",
                    SessionStructure_PendingPreKey::get_preKeyId_for_reflect,
                    SessionStructure_PendingPreKey::mut_preKeyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "signedPreKeyId",
                    SessionStructure_PendingPreKey::get_signedPreKeyId_for_reflect,
                    SessionStructure_PendingPreKey::mut_signedPreKeyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "baseKey",
                    SessionStructure_PendingPreKey::get_baseKey_for_reflect,
                    SessionStructure_PendingPreKey::mut_baseKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionStructure_PendingPreKey>(
                    "SessionStructure_PendingPreKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionStructure_PendingPreKey {
    fn clear(&mut self) {
        self.clear_preKeyId();
        self.clear_signedPreKeyId();
        self.clear_baseKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionStructure_PendingPreKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionStructure_PendingPreKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RecordStructure {
    // message fields
    currentSession: ::protobuf::SingularPtrField<SessionStructure>,
    previousSessions: ::protobuf::RepeatedField<SessionStructure>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecordStructure {}

impl RecordStructure {
    pub fn new() -> RecordStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecordStructure {
        static mut instance: ::protobuf::lazy::Lazy<RecordStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecordStructure,
        };
        unsafe {
            instance.get(RecordStructure::new)
        }
    }

    // optional .textsecure.SessionStructure currentSession = 1;

    pub fn clear_currentSession(&mut self) {
        self.currentSession.clear();
    }

    pub fn has_currentSession(&self) -> bool {
        self.currentSession.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentSession(&mut self, v: SessionStructure) {
        self.currentSession = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentSession(&mut self) -> &mut SessionStructure {
        if self.currentSession.is_none() {
            self.currentSession.set_default();
        };
        self.currentSession.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentSession(&mut self) -> SessionStructure {
        self.currentSession.take().unwrap_or_else(|| SessionStructure::new())
    }

    pub fn get_currentSession(&self) -> &SessionStructure {
        self.currentSession.as_ref().unwrap_or_else(|| SessionStructure::default_instance())
    }

    fn get_currentSession_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionStructure> {
        &self.currentSession
    }

    fn mut_currentSession_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionStructure> {
        &mut self.currentSession
    }

    // repeated .textsecure.SessionStructure previousSessions = 2;

    pub fn clear_previousSessions(&mut self) {
        self.previousSessions.clear();
    }

    // Param is passed by value, moved
    pub fn set_previousSessions(&mut self, v: ::protobuf::RepeatedField<SessionStructure>) {
        self.previousSessions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_previousSessions(&mut self) -> &mut ::protobuf::RepeatedField<SessionStructure> {
        &mut self.previousSessions
    }

    // Take field
    pub fn take_previousSessions(&mut self) -> ::protobuf::RepeatedField<SessionStructure> {
        ::std::mem::replace(&mut self.previousSessions, ::protobuf::RepeatedField::new())
    }

    pub fn get_previousSessions(&self) -> &[SessionStructure] {
        &self.previousSessions
    }

    fn get_previousSessions_for_reflect(&self) -> &::protobuf::RepeatedField<SessionStructure> {
        &self.previousSessions
    }

    fn mut_previousSessions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SessionStructure> {
        &mut self.previousSessions
    }
}

impl ::protobuf::Message for RecordStructure {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentSession)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.previousSessions)?;
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
        if let Some(v) = self.currentSession.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.previousSessions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentSession.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.previousSessions {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RecordStructure {
    fn new() -> RecordStructure {
        RecordStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecordStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure>>(
                    "currentSession",
                    RecordStructure::get_currentSession_for_reflect,
                    RecordStructure::mut_currentSession_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionStructure>>(
                    "previousSessions",
                    RecordStructure::get_previousSessions_for_reflect,
                    RecordStructure::mut_previousSessions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecordStructure>(
                    "RecordStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecordStructure {
    fn clear(&mut self) {
        self.clear_currentSession();
        self.clear_previousSessions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RecordStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RecordStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PreKeyRecordStructure {
    // message fields
    id: ::std::option::Option<u32>,
    publicKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    privateKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PreKeyRecordStructure {}

impl PreKeyRecordStructure {
    pub fn new() -> PreKeyRecordStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PreKeyRecordStructure {
        static mut instance: ::protobuf::lazy::Lazy<PreKeyRecordStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PreKeyRecordStructure,
        };
        unsafe {
            instance.get(PreKeyRecordStructure::new)
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

    // optional bytes publicKey = 2;

    pub fn clear_publicKey(&mut self) {
        self.publicKey.clear();
    }

    pub fn has_publicKey(&self) -> bool {
        self.publicKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publicKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.publicKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.publicKey.is_none() {
            self.publicKey.set_default();
        };
        self.publicKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_publicKey(&mut self) -> ::std::vec::Vec<u8> {
        self.publicKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_publicKey(&self) -> &[u8] {
        match self.publicKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_publicKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.publicKey
    }

    fn mut_publicKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.publicKey
    }

    // optional bytes privateKey = 3;

    pub fn clear_privateKey(&mut self) {
        self.privateKey.clear();
    }

    pub fn has_privateKey(&self) -> bool {
        self.privateKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_privateKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.privateKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_privateKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.privateKey.is_none() {
            self.privateKey.set_default();
        };
        self.privateKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_privateKey(&mut self) -> ::std::vec::Vec<u8> {
        self.privateKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_privateKey(&self) -> &[u8] {
        match self.privateKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_privateKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.privateKey
    }

    fn mut_privateKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.privateKey
    }
}

impl ::protobuf::Message for PreKeyRecordStructure {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.publicKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.privateKey)?;
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
        if let Some(v) = self.publicKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.privateKey.as_ref() {
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
        if let Some(v) = self.publicKey.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.privateKey.as_ref() {
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

impl ::protobuf::MessageStatic for PreKeyRecordStructure {
    fn new() -> PreKeyRecordStructure {
        PreKeyRecordStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<PreKeyRecordStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    PreKeyRecordStructure::get_id_for_reflect,
                    PreKeyRecordStructure::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "publicKey",
                    PreKeyRecordStructure::get_publicKey_for_reflect,
                    PreKeyRecordStructure::mut_publicKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "privateKey",
                    PreKeyRecordStructure::get_privateKey_for_reflect,
                    PreKeyRecordStructure::mut_privateKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PreKeyRecordStructure>(
                    "PreKeyRecordStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PreKeyRecordStructure {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_publicKey();
        self.clear_privateKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PreKeyRecordStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PreKeyRecordStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedPreKeyRecordStructure {
    // message fields
    id: ::std::option::Option<u32>,
    publicKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    privateKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedPreKeyRecordStructure {}

impl SignedPreKeyRecordStructure {
    pub fn new() -> SignedPreKeyRecordStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedPreKeyRecordStructure {
        static mut instance: ::protobuf::lazy::Lazy<SignedPreKeyRecordStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedPreKeyRecordStructure,
        };
        unsafe {
            instance.get(SignedPreKeyRecordStructure::new)
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

    // optional bytes publicKey = 2;

    pub fn clear_publicKey(&mut self) {
        self.publicKey.clear();
    }

    pub fn has_publicKey(&self) -> bool {
        self.publicKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publicKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.publicKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.publicKey.is_none() {
            self.publicKey.set_default();
        };
        self.publicKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_publicKey(&mut self) -> ::std::vec::Vec<u8> {
        self.publicKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_publicKey(&self) -> &[u8] {
        match self.publicKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_publicKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.publicKey
    }

    fn mut_publicKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.publicKey
    }

    // optional bytes privateKey = 3;

    pub fn clear_privateKey(&mut self) {
        self.privateKey.clear();
    }

    pub fn has_privateKey(&self) -> bool {
        self.privateKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_privateKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.privateKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_privateKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.privateKey.is_none() {
            self.privateKey.set_default();
        };
        self.privateKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_privateKey(&mut self) -> ::std::vec::Vec<u8> {
        self.privateKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_privateKey(&self) -> &[u8] {
        match self.privateKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_privateKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.privateKey
    }

    fn mut_privateKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.privateKey
    }

    // optional bytes signature = 4;

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

    // optional fixed64 timestamp = 5;

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

impl ::protobuf::Message for SignedPreKeyRecordStructure {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.publicKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.privateKey)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.publicKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.privateKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.publicKey.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.privateKey.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.signature.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_fixed64(5, v)?;
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

impl ::protobuf::MessageStatic for SignedPreKeyRecordStructure {
    fn new() -> SignedPreKeyRecordStructure {
        SignedPreKeyRecordStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedPreKeyRecordStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    SignedPreKeyRecordStructure::get_id_for_reflect,
                    SignedPreKeyRecordStructure::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "publicKey",
                    SignedPreKeyRecordStructure::get_publicKey_for_reflect,
                    SignedPreKeyRecordStructure::mut_publicKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "privateKey",
                    SignedPreKeyRecordStructure::get_privateKey_for_reflect,
                    SignedPreKeyRecordStructure::mut_privateKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    SignedPreKeyRecordStructure::get_signature_for_reflect,
                    SignedPreKeyRecordStructure::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "timestamp",
                    SignedPreKeyRecordStructure::get_timestamp_for_reflect,
                    SignedPreKeyRecordStructure::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedPreKeyRecordStructure>(
                    "SignedPreKeyRecordStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedPreKeyRecordStructure {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_publicKey();
        self.clear_privateKey();
        self.clear_signature();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedPreKeyRecordStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedPreKeyRecordStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IdentityKeyPairStructure {
    // message fields
    publicKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    privateKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IdentityKeyPairStructure {}

impl IdentityKeyPairStructure {
    pub fn new() -> IdentityKeyPairStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IdentityKeyPairStructure {
        static mut instance: ::protobuf::lazy::Lazy<IdentityKeyPairStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IdentityKeyPairStructure,
        };
        unsafe {
            instance.get(IdentityKeyPairStructure::new)
        }
    }

    // optional bytes publicKey = 1;

    pub fn clear_publicKey(&mut self) {
        self.publicKey.clear();
    }

    pub fn has_publicKey(&self) -> bool {
        self.publicKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publicKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.publicKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.publicKey.is_none() {
            self.publicKey.set_default();
        };
        self.publicKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_publicKey(&mut self) -> ::std::vec::Vec<u8> {
        self.publicKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_publicKey(&self) -> &[u8] {
        match self.publicKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_publicKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.publicKey
    }

    fn mut_publicKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.publicKey
    }

    // optional bytes privateKey = 2;

    pub fn clear_privateKey(&mut self) {
        self.privateKey.clear();
    }

    pub fn has_privateKey(&self) -> bool {
        self.privateKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_privateKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.privateKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_privateKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.privateKey.is_none() {
            self.privateKey.set_default();
        };
        self.privateKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_privateKey(&mut self) -> ::std::vec::Vec<u8> {
        self.privateKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_privateKey(&self) -> &[u8] {
        match self.privateKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_privateKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.privateKey
    }

    fn mut_privateKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.privateKey
    }
}

impl ::protobuf::Message for IdentityKeyPairStructure {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.publicKey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.privateKey)?;
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
        if let Some(v) = self.publicKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.privateKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.publicKey.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.privateKey.as_ref() {
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

impl ::protobuf::MessageStatic for IdentityKeyPairStructure {
    fn new() -> IdentityKeyPairStructure {
        IdentityKeyPairStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<IdentityKeyPairStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "publicKey",
                    IdentityKeyPairStructure::get_publicKey_for_reflect,
                    IdentityKeyPairStructure::mut_publicKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "privateKey",
                    IdentityKeyPairStructure::get_privateKey_for_reflect,
                    IdentityKeyPairStructure::mut_privateKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IdentityKeyPairStructure>(
                    "IdentityKeyPairStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IdentityKeyPairStructure {
    fn clear(&mut self) {
        self.clear_publicKey();
        self.clear_privateKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IdentityKeyPairStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IdentityKeyPairStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyStateStructure {
    // message fields
    senderKeyId: ::std::option::Option<u32>,
    senderChainKey: ::protobuf::SingularPtrField<SenderKeyStateStructure_SenderChainKey>,
    senderSigningKey: ::protobuf::SingularPtrField<SenderKeyStateStructure_SenderSigningKey>,
    senderMessageKeys: ::protobuf::RepeatedField<SenderKeyStateStructure_SenderMessageKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyStateStructure {}

impl SenderKeyStateStructure {
    pub fn new() -> SenderKeyStateStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyStateStructure {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyStateStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyStateStructure,
        };
        unsafe {
            instance.get(SenderKeyStateStructure::new)
        }
    }

    // optional uint32 senderKeyId = 1;

    pub fn clear_senderKeyId(&mut self) {
        self.senderKeyId = ::std::option::Option::None;
    }

    pub fn has_senderKeyId(&self) -> bool {
        self.senderKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderKeyId(&mut self, v: u32) {
        self.senderKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_senderKeyId(&self) -> u32 {
        self.senderKeyId.unwrap_or(0)
    }

    fn get_senderKeyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.senderKeyId
    }

    fn mut_senderKeyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.senderKeyId
    }

    // optional .textsecure.SenderKeyStateStructure.SenderChainKey senderChainKey = 2;

    pub fn clear_senderChainKey(&mut self) {
        self.senderChainKey.clear();
    }

    pub fn has_senderChainKey(&self) -> bool {
        self.senderChainKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderChainKey(&mut self, v: SenderKeyStateStructure_SenderChainKey) {
        self.senderChainKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_senderChainKey(&mut self) -> &mut SenderKeyStateStructure_SenderChainKey {
        if self.senderChainKey.is_none() {
            self.senderChainKey.set_default();
        };
        self.senderChainKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_senderChainKey(&mut self) -> SenderKeyStateStructure_SenderChainKey {
        self.senderChainKey.take().unwrap_or_else(|| SenderKeyStateStructure_SenderChainKey::new())
    }

    pub fn get_senderChainKey(&self) -> &SenderKeyStateStructure_SenderChainKey {
        self.senderChainKey.as_ref().unwrap_or_else(|| SenderKeyStateStructure_SenderChainKey::default_instance())
    }

    fn get_senderChainKey_for_reflect(&self) -> &::protobuf::SingularPtrField<SenderKeyStateStructure_SenderChainKey> {
        &self.senderChainKey
    }

    fn mut_senderChainKey_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SenderKeyStateStructure_SenderChainKey> {
        &mut self.senderChainKey
    }

    // optional .textsecure.SenderKeyStateStructure.SenderSigningKey senderSigningKey = 3;

    pub fn clear_senderSigningKey(&mut self) {
        self.senderSigningKey.clear();
    }

    pub fn has_senderSigningKey(&self) -> bool {
        self.senderSigningKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderSigningKey(&mut self, v: SenderKeyStateStructure_SenderSigningKey) {
        self.senderSigningKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_senderSigningKey(&mut self) -> &mut SenderKeyStateStructure_SenderSigningKey {
        if self.senderSigningKey.is_none() {
            self.senderSigningKey.set_default();
        };
        self.senderSigningKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_senderSigningKey(&mut self) -> SenderKeyStateStructure_SenderSigningKey {
        self.senderSigningKey.take().unwrap_or_else(|| SenderKeyStateStructure_SenderSigningKey::new())
    }

    pub fn get_senderSigningKey(&self) -> &SenderKeyStateStructure_SenderSigningKey {
        self.senderSigningKey.as_ref().unwrap_or_else(|| SenderKeyStateStructure_SenderSigningKey::default_instance())
    }

    fn get_senderSigningKey_for_reflect(&self) -> &::protobuf::SingularPtrField<SenderKeyStateStructure_SenderSigningKey> {
        &self.senderSigningKey
    }

    fn mut_senderSigningKey_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SenderKeyStateStructure_SenderSigningKey> {
        &mut self.senderSigningKey
    }

    // repeated .textsecure.SenderKeyStateStructure.SenderMessageKey senderMessageKeys = 4;

    pub fn clear_senderMessageKeys(&mut self) {
        self.senderMessageKeys.clear();
    }

    // Param is passed by value, moved
    pub fn set_senderMessageKeys(&mut self, v: ::protobuf::RepeatedField<SenderKeyStateStructure_SenderMessageKey>) {
        self.senderMessageKeys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_senderMessageKeys(&mut self) -> &mut ::protobuf::RepeatedField<SenderKeyStateStructure_SenderMessageKey> {
        &mut self.senderMessageKeys
    }

    // Take field
    pub fn take_senderMessageKeys(&mut self) -> ::protobuf::RepeatedField<SenderKeyStateStructure_SenderMessageKey> {
        ::std::mem::replace(&mut self.senderMessageKeys, ::protobuf::RepeatedField::new())
    }

    pub fn get_senderMessageKeys(&self) -> &[SenderKeyStateStructure_SenderMessageKey] {
        &self.senderMessageKeys
    }

    fn get_senderMessageKeys_for_reflect(&self) -> &::protobuf::RepeatedField<SenderKeyStateStructure_SenderMessageKey> {
        &self.senderMessageKeys
    }

    fn mut_senderMessageKeys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SenderKeyStateStructure_SenderMessageKey> {
        &mut self.senderMessageKeys
    }
}

impl ::protobuf::Message for SenderKeyStateStructure {
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
                    self.senderKeyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.senderChainKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.senderSigningKey)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.senderMessageKeys)?;
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
        if let Some(v) = self.senderKeyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.senderChainKey.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.senderSigningKey.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.senderMessageKeys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.senderKeyId {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.senderChainKey.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.senderSigningKey.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.senderMessageKeys {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SenderKeyStateStructure {
    fn new() -> SenderKeyStateStructure {
        SenderKeyStateStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyStateStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "senderKeyId",
                    SenderKeyStateStructure::get_senderKeyId_for_reflect,
                    SenderKeyStateStructure::mut_senderKeyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SenderKeyStateStructure_SenderChainKey>>(
                    "senderChainKey",
                    SenderKeyStateStructure::get_senderChainKey_for_reflect,
                    SenderKeyStateStructure::mut_senderChainKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SenderKeyStateStructure_SenderSigningKey>>(
                    "senderSigningKey",
                    SenderKeyStateStructure::get_senderSigningKey_for_reflect,
                    SenderKeyStateStructure::mut_senderSigningKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SenderKeyStateStructure_SenderMessageKey>>(
                    "senderMessageKeys",
                    SenderKeyStateStructure::get_senderMessageKeys_for_reflect,
                    SenderKeyStateStructure::mut_senderMessageKeys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyStateStructure>(
                    "SenderKeyStateStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyStateStructure {
    fn clear(&mut self) {
        self.clear_senderKeyId();
        self.clear_senderChainKey();
        self.clear_senderSigningKey();
        self.clear_senderMessageKeys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyStateStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyStateStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyStateStructure_SenderChainKey {
    // message fields
    iteration: ::std::option::Option<u32>,
    seed: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyStateStructure_SenderChainKey {}

impl SenderKeyStateStructure_SenderChainKey {
    pub fn new() -> SenderKeyStateStructure_SenderChainKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyStateStructure_SenderChainKey {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyStateStructure_SenderChainKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyStateStructure_SenderChainKey,
        };
        unsafe {
            instance.get(SenderKeyStateStructure_SenderChainKey::new)
        }
    }

    // optional uint32 iteration = 1;

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

    // optional bytes seed = 2;

    pub fn clear_seed(&mut self) {
        self.seed.clear();
    }

    pub fn has_seed(&self) -> bool {
        self.seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: ::std::vec::Vec<u8>) {
        self.seed = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_seed(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.seed.is_none() {
            self.seed.set_default();
        };
        self.seed.as_mut().unwrap()
    }

    // Take field
    pub fn take_seed(&mut self) -> ::std::vec::Vec<u8> {
        self.seed.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_seed(&self) -> &[u8] {
        match self.seed.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_seed_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.seed
    }

    fn mut_seed_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.seed
    }
}

impl ::protobuf::Message for SenderKeyStateStructure_SenderChainKey {
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
                    self.iteration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.seed)?;
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
        if let Some(v) = self.iteration {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seed.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.iteration {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.seed.as_ref() {
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

impl ::protobuf::MessageStatic for SenderKeyStateStructure_SenderChainKey {
    fn new() -> SenderKeyStateStructure_SenderChainKey {
        SenderKeyStateStructure_SenderChainKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyStateStructure_SenderChainKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "iteration",
                    SenderKeyStateStructure_SenderChainKey::get_iteration_for_reflect,
                    SenderKeyStateStructure_SenderChainKey::mut_iteration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "seed",
                    SenderKeyStateStructure_SenderChainKey::get_seed_for_reflect,
                    SenderKeyStateStructure_SenderChainKey::mut_seed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyStateStructure_SenderChainKey>(
                    "SenderKeyStateStructure_SenderChainKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyStateStructure_SenderChainKey {
    fn clear(&mut self) {
        self.clear_iteration();
        self.clear_seed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyStateStructure_SenderChainKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyStateStructure_SenderChainKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyStateStructure_SenderMessageKey {
    // message fields
    iteration: ::std::option::Option<u32>,
    seed: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyStateStructure_SenderMessageKey {}

impl SenderKeyStateStructure_SenderMessageKey {
    pub fn new() -> SenderKeyStateStructure_SenderMessageKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyStateStructure_SenderMessageKey {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyStateStructure_SenderMessageKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyStateStructure_SenderMessageKey,
        };
        unsafe {
            instance.get(SenderKeyStateStructure_SenderMessageKey::new)
        }
    }

    // optional uint32 iteration = 1;

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

    // optional bytes seed = 2;

    pub fn clear_seed(&mut self) {
        self.seed.clear();
    }

    pub fn has_seed(&self) -> bool {
        self.seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: ::std::vec::Vec<u8>) {
        self.seed = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_seed(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.seed.is_none() {
            self.seed.set_default();
        };
        self.seed.as_mut().unwrap()
    }

    // Take field
    pub fn take_seed(&mut self) -> ::std::vec::Vec<u8> {
        self.seed.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_seed(&self) -> &[u8] {
        match self.seed.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_seed_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.seed
    }

    fn mut_seed_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.seed
    }
}

impl ::protobuf::Message for SenderKeyStateStructure_SenderMessageKey {
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
                    self.iteration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.seed)?;
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
        if let Some(v) = self.iteration {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seed.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.iteration {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.seed.as_ref() {
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

impl ::protobuf::MessageStatic for SenderKeyStateStructure_SenderMessageKey {
    fn new() -> SenderKeyStateStructure_SenderMessageKey {
        SenderKeyStateStructure_SenderMessageKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyStateStructure_SenderMessageKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "iteration",
                    SenderKeyStateStructure_SenderMessageKey::get_iteration_for_reflect,
                    SenderKeyStateStructure_SenderMessageKey::mut_iteration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "seed",
                    SenderKeyStateStructure_SenderMessageKey::get_seed_for_reflect,
                    SenderKeyStateStructure_SenderMessageKey::mut_seed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyStateStructure_SenderMessageKey>(
                    "SenderKeyStateStructure_SenderMessageKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyStateStructure_SenderMessageKey {
    fn clear(&mut self) {
        self.clear_iteration();
        self.clear_seed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyStateStructure_SenderMessageKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyStateStructure_SenderMessageKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyStateStructure_SenderSigningKey {
    // message fields
    public: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    private: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyStateStructure_SenderSigningKey {}

impl SenderKeyStateStructure_SenderSigningKey {
    pub fn new() -> SenderKeyStateStructure_SenderSigningKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyStateStructure_SenderSigningKey {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyStateStructure_SenderSigningKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyStateStructure_SenderSigningKey,
        };
        unsafe {
            instance.get(SenderKeyStateStructure_SenderSigningKey::new)
        }
    }

    // optional bytes public = 1;

    pub fn clear_public(&mut self) {
        self.public.clear();
    }

    pub fn has_public(&self) -> bool {
        self.public.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public(&mut self, v: ::std::vec::Vec<u8>) {
        self.public = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.public.is_none() {
            self.public.set_default();
        };
        self.public.as_mut().unwrap()
    }

    // Take field
    pub fn take_public(&mut self) -> ::std::vec::Vec<u8> {
        self.public.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public(&self) -> &[u8] {
        match self.public.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_public_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.public
    }

    fn mut_public_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.public
    }

    // optional bytes private = 2;

    pub fn clear_private(&mut self) {
        self.private.clear();
    }

    pub fn has_private(&self) -> bool {
        self.private.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private(&mut self, v: ::std::vec::Vec<u8>) {
        self.private = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.private.is_none() {
            self.private.set_default();
        };
        self.private.as_mut().unwrap()
    }

    // Take field
    pub fn take_private(&mut self) -> ::std::vec::Vec<u8> {
        self.private.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_private(&self) -> &[u8] {
        match self.private.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_private_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.private
    }

    fn mut_private_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.private
    }
}

impl ::protobuf::Message for SenderKeyStateStructure_SenderSigningKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.public)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.private)?;
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
        if let Some(v) = self.public.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.private.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.private.as_ref() {
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

impl ::protobuf::MessageStatic for SenderKeyStateStructure_SenderSigningKey {
    fn new() -> SenderKeyStateStructure_SenderSigningKey {
        SenderKeyStateStructure_SenderSigningKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyStateStructure_SenderSigningKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public",
                    SenderKeyStateStructure_SenderSigningKey::get_public_for_reflect,
                    SenderKeyStateStructure_SenderSigningKey::mut_public_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "private",
                    SenderKeyStateStructure_SenderSigningKey::get_private_for_reflect,
                    SenderKeyStateStructure_SenderSigningKey::mut_private_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyStateStructure_SenderSigningKey>(
                    "SenderKeyStateStructure_SenderSigningKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyStateStructure_SenderSigningKey {
    fn clear(&mut self) {
        self.clear_public();
        self.clear_private();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyStateStructure_SenderSigningKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyStateStructure_SenderSigningKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SenderKeyRecordStructure {
    // message fields
    senderKeyStates: ::protobuf::RepeatedField<SenderKeyStateStructure>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SenderKeyRecordStructure {}

impl SenderKeyRecordStructure {
    pub fn new() -> SenderKeyRecordStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SenderKeyRecordStructure {
        static mut instance: ::protobuf::lazy::Lazy<SenderKeyRecordStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SenderKeyRecordStructure,
        };
        unsafe {
            instance.get(SenderKeyRecordStructure::new)
        }
    }

    // repeated .textsecure.SenderKeyStateStructure senderKeyStates = 1;

    pub fn clear_senderKeyStates(&mut self) {
        self.senderKeyStates.clear();
    }

    // Param is passed by value, moved
    pub fn set_senderKeyStates(&mut self, v: ::protobuf::RepeatedField<SenderKeyStateStructure>) {
        self.senderKeyStates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_senderKeyStates(&mut self) -> &mut ::protobuf::RepeatedField<SenderKeyStateStructure> {
        &mut self.senderKeyStates
    }

    // Take field
    pub fn take_senderKeyStates(&mut self) -> ::protobuf::RepeatedField<SenderKeyStateStructure> {
        ::std::mem::replace(&mut self.senderKeyStates, ::protobuf::RepeatedField::new())
    }

    pub fn get_senderKeyStates(&self) -> &[SenderKeyStateStructure] {
        &self.senderKeyStates
    }

    fn get_senderKeyStates_for_reflect(&self) -> &::protobuf::RepeatedField<SenderKeyStateStructure> {
        &self.senderKeyStates
    }

    fn mut_senderKeyStates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SenderKeyStateStructure> {
        &mut self.senderKeyStates
    }
}

impl ::protobuf::Message for SenderKeyRecordStructure {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.senderKeyStates)?;
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
        for value in &self.senderKeyStates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.senderKeyStates {
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

impl ::protobuf::MessageStatic for SenderKeyRecordStructure {
    fn new() -> SenderKeyRecordStructure {
        SenderKeyRecordStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<SenderKeyRecordStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SenderKeyStateStructure>>(
                    "senderKeyStates",
                    SenderKeyRecordStructure::get_senderKeyStates_for_reflect,
                    SenderKeyRecordStructure::mut_senderKeyStates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SenderKeyRecordStructure>(
                    "SenderKeyRecordStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SenderKeyRecordStructure {
    fn clear(&mut self) {
        self.clear_senderKeyStates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderKeyRecordStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderKeyRecordStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x4c, 0x6f, 0x63, 0x61, 0x6c,
    0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63, 0x75, 0x72,
    0x65, 0x22, 0xbf, 0x0c, 0x0a, 0x10, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x26, 0x0a, 0x0e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e,
    0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x30,
    0x0a, 0x13, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x50,
    0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x13, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63,
    0x12, 0x32, 0x0a, 0x14, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x14,
    0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x12, 0x18, 0x0a, 0x07, 0x72, 0x6f, 0x6f, 0x74, 0x4b, 0x65, 0x79, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x72, 0x6f, 0x6f, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x28,
    0x0a, 0x0f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75,
    0x73, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x44, 0x0a, 0x0b, 0x73, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e,
    0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x52, 0x0b, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x12, 0x4a,
    0x0a, 0x0e, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x73,
    0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63,
    0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x52, 0x0e, 0x72, 0x65, 0x63, 0x65,
    0x69, 0x76, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x5f, 0x0a, 0x12, 0x70, 0x65,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63,
    0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x2e, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x45,
    0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x12, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x4b, 0x65, 0x79, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x50, 0x0a, 0x0d, 0x70,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x2e,
    0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65,
    0x2e, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x52, 0x0d,
    0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x32, 0x0a,
    0x14, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x72, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49,
    0x64, 0x12, 0x30, 0x0a, 0x13, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x64, 0x12, 0x22, 0x0a, 0x0c, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x52, 0x65, 0x66, 0x72,
    0x65, 0x73, 0x68, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x6e, 0x65, 0x65, 0x64, 0x73,
    0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x12, 0x22, 0x0a, 0x0c, 0x61, 0x6c, 0x69, 0x63, 0x65,
    0x42, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x61,
    0x6c, 0x69, 0x63, 0x65, 0x42, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x1a, 0xa5, 0x03, 0x0a, 0x05,
    0x43, 0x68, 0x61, 0x69, 0x6e, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x52,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x10, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x52, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65,
    0x79, 0x12, 0x38, 0x0a, 0x17, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x52, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x17, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x52, 0x61, 0x74, 0x63, 0x68, 0x65,
    0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x12, 0x47, 0x0a, 0x08, 0x63,
    0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e,
    0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x52, 0x08, 0x63, 0x68, 0x61, 0x69,
    0x6e, 0x4b, 0x65, 0x79, 0x12, 0x4f, 0x0a, 0x0b, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4b,
    0x65, 0x79, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x74, 0x65, 0x78, 0x74,
    0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x52, 0x0b, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x4b, 0x65, 0x79, 0x73, 0x1a, 0x32, 0x0a, 0x08, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65,
    0x79, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x1a, 0x68, 0x0a, 0x0a, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1c, 0x0a,
    0x09, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x09, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x6d,
    0x61, 0x63, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x6d, 0x61, 0x63,
    0x4b, 0x65, 0x79, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x76, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x02, 0x69, 0x76, 0x1a, 0xce, 0x02, 0x0a, 0x12, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4b,
    0x65, 0x79, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x73, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x22, 0x0a, 0x0c, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x42,
    0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x6c, 0x6f,
    0x63, 0x61, 0x6c, 0x42, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x30, 0x0a, 0x13, 0x6c, 0x6f,
    0x63, 0x61, 0x6c, 0x42, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x13, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x42, 0x61,
    0x73, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x12, 0x28, 0x0a, 0x0f,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61, 0x74, 0x63,
    0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x36, 0x0a, 0x16, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x52,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x16, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x12, 0x2a,
    0x0a, 0x10, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4b,
    0x65, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x10, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4b, 0x65, 0x79, 0x12, 0x38, 0x0a, 0x17, 0x6c, 0x6f,
    0x63, 0x61, 0x6c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4b, 0x65, 0x79, 0x50, 0x72,
    0x69, 0x76, 0x61, 0x74, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x17, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x69,
    0x76, 0x61, 0x74, 0x65, 0x1a, 0x6d, 0x0a, 0x0d, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x50,
    0x72, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x49,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x70, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x49,
    0x64, 0x12, 0x26, 0x0a, 0x0e, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x72, 0x65, 0x4b, 0x65,
    0x79, 0x49, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x73, 0x69, 0x67, 0x6e, 0x65,
    0x64, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x62, 0x61, 0x73,
    0x65, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x62, 0x61, 0x73, 0x65,
    0x4b, 0x65, 0x79, 0x22, 0xa1, 0x01, 0x0a, 0x0f, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x53, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x44, 0x0a, 0x0e, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x52, 0x0e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x48, 0x0a,
    0x10, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65,
    0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x75, 0x72, 0x65, 0x52, 0x10, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x53,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x65, 0x0a, 0x15, 0x50, 0x72, 0x65, 0x4b, 0x65,
    0x79, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65,
    0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64,
    0x12, 0x1c, 0x0a, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x1e,
    0x0a, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x22, 0xa7,
    0x01, 0x0a, 0x1b, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x52,
    0x65, 0x63, 0x6f, 0x72, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x0e,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64, 0x12, 0x1c,
    0x0a, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x1e, 0x0a, 0x0a,
    0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x1c, 0x0a, 0x09,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x58, 0x0a, 0x18, 0x49, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x69, 0x72, 0x53, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b,
    0x65, 0x79, 0x12, 0x1e, 0x0a, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b,
    0x65, 0x79, 0x22, 0xad, 0x04, 0x0a, 0x17, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x20,
    0x0a, 0x0b, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x49, 0x64,
    0x12, 0x5a, 0x0a, 0x0e, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4b,
    0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73,
    0x65, 0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x52, 0x0e, 0x73, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x12, 0x60, 0x0a, 0x10,
    0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65, 0x63,
    0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x52, 0x10, 0x73, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x12, 0x62,
    0x0a, 0x11, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4b,
    0x65, 0x79, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x74, 0x65, 0x78, 0x74,
    0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x53,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x52,
    0x11, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4b, 0x65,
    0x79, 0x73, 0x1a, 0x42, 0x0a, 0x0e, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x4b, 0x65, 0x79, 0x12, 0x1c, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x65, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x04, 0x73, 0x65, 0x65, 0x64, 0x1a, 0x44, 0x0a, 0x10, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x1c, 0x0a, 0x09, 0x69, 0x74,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x69,
    0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x65, 0x65, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x73, 0x65, 0x65, 0x64, 0x1a, 0x44, 0x0a, 0x10,
    0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79,
    0x12, 0x16, 0x0a, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x76,
    0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x72, 0x69, 0x76, 0x61,
    0x74, 0x65, 0x22, 0x69, 0x0a, 0x18, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x52,
    0x65, 0x63, 0x6f, 0x72, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x4d,
    0x0a, 0x0f, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x65,
    0x63, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x52, 0x0f, 0x73, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x53, 0x74, 0x61, 0x74, 0x65, 0x73, 0x42, 0x33, 0x0a,
    0x22, 0x6f, 0x72, 0x67, 0x2e, 0x77, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x73, 0x79, 0x73, 0x74,
    0x65, 0x6d, 0x73, 0x2e, 0x6c, 0x69, 0x62, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x42, 0x0d, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x4a, 0xa8, 0x25, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x71, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03,
    0x04, 0x16, 0x3a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0b, 0x0a,
    0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x12, 0x03, 0x05, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x05, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07,
    0x12, 0x03, 0x05, 0x1e, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x3f,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x18, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x08, 0x04, 0x1b, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x08, 0x0c, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x17, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0a, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0a, 0x17, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0a, 0x31, 0x32, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x03,
    0x00, 0x12, 0x04, 0x0c, 0x08, 0x0f, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x03,
    0x00, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x0c, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x0c, 0x14, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x15, 0x1b, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x1c, 0x21, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x24, 0x25,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x0c,
    0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0e, 0x0c, 0x14, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0e, 0x15, 0x1a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0e, 0x1c, 0x1f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x11, 0x08, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x11, 0x11, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x1a, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00,
    0x03, 0x01, 0x12, 0x04, 0x13, 0x08, 0x18, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x03, 0x01, 0x01, 0x12, 0x03, 0x13, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x14, 0x0c, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03,
    0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x0c, 0x14, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x15, 0x1b, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x1c, 0x21, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x28,
    0x29, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x15,
    0x0c, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x15, 0x0c, 0x14, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x15, 0x15, 0x1a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x1c, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x28, 0x29, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00,
    0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03, 0x16, 0x0c, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x0c, 0x14, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x16, 0x15, 0x1a, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x1c,
    0x22, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x16, 0x28, 0x29, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x03, 0x12,
    0x03, 0x17, 0x0c, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x17, 0x0c, 0x14, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x03, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x17, 0x15, 0x1a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17, 0x1c, 0x1e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x03, 0x00, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x08, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1a, 0x11, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x1c, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x03, 0x01, 0x12, 0x04, 0x1d, 0x04, 0x25, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03,
    0x01, 0x01, 0x12, 0x03, 0x1d, 0x0c, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x1e, 0x08, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x1e, 0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1e, 0x18, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1e, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x1f, 0x08, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x1f, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1f, 0x18, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1f, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x12,
    0x03, 0x20, 0x08, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x20, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x20, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x20, 0x18, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x20, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03, 0x12, 0x03,
    0x21, 0x08, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x21, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x21, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x21, 0x18, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x21, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x12, 0x03, 0x22,
    0x08, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x22,
    0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x22,
    0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x22,
    0x18, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x22,
    0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x12, 0x03, 0x23, 0x08,
    0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x23, 0x08,
    0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x23, 0x11,
    0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x23, 0x18,
    0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x23, 0x33,
    0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x12, 0x03, 0x24, 0x08, 0x35,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x24, 0x11, 0x16,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x24, 0x18, 0x2f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x24, 0x33, 0x34,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x02, 0x12, 0x04, 0x27, 0x04, 0x2b, 0x05, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x27, 0x0c, 0x19, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x28, 0x08, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x18, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x08, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x18, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x08, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2a, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x2d, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x14, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2e, 0x13, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2e, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x04, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x13, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x31, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x31, 0x13, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x31, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x32, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x32, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x32, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x32,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x34, 0x04, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x34, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x34, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x34, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x35, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x35,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x35, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x35, 0x13, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x35, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x37, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x37, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x37, 0x20, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x37, 0x35,
    0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x38, 0x04, 0x37, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x38, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x38, 0x20, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08,
    0x03, 0x12, 0x03, 0x38, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03,
    0x3a, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x3a, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x3a, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x3a, 0x14, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x3a, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x0a, 0x12, 0x03, 0x3b, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12,
    0x03, 0x3b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x3b,
    0x14, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x3b, 0x2b, 0x2d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x3d, 0x04, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03,
    0x12, 0x03, 0x3d, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x3e,
    0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x3e, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x3e, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x3e, 0x22, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x41, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x41,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x42, 0x04, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x42, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x1e, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x42, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x43, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x43,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x43, 0x0d, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x1e, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x46, 0x00, 0x4a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x46, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x47, 0x04,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x48, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x14, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x49, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x49, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x49, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x49, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x4c, 0x00, 0x52, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x4d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d,
    0x15, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x04, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x4e, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x4e, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x4f,
    0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4f, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x15, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x03, 0x12, 0x03, 0x50, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x50, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x50, 0x15,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x50, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x51, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x51, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x51, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x51, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x51, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x54, 0x00, 0x57, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x54, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x55, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x55, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x55, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x56, 0x04, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x56, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x56, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x59, 0x00,
    0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x59, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x05, 0x03, 0x00, 0x12, 0x04, 0x5a, 0x04, 0x5d, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x03, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x0c, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x08, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5b, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5b, 0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x18, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x5c, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x5c, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x01,
    0x12, 0x04, 0x5f, 0x04, 0x62, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x01, 0x01, 0x12,
    0x03, 0x5f, 0x0c, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x60, 0x08, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x60, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x60, 0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x60, 0x18, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x60, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x61,
    0x08, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x61,
    0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x61,
    0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x61,
    0x18, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x61,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x02, 0x12, 0x04, 0x64, 0x04, 0x67, 0x05,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x64, 0x0c, 0x1c, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x05, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x65, 0x08, 0x23, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x05, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x05, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x65, 0x11, 0x16, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x05, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x17, 0x1d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x05, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x65, 0x21, 0x22, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x05, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x66, 0x08, 0x23, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x66, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x66, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x66, 0x17, 0x1e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x66, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x69, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x69, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x69, 0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x32,
    0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x04, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6a, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x6a, 0x1e, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x6a, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03,
    0x6b, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6b, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x6b, 0x0d, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x1e, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x6c, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6c,
    0x1e, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6c, 0x32, 0x33,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x6f, 0x00, 0x71, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x06, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00,
    0x12, 0x03, 0x70, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x70, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x70, 0x0d,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x25, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x37, 0x38,
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
