#![allow(clippy::redundant_closure)]

use crate::*;

pub mod names;

/*
 * Property
 */

#[doc = " @brief Defines the possible property types\n\n The property type defines the possible operations on a property and its data type."]
pub type PropertyType = ic4_sys::IC4_PROPERTY_TYPE;

#[doc = " @brief Defines the possible property visibilities\n\n Each property has a visibility hint that can be used to create user interfaces for different user types."]
pub type PropertyVisibility = ic4_sys::IC4_PROPERTY_VISIBILITY;

#[doc = " @brief Defines the possible property increment modes for Integer and Float properties"]
pub type PropertyIncrementMode = ic4_sys::IC4_PROPERTY_INCREMENT_MODE;

#[doc = " @struct IC4_PROPERTY\n\n @brief Represents a property of a component, usually of a video capture device.\n\n All property types are referred to by pointers to \\c IC4_PROPERTY, there are no specialized types for\n different property types.\n\n \\c IC4_PROPERTY also represents enumeration entries, even though they are not actually property that is part of\n the category tree. Nevertheless, enumeration entries support all standard property operations.\n\n Property objects are created in multiple ways:\n\t- By calling @ref ic4_propmap_find() or one of its typed sibling functions like @ref ic4_propmap_find_integer().\n\t- By calling @ref ic4_proplist_at() to retrieve a property out of a property list, that was previously obtained:\n\t\t- By calling @ref ic4_prop_category_get_features() to get all properties from a category.\n\t\t- By calling @ref ic4_prop_enum_get_entries() to get the enumeration entries of an enumeration property.\n\t\t- By calling @ref ic4_prop_get_selected_props() to get the properties selected by a property.\n\t\t- By calling @ref ic4_propmap_get_all() to get all properties in a property map's category tree.\n\n Property objects obtained in any of these ways that refer to the same device property will always have the same pointer value.\n\n Calling a function expecting a property with a certain property type on a property with a different type will fail\n and the error value will be set to @ref IC4_ERROR_GENICAM_TYPE_MISMATCH."]
pub type PropertyOri = ic4_sys::IC4_PROPERTY;
bind_ptr_type!(
    Property,
    ic4_sys::IC4_PROPERTY,
    ic4_sys::ic4_prop_ref,
    ic4_sys::ic4_prop_unref
);

#[doc = " @struct IC4_PROPERTY_LIST\n\n @brief Represents a list of properties."]
pub type PropertyListOri = ic4_sys::IC4_PROPERTY_LIST;
bind_ptr_type!(
    PropertyList,
    ic4_sys::IC4_PROPERTY_LIST,
    ic4_sys::ic4_proplist_ref,
    ic4_sys::ic4_proplist_unref
);

#[doc = " @struct IC4_PROPERTY_MAP\n\n @brief Represents the property interface of a component, usually a video capture device.\n\n A property map offers quick access to known properties as well as functions to enumerate all features through the category tree.\n\n This type is opaque, programs only use pointers of type \\c IC4_PROPERTY_MAP*.\n\n Property maps are created by their respective component when asked for, for example by calling #ic4_grabber_device_get_property_map()\n or #ic4_videowriter_get_property_map().\n\n Property map objects are reference-counted.\n To share a propertyp map between multiple parts of a program, create a new reference by calling #ic4_propmap_ref().\n When a reference is no longer required, call #ic4_propmap_unref(). The property map is destroyed when the final reference\n is released.\n"]
pub type PropertyMapOri = ic4_sys::IC4_PROPERTY_MAP;
bind_ptr_type!(
    PropertyMap,
    ic4_sys::IC4_PROPERTY_MAP,
    ic4_sys::ic4_propmap_ref,
    ic4_sys::ic4_propmap_unref
);

impl PropertyMap {
    #[doc = " @brief Execute a command with a known name.\n\n @param[in] map\t\tA property map\n @param[in] prop_name\tName of a command property inside \\c map\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a command property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn execute_command(&mut self, prop_name: PropertyName) -> self::Result<()> {
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
    #[doc = " @brief Get the value of a property with a known name as an integer value.\n\n The behavior depends on the type of the property:\n - For integer properties, the value is returned directly.\n - For boolean properties, the value returned is @c 1 or @c 0.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\t\tA property map\n @param[in] prop_name\t\tName of a property inside @c map\n @param[out] pValue\t\tPointer to an integer to receive the value of the property\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn get_value_i64(&mut self, prop_name: PropertyName) -> self::Result<i64> {
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
    #[doc = " @brief Get the value of a property with a known name as a double value.\n\n The behavior depends on the type of the property:\n - For integer properties, the value is converted to @c double.\n - For float properties, the value is returned directly.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\t\tA property map\n @param[in] prop_name\t\tName of a property inside @c map\n @param[out] pValue\t\tPointer to a double to receive the value of the property\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn get_value_f64(&mut self, prop_name: PropertyName) -> self::Result<f64> {
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
    #[doc = " @brief Get the value of a property with a known name as a bool value.\n\n The behavior depends on the type of the property:\n - For boolean properties, the value is returned directly.\n - For enumeration properties, a value is returned if the name of the currently selected entry unambiguously suggests to represent @c true or @c false.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\t\tA property map\n @param[in] prop_name\t\tName of a property inside @c map\n @param[out] pValue\t\tPointer to a double to receive the value of the property\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn get_value_bool(&mut self, prop_name: PropertyName) -> self::Result<bool> {
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
    #[doc = " @brief Get the value of a property with a known name as a string value.\n\n The behavior depends on the type of the property:\n - For integer properties, the value is converted to a string\n - For float properties, the value is converted to a string\n - For boolean properties, the value is converted to the string @c \"true\" or @c \"false\".\n - For enumeration properties, the name of the currently selected entry is returned.\n - For string properties, the value is returned directly.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\t\t\tA property map\n @param[in] prop_name\t\t\tName of a property inside \\c map\n @param[out] buffer\t\t\tPointer to a character array to receive the string value.\\n\n\t\t\t\t\t\t\t\tThis parameter can be \\c NULL to find out the required space without allocating an initial array.\n @param[in, out] buffer_size\tPointer to a \\c size_t describing the length of the array pointed to by \\a buffer.\\n\n\t\t\t\t\t\t\t\tThe function always writes the actual number of characters required to store the string representation.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn get_value_cstring(&mut self, prop_name: PropertyName) -> self::Result<CString> {
        let mut message_length = 10 * 1024;
        let mut message_vec = vec![0u8; 10 * 1024];
        unsafe {
            ic4_sys::ic4_propmap_get_value_string(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut message_vec) as *mut c_char,
                ptr_from_mut(&mut message_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            message_vec.resize(message_length - 1, 0);
            Ok(CString::from_vec_unchecked(message_vec))
        }
    }
}

