#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Property
 */

pub type PropertyType = ic4_sys::IC4_PROPERTY_TYPE;

pub type PropertyVisibility = ic4_sys::IC4_PROPERTY_VISIBILITY;

pub type PropertyIncrementMode = ic4_sys::IC4_PROPERTY_INCREMENT_MODE;

pub type PropertyOri = ic4_sys::IC4_PROPERTY;
bind_ptr_type!(
    Property,
    PropertyOri,
    ic4_sys::ic4_prop_ref,
    ic4_sys::ic4_prop_unref
);

pub type PropertyListOri = ic4_sys::IC4_PROPERTY_LIST;
bind_ptr_type!(
    PropertyList,
    PropertyListOri,
    ic4_sys::ic4_proplist_ref,
    ic4_sys::ic4_proplist_unref
);

pub type PropertyMapOri = ic4_sys::IC4_PROPERTY_MAP;
bind_ptr_type!(
    PropertyMap,
    PropertyMapOri,
    ic4_sys::ic4_propmap_ref,
    ic4_sys::ic4_propmap_unref
);

impl PropertyMap {
    pub fn execute_command(&mut self, prop_name: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_execute_command(
                self.as_mut_ptr(),
                prop_name.as_ptr() as *const std::os::raw::c_char,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl PropertyMap {
    pub fn get_value_i64(&mut self, prop_name: &CStr) -> self::Result<i64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_propmap_get_value_int64(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut value),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn get_value_f64(&mut self, prop_name: &CStr) -> self::Result<f64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_propmap_get_value_double(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut value),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn get_value_bool(&mut self, prop_name: &CStr) -> self::Result<bool> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_propmap_get_value_bool(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut value),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn get_value_cstring(&mut self, prop_name: &CStr) -> self::Result<CString> {
        let mut message_buffer = vec![0u8; 1024 * 1024];
        let mut message_length = 0;
        unsafe {
            ic4_sys::ic4_propmap_get_value_string(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut message_buffer) as *mut std::ffi::c_char,
                ptr_from_mut(&mut message_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            Ok(CString::from_vec_unchecked(message_buffer))
        }
    }
}

impl PropertyMap {
    pub fn set_value_i64(&mut self, prop_name: &CStr, value: i64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_int64(self.as_mut_ptr(), prop_name.as_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn set_value_f64(&mut self, prop_name: &CStr, value: f64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_double(self.as_mut_ptr(), prop_name.as_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn set_value_bool(&mut self, prop_name: &CStr, value: bool) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_bool(self.as_mut_ptr(), prop_name.as_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn set_value_cstr(&mut self, prop_name: &CStr, value: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_string(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                value.as_ptr(),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl PropertyMap {
    pub fn find(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_command(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_command(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_float(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_float(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_boolean(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_boolean(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_string(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_string(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_enumeration(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_enumeration(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_register(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_register(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn find_category(&mut self, prop_name: &CStr) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_category(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
}

impl PropertyMap {
    pub fn get_all(&mut self) -> Result<PropertyList> {
        let mut property_list = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_get_all(self.as_mut_ptr(), ptr_from_mut(&mut property_list))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_list.into())
    }
}

impl PropertyMap {
    pub fn connect_chunkdata(&mut self, image_buffer: &mut ImageBuffer) -> Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_connect_chunkdata(self.as_mut_ptr(), image_buffer.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn serialize_to_file(&mut self, file_path: &CStr) -> Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_serialize_to_file(self.as_mut_ptr(), file_path.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    /*
     * ic4_sys::ic4_propmap_serialize_to_fileW not useful.
     */
    /// 修改自身
    pub fn deserialize_from_file(&mut self, file_path: &CStr) -> Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_deserialize_from_file(self.as_mut_ptr(), file_path.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    /*
     * ic4_sys::ic4_propmap_deserialize_from_fileW not useful.
     */
}

pub type SerlalizationAllocatorOri = ic4_sys::ic4_serialization_allocator;
bind_type!(SerlalizationAllocator, SerlalizationAllocatorOri);

impl PropertyMap {
    pub fn serialize_to_memory(&mut self, alloc: SerlalizationAllocator) -> Result<Vec<u8>> {
        let mut data = null_mut();
        let mut data_size = 0;
        unsafe {
            ic4_sys::ic4_propmap_serialize_to_memory(
                self.as_mut_ptr(),
                alloc.inner,
                ptr_from_mut(&mut data),
                ptr_from_mut(&mut data_size),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            Ok(Vec::from_raw_parts(data as *mut u8, data_size, data_size))
        }
    }
    pub fn deserialize_from_memory(&mut self, data: &mut [u8]) -> Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_deserialize_from_memory(
                self.as_mut_ptr(),
                data.as_mut_ptr() as *const c_void,
                data.len(),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl Property {
    pub fn get_type(&mut self) -> PropertyType {
        unsafe { ic4_sys::ic4_prop_get_type(self.as_mut_ptr()) }
    }
    pub fn get_name(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_name(self.as_mut_ptr())) }
    }
    pub fn is_available(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_available(self.as_mut_ptr()) }
    }
    pub fn is_locked(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_locked(self.as_mut_ptr()) }
    }
    pub fn is_likely_locked_by_stream(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_likely_locked_by_stream(self.as_mut_ptr()) }
    }
    pub fn is_readonly(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_readonly(self.as_mut_ptr()) }
    }
    pub fn get_visibility(&mut self) -> PropertyVisibility {
        unsafe { ic4_sys::ic4_prop_get_visibility(self.as_mut_ptr()) }
    }
    pub fn get_display_name(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_display_name(self.as_mut_ptr())) }
    }
    pub fn get_tooltip(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_tooltip(self.as_mut_ptr())) }
    }
    pub fn get_description(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_description(self.as_mut_ptr())) }
    }
}

pub type PropertyNotificationOri = ic4_sys::ic4_prop_notification;
bind_type!(PropertyNotification, PropertyNotificationOri);
pub type PropertyNotificationDeleterOri = ic4_sys::ic4_prop_notification_deleter;
bind_type!(PropertyNotificationDeleter, PropertyNotificationDeleterOri);

impl Property {
    /// # Safety
    ///
    /// see ic4_sys::ic4_prop_event_add_notification
    pub unsafe fn event_add_notification(
        &mut self,
        handler: PropertyNotification,
        user_ptr: *mut c_void,
        deleter: PropertyNotificationDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_prop_event_add_notification(
            self.as_mut_ptr(),
            handler.inner,
            user_ptr,
            deleter.inner,
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())?;
        Ok(())
    }

    /// # Safety
    ///
    /// see ic4_sys::ic4_prop_event_remove_notification
    pub unsafe fn event_remove_notification(
        &mut self,
        handler: PropertyNotification,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_prop_event_remove_notification(self.as_mut_ptr(), handler.inner, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        Ok(())
    }
}

impl Property {
    pub fn is_selector(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_selector(self.as_mut_ptr()) }
    }
    pub fn get_selected_props(&mut self) -> Result<PropertyList> {
        let mut data = null_mut();
        unsafe {
            ic4_sys::ic4_prop_get_selected_props(self.as_mut_ptr(), ptr_from_mut(&mut data))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
            Ok(data.into())
        }
    }
    pub fn category_get_features(&mut self) -> Result<PropertyList> {
        let mut data = null_mut();
        unsafe {
            ic4_sys::ic4_prop_category_get_features(self.as_mut_ptr(), ptr_from_mut(&mut data))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
            Ok(data.into())
        }
    }
    pub fn command_execute(&mut self) -> Result<()> {
        unsafe {
            ic4_sys::ic4_prop_command_execute(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn command_is_done(&mut self) -> Result<bool> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_command_is_done(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
}

pub type PropertyIntRepresentationOri = ic4_sys::IC4_PROPERTY_INT_REPRESENTATION;
bind_type!(PropertyIntRepresentation, PropertyIntRepresentationOri);

impl Property {
    pub fn integer_get_unit(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_integer_get_unit(self.as_mut_ptr())) }
    }
    pub fn integer_get_value(&mut self) -> self::Result<i64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn integer_set_value(&mut self, value: i64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_integer_set_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn integer_get_min(&mut self) -> self::Result<i64> {
        let mut min = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_min(self.as_mut_ptr(), ptr_from_mut(&mut min))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(min)
    }
    pub fn integer_get_max(&mut self) -> self::Result<i64> {
        let mut max = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_max(self.as_mut_ptr(), ptr_from_mut(&mut max))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(max)
    }
    pub fn integer_get_increment(&mut self) -> self::Result<i64> {
        let mut increment = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_inc(self.as_mut_ptr(), ptr_from_mut(&mut increment))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(increment)
    }
    pub fn integer_get_increment_mode(&mut self) -> PropertyIncrementMode {
        unsafe { ic4_sys::ic4_prop_integer_get_inc_mode(self.as_mut_ptr()) }
    }
    pub fn integer_get_vaild_value_set(&mut self) -> self::Result<Vec<i64>> {
        let mut vaild_value_set = vec![0; 1024 * 1024];
        let mut vaild_value_length = 0;
        unsafe {
            ic4_sys::ic4_prop_integer_get_valid_value_set(
                self.as_mut_ptr(),
                vaild_value_set.as_mut_ptr(),
                ptr_from_mut(&mut vaild_value_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        vaild_value_set.resize(vaild_value_length, 0);
        Ok(vaild_value_set)
    }
}

pub type PropertyFloatRepresentationOri = ic4_sys::IC4_PROPERTY_FLOAT_REPRESENTATION;
bind_type!(PropertyFloatRepresentation, PropertyFloatRepresentationOri);

impl Property {
    pub fn float_get_unit(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_float_get_unit(self.as_mut_ptr())) }
    }
    pub fn float_get_value(&mut self) -> self::Result<f64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn float_set_value(&mut self, value: f64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_float_set_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn float_get_min(&mut self) -> self::Result<f64> {
        let mut min = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_min(self.as_mut_ptr(), ptr_from_mut(&mut min))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(min)
    }
    pub fn float_get_max(&mut self) -> self::Result<f64> {
        let mut max = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_max(self.as_mut_ptr(), ptr_from_mut(&mut max))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(max)
    }
    pub fn float_get_increment(&mut self) -> self::Result<f64> {
        let mut increment = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_inc(self.as_mut_ptr(), ptr_from_mut(&mut increment))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(increment)
    }
    pub fn float_get_increment_mode(&mut self) -> PropertyIncrementMode {
        unsafe { ic4_sys::ic4_prop_float_get_inc_mode(self.as_mut_ptr()) }
    }
    pub fn float_get_valid_value_set(&mut self) -> self::Result<Vec<f64>> {
        let mut vaild_value_set = vec![0.0; 1024 * 1024];
        let mut vaild_value_length = 0;
        unsafe {
            ic4_sys::ic4_prop_float_get_valid_value_set(
                self.as_mut_ptr(),
                vaild_value_set.as_mut_ptr(),
                ptr_from_mut(&mut vaild_value_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        vaild_value_set.resize(vaild_value_length, 0.0);
        Ok(vaild_value_set)
    }
}

pub type PropertyDisplayNotationOri = ic4_sys::IC4_PROPERTY_DISPLAY_NOTATION;
bind_type!(PropertyDisplayNotation, PropertyDisplayNotationOri);

impl Property {
    pub fn get_display_notation(&mut self) -> PropertyDisplayNotation {
        unsafe { ic4_sys::ic4_prop_float_get_display_notation(self.as_mut_ptr()).into() }
    }
    pub fn get_display_precision(&mut self) -> i64 {
        unsafe { ic4_sys::ic4_prop_float_get_display_precision(self.as_mut_ptr()) }
    }
}

impl Property {
    pub fn boolean_get_value(&mut self) -> self::Result<bool> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_boolean_get_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn boolean_set_value(&mut self, value: bool) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_boolean_set_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl Property {
    pub fn string_get_value(&mut self) -> self::Result<CString> {
        let mut value = vec![0; 1024 * 1024];
        let mut value_length = 0;
        unsafe {
            ic4_sys::ic4_prop_string_get_value(
                self.as_mut_ptr(),
                value.as_mut_ptr() as *mut c_char,
                ptr_from_mut(&mut value_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            Ok(CString::from_vec_unchecked(value))
        }
    }
    pub fn string_set_value(&mut self, value: &CStr) -> self::Result<()> {
        unsafe {
            let value_str = value.to_string_lossy();
            ic4_sys::ic4_prop_string_set_value(self.as_mut_ptr(), value.as_ptr(), value_str.len())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn string_get_max_len(&mut self) -> self::Result<u64> {
        let mut max_len = Default::default();
        unsafe {
            ic4_sys::ic4_prop_string_get_max_len(self.as_mut_ptr(), ptr_from_mut(&mut max_len))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(max_len)
    }
}

impl Property {
    pub fn enum_get_entries(&mut self) -> self::Result<PropertyList> {
        let mut entries = null_mut();
        unsafe {
            ic4_sys::ic4_prop_enum_get_entries(self.as_mut_ptr(), ptr_from_mut(&mut entries))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(entries.into())
    }
    pub fn enum_find_entry_by_name(&mut self, entry_name: &CStr) -> self::Result<Property> {
        let mut entry = null_mut();
        unsafe {
            ic4_sys::ic4_prop_enum_find_entry_by_name(
                self.as_mut_ptr(),
                entry_name.as_ptr(),
                ptr_from_mut(&mut entry),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(entry.into())
    }
    pub fn enum_find_entry_by_value(&mut self, entry_value: i64) -> self::Result<Property> {
        let mut entry = null_mut();
        unsafe {
            ic4_sys::ic4_prop_enum_find_entry_by_value(
                self.as_mut_ptr(),
                entry_value,
                ptr_from_mut(&mut entry),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(entry.into())
    }
}

impl Property {
    pub fn enum_get_value(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_enum_get_value(self.as_mut_ptr())) }
    }
    pub fn enum_set_value(&mut self, value: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_enum_set_value(self.as_mut_ptr(), value.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl Property {
    pub fn enum_get_selected_entry(&mut self) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_prop_enum_get_selected_entry(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    pub fn enum_set_selected_entry(&mut self, value: &mut Property) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_enum_set_selected_entry(self.as_mut_ptr(), value.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl Property {
    pub fn enum_get_int_value(&mut self) -> self::Result<i64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_enum_get_int_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn enum_set_int_value(&mut self, value: i64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_enum_set_int_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn enumentry_get_int_value(&mut self) -> self::Result<i64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_enumentry_get_int_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
}

impl Property {
    pub fn register_get_size(&mut self) -> self::Result<u64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_register_get_size(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    // # TODO: is this a bug?
    pub fn register_get_value(&mut self) -> self::Result<Vec<u8>> {
        let mut data = vec![0u8; 1024 * 1024];
        unsafe {
            ic4_sys::ic4_prop_register_get_value(
                self.as_mut_ptr(),
                data.as_mut_ptr() as *mut c_void,
                1024 * 1024,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(data)
    }
    pub fn register_set_value(&mut self, buffer: &[u8]) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_register_set_value(
                self.as_mut_ptr(),
                buffer.as_ptr() as *const c_void,
                buffer.len(),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

/*
 * PropertyList
 */

impl PropertyList {
    pub fn size(&mut self) -> self::Result<usize> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_proplist_size(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    pub fn at(&mut self, index: usize) -> self::Result<Property> {
        let mut value = null_mut();
        unsafe {
            ic4_sys::ic4_proplist_at(self.as_mut_ptr(), index, ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value.into())
    }
}