impl PropertyMap {
    #[doc = " @brief Set the value of a property with a known name to the passed integer value.\n\n The behavior depends on the type of the property:\n - For integer properties, the value is set directly.\n - For float properties, the value is set directly.\n - For boolean properties, if the value is @c 1 or @c 0, it is set to @c true or @c false respectively. Other values result in an error.\n - For enumeration properties, the numeric value is set if the property is @c PixelFormat. Other properties result in an error.\n - For command properties, the command is executed if @a value is @c 1.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\tA property map\n @param[in] prop_name\tName of a property inside \\c map\n @param[in] value\t\tNew value to be set for the property identified by \\c prop_name\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn set_value_i64(&mut self, prop_name: PropertyName, value: i64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_int64(self.as_mut_ptr(), prop_name.as_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Set the value of a property with a known name to the passed double value.\n\n The behavior depends on the type of the property:\n - For integer properties, the value is rounded to the nearest integer.\n - For float properties, the value is set directly.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\tA property map\n @param[in] prop_name\tName of a property inside \\c map\n @param[in] value\t\tNew value to be set for the property identified by \\c prop_name\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn set_value_f64(&mut self, prop_name: PropertyName, value: f64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_double(self.as_mut_ptr(), prop_name.as_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Set the value of a property with a known name to the passed bool value.\n\n The behavior depends on the type of the property:\n - For boolean properties, the value is set directly.\n - For enumeration properties, it selects the entry with a name that unambiguously suggests to represent @c true or @c false, if available.\n - For command properties, the command is executed if @a value is @c true.\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\tA property map\n @param[in] prop_name\tName of a property inside \\c map\n @param[in] value\t\tNew value to be set for the property identified by \\c prop_name\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn set_value_bool(&mut self, prop_name: PropertyName, value: bool) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_set_value_bool(self.as_mut_ptr(), prop_name.as_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Set the value of a property with a known name to the passed string value.\n\n The behavior depends on the type of the property:\n - For integer properties, the string is parsed, and the found integer value is set\n - For float properties, the string is parsed, and the found floating-point value is set\n - For boolean properties, a value is set if the string can be unambiguously identified to represent @c true or @c false.\n - For enumeration properties, the entry with a name or display name matching the value is set.\n - For string properties, the value is set directly.\n - For command properties, the command is executed if @a value is @c \"1\", @c \"true\" or @c \"execute\".\n - For all other property types, the call results in an error (@ref IC4_ERROR_GENICAM_TYPE_MISMATCH).\n\n @param[in] map\t\tA property map\n @param[in] prop_name\tName of a property inside \\c map\n @param[in] value\t\tNew value to be set for the property identified by \\c prop_name\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name in \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn set_value_cstr(&mut self, prop_name: PropertyName, value: &CStr) -> self::Result<()> {
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
    #[doc = " @brief Returns a property object representing the property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n"]
    pub fn find(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the command property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a command property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a command property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_command(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the integer property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of an integer property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_integer(&mut self, prop_name: PropertyName) -> self::Result<Property> {
        let mut property = null_mut();
        unsafe {
            ic4_sys::ic4_propmap_find_integer(
                self.as_mut_ptr(),
                prop_name.as_ptr(),
                ptr_from_mut(&mut property),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property.into())
    }
    #[doc = " @brief Returns a property object representing the float property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a float property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_float(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the boolean property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a boolean property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a boolean property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_boolean(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the string property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a string property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a string property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_string(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the enumeration property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of an enumeration property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_enumeration(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the register property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a register property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a register property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_register(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a property object representing the category property with the passed name.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tprop_name\tThe name of a category property inside \\c map\n @param[out]\tppProperty\tPointer to a handle receiving the property object\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If there is no property with the name \\c prop_name inside \\c map, the function fails and the error value is set to #IC4_ERROR_GENICAM_FEATURE_NOT_FOUND. \\n\n If there is a property with the name \\c prop_name, but it is not a category property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn find_category(&mut self, prop_name: PropertyName) -> self::Result<Property> {
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
    #[doc = " @brief Returns a list of all properties reachable from the property map's \"Root\" category.\n\n @param[in]\tmap\t\t\tA property map\n @param[out]\tppList\t\tA pointer to a property list receiving the list of properties.\\n\n\t\t\t\t\t\t\tWhen the property list is no longer required, release the object reference using #ic4_proplist_unref().\n\n @note\n This generated list does not include category properties.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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
    #[doc = " @brief Enables the use of the chunk data in the passed #IC4_IMAGE_BUFFER as a backend for chunk properties in the property map.\n\n @param[in]\tmap\t\t\t\tA property map\n @param[in]\timage_buffer\tAn image buffer with chunk data.\\n\n\t\t\t\t\t\t\t\tThis parameter may be \\c NULL to disconnect the previously connected buffer.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n The property map takes a reference to the passed image buffer, extending its lifetime and preventing automatic reuse.\n The reference is released when a new image buffer is connected to the property map, or \\c NULL is passed in the \\c image_buffer argument.\n"]
    pub fn connect_chunkdata(&mut self, image_buffer: &mut ImageBuffer) -> Result<()> {
        unsafe {
            ic4_sys::ic4_propmap_connect_chunkdata(self.as_mut_ptr(), image_buffer.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Saves the state of the properties in this property map into a file.\n\n @param[in]\tmap\t\tA property map\n @param[in]\tpath\tA path to a file that the property state is written to\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n The actual set of properties that is stored by this function is controlled by the object the property map belongs to.\\n\n To restore the properties at a later time, use #ic4_propmap_deserialize_from_file.\n\n @see ic4_propmap_deserialize_from_file\n @see ic4_propmap_serialize_to_memory"]
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
    ///
    #[doc = " @brief Restores the state of the properties in this property map from a file that was previously written by #ic4_propmap_serialize_to_file.\n\n @param[in]\tmap\t\tA property map\n @param[in]\tpath\tPath to a file that was previously written by #ic4_propmap_serialize_to_file\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n If the file contains settings for properties that could not be written, the function fails and the error value is set to #IC4_ERROR_INCOMPLETE.\n\n @see ic4_propmap_serialize_to_file"]
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

#[doc = " @brief Callback function called to allocate memory during the call of #ic4_propmap_serialize_to_memory.\n\n @param[in]\tsize\tSize of the memory buffer to be allocated.\n\n @return\t\tThe pointer to the allocated memory buffer, or @c NULL if the allocation was not possible.\n\n @note\n If this function returns @c NULL, the call to #ic4_propmap_serialize_to_memory will fail.\n\n @see ic4_propmap_serialize_to_memory"]
pub type SerlalizationAllocator = ic4_sys::ic4_serialization_allocator;

impl PropertyMap {
    #[doc = " @brief Saves the state of the properties in this property map in a memory buffer.\n\n @param[in]\t\tmap\t\t\tA property map\n @param[in]\t\talloc\t\tPointer to a function that allocates the memory buffer.\\n\n\t\t\t\t\t\t\t\tFor example, @c malloc can be passed here.\n @param[out]\t\tppData\t\tPointer to a pointer to receive the newly-allocated memory buffer containing the properties.\\n\n\t\t\t\t\t\t\t\tThe caller is responsible for releasing the memory, using a function that can free memory returned by @c alloc.\n @param[out]\t\tdata_size\tPointer to size_t to receive the size of the memory buffer allocated by the call\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n The actual set of properties that is stored by this function is controlled by the object the property map belongs to.\\n\n To restore the properties at a later time, use #ic4_propmap_deserialize_from_memory.\n\n @see ic4_propmap_deserialize_from_memory\n @see ic4_propmap_serialize_to_file"]
    pub fn serialize_to_memory(&mut self, alloc: SerlalizationAllocator) -> Result<Vec<u8>> {
        let mut data = null_mut();
        let mut data_size = 0;
        unsafe {
            ic4_sys::ic4_propmap_serialize_to_memory(
                self.as_mut_ptr(),
                alloc,
                ptr_from_mut(&mut data),
                ptr_from_mut(&mut data_size),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            Ok(Vec::from_raw_parts(data as *mut u8, data_size, data_size))
        }
    }
    #[doc = " @brief Restores the state of the properties in this property map from a memory buffer containing data that was previously written by #ic4_propmap_serialize_to_memory.\n\n @param[in]\tmap\t\t\tA property map\n @param[in]\tpData\t\tPointer to a memory buffer containing data that was written by #ic4_propmap_serialize_to_memory\n @param[in]\tdata_size\tSize of the memory buffer pointed to by @c pData\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n If the memory buffer contains settings for properties that could not be written, the function fails and the error value is set to #IC4_ERROR_INCOMPLETE.\n\n @see ic4_propmap_serialize_to_memory"]
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
    #[doc = " @brief Returns the type of the passed property\n\n @param[in] prop\tA property\n\n @return\tThe type of the property \\c prop, or #IC4_PROPTYPE_INVALID in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @see IC4_PROPERTY_TYPE"]
    pub fn get_type(&mut self) -> PropertyType {
        unsafe { ic4_sys::ic4_prop_get_type(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the name of the passed property\n\n @param[in] prop\tA property\n\n @return\tThe name of the passed property, or \\c NULL in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\t\t\tThe memory pointed to by the return value is valid as long as the property object exists.\n\n @remarks\n A property's name is the symbolic name with which it can be found in a @ref propmap, for example \\c ExposureTime or \\c AcquisitionFrameRate.\n\n @see ic4_prop_get_display_name"]
    pub fn get_name(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_name(self.as_mut_ptr())) }
    }
    #[doc = " @brief Checks whether a property is currently available.\n\n If a property is not available, attempts to read or write its value will fail.\n\n A property may become unavailable, if its value does not have a meaning in the current state of the device.\n The property's availability status can change upon writing to another property.\n\n @param[in] prop\tA property\n\n @return\t\\c true, if the property is currently available, otherwise \\c false.\\n\n\t\t\tIf there is an error, this function returns \\c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_prop_is_locked\n @see ic4_prop_is_readonly"]
    pub fn is_available(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_available(self.as_mut_ptr()) }
    }
    #[doc = " @brief Checks whether a property is currently locked.\n\n A locked property can be read, but attempts to write its value will fail.\n\n A property's locked status may change upon writing to another property.\n\n Common examples for locked properties are \\c ExposureTime or \\c Gain if \\c ExposureAuto or \\c GainAuto are enabled.\n\n @param[in] prop\tA property\n\n @return\t\\c true, if the property is currently locked, otherwise \\c false.\\n\n\t\t\tIf there is an error, this function returns \\c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_prop_is_available\n @see ic4_prop_is_readonly\n @see ic4_prop_is_likely_locked_by_stream"]
    pub fn is_locked(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_locked(self.as_mut_ptr()) }
    }
    #[doc = " @brief Tries to determine whether a property is locked because a data stream is active.\n\n @param[in] prop\tA property\n\n @return\t@c true, if the property is currently locked, and will likely be unlocked if the data stream is stopped.\\n\n\t\t\t@c false, if the property is not currently locked, or stopping the data stream will probably not lead to\n\t\t\tthe property being unlocked.\\n\n\t\t\tIf there is an error, this function returns \\c false. Use ic4_get_last_error() to query error information.\n\n @remarks\tFor technical reasons, this function cannot always accurately predict the future.\n\n @see ic4_prop_is_locked"]
    pub fn is_likely_locked_by_stream(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_likely_locked_by_stream(self.as_mut_ptr()) }
    }
    #[doc = " @brief Checks whether a property is read-only.\n\n A read-only property will never be writable, the read-only status will never change.\n\n A Common examples for read-only property is \\c DeviceTemperature.\n\n @param[in] prop\tA property\n\n @return\t\\c true, if the property is read-only, otherwise \\c false.\\n\n\t\t\tIf there is an error, this function returns \\c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_prop_is_available\n @see ic4_prop_is_locked"]
    pub fn is_readonly(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_readonly(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns a visibility hint for the property.\n\n The visibility hint can be used to create user interfaces with different complexities. The most commonly used properties\n have the beginner visibility, while rarely used or diagnostic features might be tagged guru or even invisible.\n\n @param[in] prop\tA property\n\n @return\tThe visibility hint for the property.\\n\n\t\t\tIf there is an error, this function returns @ref IC4_PROPVIS_INVISIBLE. Use ic4_get_last_error() to query error information."]
    pub fn get_visibility(&mut self) -> PropertyVisibility {
        unsafe { ic4_sys::ic4_prop_get_visibility(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the display name of the passed property\n\n @param[in] prop\tA property\n\n @return\tThe display name of the passed property, or \\c NULL in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\t\t\tThe memory pointed to by the return value is valid as long as the property object exists.\n\n @remarks\n A property's display name is a text representation of the property that is meant to be displayed in user interfaces.\\n\n For example, the display name of the \\c ExposureTime property usually is <i>Exposure Time</i>.\n\n @see ic4_prop_get_name"]
    pub fn get_display_name(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_display_name(self.as_mut_ptr())) }
    }
    #[doc = " @brief Returns a tooltip for the passed property\n\n @param[in] prop\tA property\n\n @return\tThe tooltip for the passed property, or \\c NULL in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\t\t\tThe memory pointed to by the return value is valid as long as the property object exists.\n\n @remarks\n A property's tooltip is a text that can be used when a tooltip is required by a user interface.\\n\n Usually, the tooltip is a short description of the property.\n\n @see ic4_prop_get_description"]
    pub fn get_tooltip(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_tooltip(self.as_mut_ptr())) }
    }
    #[doc = " @brief Returns a description text for the passed property\n\n @param[in] prop\tA property\n\n @return\tThe description for the passed property, or \\c NULL in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\t\t\tThe memory pointed to by the return value is valid as long as the property object exists.\n\n @remarks\n A property's description is a short text that describes the property, usually in more detail than the tooltip.\n\n @see ic4_prop_get_tooltip"]
    pub fn get_description(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_get_description(self.as_mut_ptr())) }
    }
}

#[doc = " @brief Property notification handler function pointer\n\n @param[in] prop\t\tThe property that has changed\n @param[in] user_ptr\tThe user data that was specified when the notification handler was registered\n\n @see ic4_prop_event_add_notification"]
pub type PropertyNotification = ic4_sys::ic4_prop_notification;

#[doc = " @brief Property notification deleter\n\n @param[in] user_ptr\tThe user data that was specified when the notification deleter was registered\n\n @see ic4_prop_event_add_notification\n @see ic4_prop_event_remove_notification"]
pub type PropertyNotificationDeleter = ic4_sys::ic4_prop_notification_deleter;

impl Property {
    /// # Safety
    ///
    /// see ic4_sys::ic4_prop_event_add_notification
    ///
    #[doc = " @brief Registers a property notification handler\n\n The property notification handler is called whenever an aspect of the property changes, for example its value or locked status.\n\n @param[in] prop\t\tA property\n @param[in] handler\tThe property notification handler to be called if \\a prop changes\n @param[in] user_ptr\tUser data to be passed to the notification handler\n @param[in] deleter\tThe property notification deleter to be called if the notification is removed, or the property is destroyed\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n The \\c deleter callback can useful to release resources that the caller passed in into \\c user_ptr.\n\n @remarks\n Multiple notification handlers can be registered for the same property, as long as the \\c user_ptr parameter is different.\n\n @see ic4_prop_event_remove_notification\n @see ic4_prop_notification\n @see ic4_prop_notification_deleter"]
    pub unsafe fn event_add_notification(
        &mut self,
        handler: PropertyNotification,
        user_ptr: *mut c_void,
        deleter: PropertyNotificationDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_prop_event_add_notification(self.as_mut_ptr(), handler, user_ptr, deleter)
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        Ok(())
    }

    /// # Safety
    ///
    /// see ic4_sys::ic4_prop_event_remove_notification
    ///
    #[doc = " @brief Unregisters a previously registered notification handler\n\n @param[in] prop\t\tA property with a registered notification handler\n @param[in] handler\tThe property notification handler to be removed\n @param[in] user_ptr\tThe user data that was specified when the handler was registered\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n The values for \\c handler and \\c user_ptr must exactly match the arguments of the previous successful call to #ic4_prop_event_add_notification().\n\n @see ic4_prop_event_add_notification"]
    pub unsafe fn event_remove_notification(
        &mut self,
        handler: PropertyNotification,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_prop_event_remove_notification(self.as_mut_ptr(), handler, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        Ok(())
    }
}

impl Property {
    #[doc = " @brief Indicates whether this property's value changes the meaning and/or value of other properties.\n\n @param[in] prop\t\t\tA property\n\n @return @c true, if @c prop is a selector, otherwise @c false.\\n\n\t\t\tIf an error occurs, the function returns @c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_prop_get_selected"]
    pub fn is_selector(&mut self) -> bool {
        unsafe { ic4_sys::ic4_prop_is_selector(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the list of properties whose values' meaning depend on this property.\n\n @param[in] prop\t\t\tA property that is a selector\n @param[out] ppSelected\tA pointer to a property list receiving the list of properties selected by \\c prop.\n\t\t\t\t\t\t\tWhen the property list is no longer required, release the object reference using #ic4_proplist_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a boolean property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_is_selector"]
    pub fn get_selected_props(&mut self) -> Result<PropertyList> {
        let mut data = null_mut();
        unsafe {
            ic4_sys::ic4_prop_get_selected_props(self.as_mut_ptr(), ptr_from_mut(&mut data))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
            Ok(data.into())
        }
    }
    #[doc = " @brief Retrieves the list of properties in a property category.\n\n @param[in] prop\t\t\tA category property\n @param[in] ppFeatures\tA pointer to a property list receiving the list of properties inside the category \\c prop.\n\t\t\t\t\t\t\tWhen the property list is no longer required, release the object reference using #ic4_proplist_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a category property, this function returns \\c false and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH."]
    pub fn category_get_features(&mut self) -> Result<PropertyList> {
        let mut data = null_mut();
        unsafe {
            ic4_sys::ic4_prop_category_get_features(self.as_mut_ptr(), ptr_from_mut(&mut data))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
            Ok(data.into())
        }
    }
    #[doc = " @brief Execute a command property\n\n @param[in] prop\t\t\tA command property\n\n @return @c true on success, otherwise @c false.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a command property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn command_execute(&mut self) -> Result<()> {
        unsafe {
            ic4_sys::ic4_prop_command_execute(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Checks whether a command has finished executing.\n\n @param prop\t\t\t\tA command property\n @param is_done\t\t\tOutput parameter receiving the command's completion status.\\n\n\t\t\t\t\t\t\t@c true, if the command is completed. @c false, if the command is still executing.\n\n @return @c true on success, otherwise @c false.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information.\n\n @remarks\n If the command was never executed before, the @a is_done is set to @c false."]
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

#[doc = " @brief Defines the possible integer property representations\n\n Each integer property has a representation hint that can help creating more useful user interfaces."]
pub type PropertyIntRepresentation = ic4_sys::IC4_PROPERTY_INT_REPRESENTATION;

impl Property {
    #[doc = " @brief Returns the suggested representation for an integer property.\n\n The representation can be used as a hint when creating user interfaces.\n\n @param[in] prop\tAn integer property\n\n @return\tThe suggested representation for the property, or a default representation in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see IC4_PROPERTY_INT_REPRESENTATION"]
    pub fn integer_get_representation(&mut self) -> PropertyIntRepresentation {
        unsafe { ic4_sys::ic4_prop_integer_get_representation(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the unit of an integer property.\n\n @param[in] prop\tAn integer property\n\n @return\tThe unit of the the property. The unit can be an empty string, if there is no unit for the property.\\n\n\t\t\tThe memory pointed to by the return value is valid as long as the property object exists.\\n\n\t\t\tIf an error occurs, the function returns @c NULL. Use ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn integer_get_unit(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_integer_get_unit(self.as_mut_ptr())) }
    }
    #[doc = " @brief Reads the current value of an integer property.\n\n @param[in] prop\t\t\tAn integer property\n @param[out] pValue\t\tPointer to an integer to receive the current value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn integer_get_value(&mut self) -> self::Result<i64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    #[doc = " @brief Changes the value of an integer property.\n\n @param[in] prop\t\t\tAn integer property\n @param[in] value\t\t\tThe new value to set\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If the device or component rejected the value, the function fails and the error value is set to #IC4_ERROR_GENICAM_VALUE_ERROR. \\n\n If the value is currently not writable, the function fails and the error value is set to #IC4_ERROR_GENICAM_ACCESS_DENIED. \\n\n\n @see ic4_prop_integer_get_min\n @see ic4_prop_integer_get_max\n @see ic4_prop_integer_get_inc"]
    pub fn integer_set_value(&mut self, value: i64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_integer_set_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Returns the minimum value accepted by an integer property.\n\n @param[in] prop\t\t\tAn integer property\n @param[out] pMinimum\t\tPointer to an integer to receive the minimum value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_integer_get_max\n @see ic4_prop_integer_get_inc"]
    pub fn integer_get_min(&mut self) -> self::Result<i64> {
        let mut min = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_min(self.as_mut_ptr(), ptr_from_mut(&mut min))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(min)
    }
    #[doc = " @brief Returns the maximum value accepted by an integer property.\n\n @param[in] prop\t\t\tAn integer property\n @param[out] pMaximum\t\tPointer to an integer to receive the maximum value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_integer_get_min\n @see ic4_prop_integer_get_inc"]
    pub fn integer_get_max(&mut self) -> self::Result<i64> {
        let mut max = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_max(self.as_mut_ptr(), ptr_from_mut(&mut max))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(max)
    }
    #[doc = " @brief Returns the step size for valid values accepted by an integer property.\n\n The increment restricts the set of valid values for an integer property.\n For example, if the property's minimum value is \\c 0, the maximum is \\c 10, and the increment is \\c 2, \\c 5 is not a valid value for the property\n and will be rejected when trying to write it.\n\n @param[in] prop\t\t\tAn integer property\n @param[out] pIncrement\tPointer to an integer to receive the increment\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an integer property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_integer_get_min\n @see ic4_prop_integer_get_max"]
    pub fn integer_get_increment(&mut self) -> self::Result<i64> {
        let mut increment = Default::default();
        unsafe {
            ic4_sys::ic4_prop_integer_get_inc(self.as_mut_ptr(), ptr_from_mut(&mut increment))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(increment)
    }
    #[doc = " @brief Returns how this integer property restricts which values are valid between its minimum and maximum value.\n\n @param[in] prop\tAn integer property\n\n @return\t#IC4_PROPINCMODE_INCREMENT if the property has a fixed step size between valid values.\\n\n\t\t\t#IC4_PROPINCMODE_VALUESET, if the property has a set of valid values.\\n\n\t\t\tIf an error occurs, the function returns #IC4_PROPINCMODE_INCREMENT and the error value is set.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @see ic4_prop_integer_get_inc\n @see ic4_prop_integer_get_valid_value_set"]
    pub fn integer_get_increment_mode(&mut self) -> PropertyIncrementMode {
        unsafe { ic4_sys::ic4_prop_integer_get_inc_mode(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the set of valid values for an integer\n\n @param[in] prop\t\t\t\tAn integer property restricted to a set of values\n @param[out] value_set\t\tAn array to receive the set of valid values\n @param[in, out] array_size\tPointer to a @c size_t indicating the length of @c value_set.\\n\n\t\t\t\t\t\t\t\tAfter the call, this contains the number of entries in this property's set of valid values.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n If @c value_set is not @c NULL and @a array_size is \\c NULL, the function fails and the error value is set to #IC4_ERROR_INVALID_PARAM_VAL.\n If @c *array_size is lower than the number of entries in this property's set of valid values, the function fails the error value is set to #IC4_ERROR_BUFFER_TOO_SMALL.\n If @c prop is not restricted by a set of valid values, the function fails and the error value is set to #IC4_ERROR_GENICAM_NOT_IMPLEMENTED.\n"]
    pub fn integer_get_vaild_value_set(&mut self) -> self::Result<Vec<i64>> {
        let mut vaild_value_length = 10 * 1024;
        let mut vaild_value_set = vec![0; 10 * 1024];
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

#[doc = " @brief Defines the possible float property representations\n\n Each float property has a representation hint that can help creating more useful user interfaces."]
pub type PropertyFloatRepresentation = ic4_sys::IC4_PROPERTY_FLOAT_REPRESENTATION;

impl Property {
    #[doc = " @brief Returns the suggested represenation for a float property.\n\n The representation can be used as a hint when creating user interfaces.\n\n @param[in] prop\tA float property\n\n @return\tThe suggested representation for the property, or a default representation in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see IC4_PROPERTY_FLOAT_REPRESENTATION"]
    pub fn float_get_representation(&mut self) -> PropertyFloatRepresentation {
        unsafe { ic4_sys::ic4_prop_float_get_representation(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the unit of a float property.\n\n @param[in] prop\tA float property\n\n @return\tThe unit of the the property. The unit can be an empty string, if there is no unit for the property.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\t\t\tThe memory pointed to by the return value is valid as long as the property object exists.\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn float_get_unit(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_float_get_unit(self.as_mut_ptr())) }
    }
    #[doc = " @brief Reads the current value of a float property.\n\n @param[in] prop\t\t\tA float property\n @param[out] pValue\t\tPointer to a double to receive the current value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn float_get_value(&mut self) -> self::Result<f64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    #[doc = " @brief Changes the value of a float property.\n\n @param[in] prop\t\t\tA float property\n @param[in] value\t\t\tThe new value to set\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If the device or component rejected the value, the function fails and the error value is set to #IC4_ERROR_GENICAM_VALUE_ERROR. \\n\n If the value is currently not writable, the function fails and the error value is set to #IC4_ERROR_GENICAM_ACCESS_DENIED. \\n\n\n @see ic4_prop_float_get_min\n @see ic4_prop_float_get_max\n @see ic4_prop_float_get_inc"]
    pub fn float_set_value(&mut self, value: f64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_float_set_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Returns the minimum value accepted by a float property.\n\n @param[in] prop\t\t\tA float property\n @param[out] pMinimum\t\tPointer to an double to receive the minimum value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_integer_get_max\n @see ic4_prop_integer_get_inc"]
    pub fn float_get_min(&mut self) -> self::Result<f64> {
        let mut min = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_min(self.as_mut_ptr(), ptr_from_mut(&mut min))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(min)
    }
    #[doc = " @brief Returns the maximum value accepted by a float property.\n\n @param[in] prop\t\t\tA float property\n @param[out] pMaximum\t\tPointer to an double to receive the maximum value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_integer_get_min\n @see ic4_prop_integer_get_inc"]
    pub fn float_get_max(&mut self) -> self::Result<f64> {
        let mut max = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_max(self.as_mut_ptr(), ptr_from_mut(&mut max))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(max)
    }
    #[doc = " @brief Returns the step size for valid values accepted by a float property.\n\n The increment restricts the set of valid values for a float property.\n For example, if the property's minimum value is \\c 0.0, the maximum is \\c 1.0, and the increment is \\c 0.5, \\c 1.25 is not a valid value for the property\n and will be rejected when trying to write it.\n\n @param[in] prop\t\t\tA float property\n @param[out] pIncrement\tPointer to a double to receive the increment\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_float_get_min\n @see ic4_prop_float_get_max\n @see ic4_prop_float_has_inc"]
    pub fn float_get_increment(&mut self) -> self::Result<f64> {
        let mut increment = Default::default();
        unsafe {
            ic4_sys::ic4_prop_float_get_inc(self.as_mut_ptr(), ptr_from_mut(&mut increment))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(increment)
    }
    #[doc = " @brief Returns how this float property restricts which values are valid between its minimum and maximum value.\n\n @param[in] prop\tA float property\n\n @return\t@ref IC4_PROPINCMODE_INCREMENT, if the property has a fixed step size between valid values.\\n\n\t\t\t@ref IC4_PROPINCMODE_VALUESET, if the property has a set of valid values.\\n\n\t\t\t@ref IC4_PROPINCMODE_NONE, if the property can be set to any value between its minimum and maximum.\\n\n\t\t\tIf an error occurs, the function returns @ref IC4_PROPINCMODE_NONE and the error value is set.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If @c prop is not a float property, the function fails and the error value is set to @ref IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_float_get_inc\n @see ic4_prop_float_get_valid_value_set"]
    pub fn float_get_increment_mode(&mut self) -> PropertyIncrementMode {
        unsafe { ic4_sys::ic4_prop_float_get_inc_mode(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns the set of valid values for a float\n\n @param[in] prop\t\t\t\tA float property restricted to a set of values\n @param[out] value_set\t\tAn array to receive the set of valid values\n @param[in, out] array_size\tPointer to a @c size_t indicating the length of @c value_set.\\n\n\t\t\t\t\t\t\t\tAfter the call, this contains the number of entries in this property's set of valid values.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n If @c value_set is not @c NULL and @a array_size is \\c NULL, the function fails and the error value is set to #IC4_ERROR_INVALID_PARAM_VAL.\n If @c *array_size is lower than the number of entries in this property's set of valid values, the function fails the error value is set to #IC4_ERROR_BUFFER_TOO_SMALL.\n If @c prop is not restricted by a set of valid values, the function fails and the error value is set to #IC4_ERROR_GENICAM_NOT_IMPLEMENTED.\n"]
    pub fn float_get_valid_value_set(&mut self) -> self::Result<Vec<f64>> {
        let mut vaild_value_length = 10 * 1024;
        let mut vaild_value_set = vec![0.0; 10 * 1024];
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

pub type PropertyDisplayNotation = ic4_sys::IC4_PROPERTY_DISPLAY_NOTATION;

impl Property {
    #[doc = " @brief Returns a suggested display notation to use when displaying the float property's value.\n\n @param[in] prop\tA float property\n\n @return\tA display notation suggestion, or @ref IC4_PROPDISPNOTATION_AUTOMATIC if there is an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn get_display_notation(&mut self) -> PropertyDisplayNotation {
        unsafe { ic4_sys::ic4_prop_float_get_display_notation(self.as_mut_ptr()) }
    }
    #[doc = " @brief Returns a suggested number of significant digits to use when displaying the float property's value.\n\n @param[in] prop\tA float property\n\n @return\tThe suggested number of significant digits for display, or a default value if there is an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\n @remarks\n If \\c prop is not a float property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn get_display_precision(&mut self) -> i64 {
        unsafe { ic4_sys::ic4_prop_float_get_display_precision(self.as_mut_ptr()) }
    }
}

impl Property {
    #[doc = " @brief Reads the current value of a boolean property.\n\n @param[in] prop\t\t\tA boolean property\n @param[out] pValue\t\tPointer to a bool to receive the current value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a boolean property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn boolean_get_value(&mut self) -> self::Result<bool> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_boolean_get_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    #[doc = " @brief Changes the value of a boolean property.\n\n @param[in] prop\t\t\tAn boolean property\n @param[in] value\t\t\tThe new value to set\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a boolean property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If the value is currently not writable, the function fails and the error value is set to #IC4_ERROR_GENICAM_ACCESS_DENIED. \\n"]
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
    #[doc = " @brief Reads the current value of a string property.\n\n @param[in] prop\t\t\t\tA string property\n @param[out] buffer\t\t\tPointer to a character array to receive the string value.\\n\n\t\t\t\t\t\t\t\tThis parameter can be \\c NULL to find out the required space without allocating an initial array.\n @param[in, out] buffer_size\tPointer to a \\c size_t describing the length of the array pointed to by \\a buffer.\\n\n\t\t\t\t\t\t\t\tThe function always writes the actual number of characters required to store the string representation.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a string property, the function fails and the error value is set to @ref IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If \\c buffer is not \\c NULL, and \\c *buffer_size is less than the length of the value of this property, the function fails and the error value is set to @ref IC4_ERROR_BUFFER_TOO_SMALL. \\n"]
    pub fn string_get_value(&mut self) -> self::Result<CString> {
        let mut value_length = 10 * 1024;
        let mut value = vec![0; 10 * 1024];
        unsafe {
            ic4_sys::ic4_prop_string_get_value(
                self.as_mut_ptr(),
                value.as_mut_ptr() as *mut c_char,
                ptr_from_mut(&mut value_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            value.resize(value_length - 1, 0);
            Ok(CString::from_vec_unchecked(value))
        }
    }
    #[doc = " @brief Changes the value of a string property.\n\n @param[in] prop\t\t\tA string property\n @param[in] buffer\t\tPointer to a buffer containing the new string value\n @param[in] buffer_size\tLength of \\c buffer.\\n\n\t\t\t\t\t\t\tIf \\c 0, interpret \\c buffer as a null-terminated string.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a string property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If the value is currently not writable, the function fails and the error value is set to #IC4_ERROR_GENICAM_ACCESS_DENIED. \\n"]
    pub fn string_set_value(&mut self, value: &CStr) -> self::Result<()> {
        unsafe {
            let value_str = value.to_string_lossy();
            ic4_sys::ic4_prop_string_set_value(self.as_mut_ptr(), value.as_ptr(), value_str.len())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Returns the maximum length of the string that can be stored in this property.\n\n @param[in] prop\t\t\tA string property\n @param[out] pMaxLength\tPointer to an integer to receive the maximum length of the string value of this property\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a string property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
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
    #[doc = " @brief Returns the list of entries in this enumeration property.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[out] ppList\t\tA pointer to a property list receiving the list of enumeration entries.\\n\n\t\t\t\t\t\t\tWhen the property list is no longer required, release the object reference using #ic4_proplist_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n"]
    pub fn enum_get_entries(&mut self) -> self::Result<PropertyList> {
        let mut entries = null_mut();
        unsafe {
            ic4_sys::ic4_prop_enum_get_entries(self.as_mut_ptr(), ptr_from_mut(&mut entries))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(entries.into())
    }
    #[doc = " @brief Finds the enumeration entry with a given name in this enumeration property.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[in] entry_name\tThe name of one of this enumeration property's enumeration entries\n @param[out] ppEntry\t\tA pointer to a property receiving the requested enumeration entry.\\n\n\t\t\t\t\t\t\tWhen the enumeration entry is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_enum_find_entry_by_value"]
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
    #[doc = " @brief Finds the enumeration entry with a given value in this enumeration property.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[in] entry_value\tThe value of one of this enumeration property's enumeration entries\n @param[out] ppEntry\t\tA pointer to a property receiving the requested enumeration entry.\\n\n\t\t\t\t\t\t\tWhen the enumeration entry is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_enum_find_entry_by_name"]
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
    #[doc = " @brief Returns the name of the currently selected entry of the enumeration.\n\n @param prop\t\t\t\tAn enumeration property\n\n @return\tThe name of the enumeration's currently selected entry, or \\c NULL in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\\n\n\t\t\tThe memory pointed to by the return value is valid at least as the property object exists,\n\t\t\tor until the next call to @c ic4_prop_enum_get_value on this enumeration.\n\n @remarks\n If @c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH."]
    pub fn enum_get_value(&mut self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_prop_enum_get_value(self.as_mut_ptr())) }
    }
    #[doc = " @brief Sets the enumeration's selected entry by name.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[in] entry_name\tThe name of an enumeration entry of @c prop.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If @c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If @c entry_name is not the name of an entry of @c prop, the function fails and the error value is set to #IC4_ERROR_GENICAM_VALUE_ERROR. \\n\n\n @see ic4_prop_enum_set_selected_entry\n @see ic4_prop_enum_set_int_value"]
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
    #[doc = " @brief Returns the currently selected entry of this enumeration property.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[out] ppEntry\t\tA pointer to a property receiving the currently selected enumeration entry.\\n\n\t\t\t\t\t\t\tWhen the enumeration entry is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_enum_get_int_value"]
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
    #[doc = " @brief Sets the enumeration's selected entry.\n\n @param[in] prop\t\tAn enumeration property\n @param[in] entry\t\tAn enumeration entry of @c prop.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If @c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If @c entry is not an enumeration entry property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If @c entry is not an enumeration entry of @c prop, the function fails and the error value is set to #IC4_ERROR_GENICAM_VALUE_ERROR. \\n\n\n @see ic4_prop_enum_set_value\n @see ic4_prop_enum_set_int_value"]
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
    #[doc = " @brief Returns the value of the currently selected entry of an enumeration property.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[out] pValue\t\tA pointer to a double receiving the currently selected enumeration entry's value\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_enum_get_selected_entry\n @see ic4_prop_enum_set_int_value"]
    pub fn enum_get_int_value(&mut self) -> self::Result<i64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_enum_get_int_value(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    #[doc = " @brief Selects the currently selected entry of this enumeration property by specifying the entry's value.\n\n This method can be useful to directly set a known enumeration entry, for example setting the \\c PixelFormat to @ref IC4_PIXEL_FORMAT_Mono8.\n\n @param[in] prop\t\t\tAn enumeration property\n @param[in] entry_value\tThe value of an enumeration entry of \\c prop.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_enum_set_selected_entry\n @see ic4_prop_enum_get_int_value"]
    pub fn enum_set_int_value(&mut self, value: i64) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_prop_enum_set_int_value(self.as_mut_ptr(), value)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Returns the value of an enumeration entry.\n\n @param[in] prop\t\t\tAn enumeration entry\n @param[out] pValue\t\tA pointer to a double receiving the value of the enumeration entry\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not an enumeration entry, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @see ic4_prop_enum_get_entries"]
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
    #[doc = " @brief Queries the size of a register property.\n\n The size of a register property is not necessarily constant; it can change depending on the value of other properties.\n\n @param[in] prop\t\tA register property\n @param[out] pSize\tPointer to a @c uint64_t to receive the data size of the register in bytes.\n\n @remarks\n If \\c prop is not a register property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn register_get_size(&mut self) -> self::Result<u64> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_prop_register_get_size(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    #[doc = " @brief Reads data from a register property.\n\n @param[in]\tprop\t\tA register property\n @param[out]\tbuffer\t\tA buffer receiving the data from the property\n @param[in]\tbuffer_size Size of @c buffer in bytes. Must be equal to the size of the register property.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If \\c prop is not a register property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If @c buffer_size is not equal to the size of the property returned from #ic4_prop_register_get_size(), the function fails and the error value is set to #IC4_ERROR_INVALID_PARAM_VAL.\\n\n\n @see ic4_prop_register_get_size\n @see ic4_prop_register_set_value"]
    pub fn register_get_value(&mut self, register_size: usize) -> self::Result<Vec<u8>> {
        let mut data: Vec<u8> = std::iter::repeat(0u8).take(register_size).collect();
        unsafe {
            ic4_sys::ic4_prop_register_get_value(
                self.as_mut_ptr(),
                data.as_mut_ptr() as *mut c_void,
                register_size,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(data)
    }
    #[doc = " @brief Writes data to a register property.\n\n @param[in]\tprop\t\tA register property\n @param[in]\tbuffer\t\tA buffer containing the data to be written to the property\n @param[in]\tbuffer_size\tSize of @c buffer in bytes. Must be equal to the size of the register property.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If @c prop is not a register property, the function fails and the error value is set to #IC4_ERROR_GENICAM_TYPE_MISMATCH. \\n\n If @c buffer_size is not equal to the size of the property returned from #ic4_prop_register_get_size(), the function fails and the error value is set to #IC4_ERROR_INVALID_PARAM_VAL.\\n\n\n @see ic4_prop_register_get_size\n @see ic4_prop_register_get_value"]
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
    #[doc = " @brief Returns the number of properties in a property list.\n\n @param[in] list\tA property list\n @param[out] size\tPointer to a \\c size_t to receive the number of properties in \\c list.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn size(&mut self) -> self::Result<usize> {
        let mut value = Default::default();
        unsafe {
            ic4_sys::ic4_proplist_size(self.as_mut_ptr(), ptr_from_mut(&mut value))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(value)
    }
    #[doc = " @brief Returns a property from a property list.\n\n @param[in] list\t\t\tA property list\n @param[in] index\t\t\tIndex of the property to retrieve from \\c list\n @param[out]\tppProperty\tPointer to a handle receiving the property object.\\n\n\t\t\t\t\t\t\tWhen the property is no longer required, release the object reference using #ic4_prop_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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
