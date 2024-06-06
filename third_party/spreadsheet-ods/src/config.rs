/// The configuration format is a convoluted tree of typed key/value pairs.
/// With some complications.
///
/// This is only used internally and is mapped to WorkBookConfig and
/// SheetConfig which are more accessible.
///
use crate::HashMap;
use get_size::GetSize;

use chrono::NaiveDateTime;
use get_size_derive::GetSize;

/// The possible value types for the configuration.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub(crate) enum ConfigValue {
    Base64Binary(String),
    Boolean(bool),
    DateTime(NaiveDateTime),
    Double(f64),
    Int(i32),
    Long(i64),
    Short(i16),
    String(String),
}

impl GetSize for ConfigValue {
    fn get_heap_size(&self) -> usize {
        match self {
            ConfigValue::Base64Binary(v) => v.get_heap_size(),
            ConfigValue::Boolean(_) => 0,
            ConfigValue::DateTime(_) => 0,
            ConfigValue::Double(_) => 0,
            ConfigValue::Int(_) => 0,
            ConfigValue::Long(_) => 0,
            ConfigValue::Short(_) => 0,
            ConfigValue::String(v) => v.get_heap_size(),
        }
    }
}

impl ConfigValue {}

impl From<&str> for ConfigValue {
    fn from(v: &str) -> Self {
        ConfigValue::String(v.into())
    }
}

impl From<String> for ConfigValue {
    fn from(v: String) -> Self {
        ConfigValue::String(v)
    }
}

impl From<bool> for ConfigValue {
    fn from(v: bool) -> Self {
        ConfigValue::Boolean(v)
    }
}

impl From<NaiveDateTime> for ConfigValue {
    fn from(v: NaiveDateTime) -> Self {
        ConfigValue::DateTime(v)
    }
}

impl From<f64> for ConfigValue {
    fn from(v: f64) -> Self {
        ConfigValue::Double(v)
    }
}

impl From<i16> for ConfigValue {
    fn from(v: i16) -> Self {
        ConfigValue::Short(v)
    }
}

impl From<i32> for ConfigValue {
    fn from(v: i32) -> Self {
        ConfigValue::Int(v)
    }
}

impl From<u32> for ConfigValue {
    fn from(v: u32) -> Self {
        ConfigValue::Int(v as i32)
    }
}

impl From<i64> for ConfigValue {
    fn from(v: i64) -> Self {
        ConfigValue::Long(v)
    }
}

/// Configuration mappings.
///
/// It behaves like a map, but the insertion order is retained.
#[derive(Debug, Clone, PartialEq, GetSize)]
pub(crate) struct ConfigMap {
    key_index: HashMap<String, usize>,
    values: Vec<(String, ConfigItem)>,
}

impl ConfigMap {
    pub(crate) fn new() -> Self {
        Self {
            key_index: Default::default(),
            values: Default::default(),
        }
    }

    /// Iterate over this map.
    pub(crate) fn iter(&self) -> ConfigIter<'_> {
        ConfigIter {
            it: Some(self.values.iter()),
        }
    }

    /// Adds a new ConfigItem
    pub(crate) fn insert<S, V>(&mut self, name: S, item: V)
    where
        S: AsRef<str>,
        V: Into<ConfigItem>,
    {
        let idx = self.key_index.get(name.as_ref());

        if let Some(idx) = idx {
            if let Some(v) = self.values.get_mut(*idx) {
                v.1 = item.into();
            } else {
                unreachable!();
            }
        } else {
            self.values.push((name.as_ref().to_string(), item.into()));
            self.key_index
                .insert(name.as_ref().to_string(), self.values.len() - 1);
        }
    }

    /// Returns a ConfigItem
    pub(crate) fn get<S>(&self, name: S) -> Option<&ConfigItem>
    where
        S: AsRef<str>,
    {
        let idx = self.key_index.get(name.as_ref());

        if let Some(idx) = idx {
            self.values.get(*idx).map(|v| &v.1)
        } else {
            None
        }
    }

    /// Returns a ConfigItem or creates it.
    pub(crate) fn get_or_create<S, F>(&mut self, name: S, default: F) -> &mut ConfigItem
    where
        S: AsRef<str>,
        F: Fn() -> ConfigItem,
    {
        let idx = self.key_index.get(name.as_ref());

        let idx = if let Some(idx) = idx {
            *idx
        } else {
            self.values.push((name.as_ref().to_string(), default()));
            self.key_index
                .insert(name.as_ref().to_string(), self.values.len() - 1);

            self.values.len() - 1
        };

        &mut self.values.get_mut(idx).unwrap().1
    }
}

pub(crate) struct ConfigIter<'a> {
    it: Option<core::slice::Iter<'a, (String, ConfigItem)>>,
}

impl<'a> Iterator for ConfigIter<'a> {
    type Item = (&'a String, &'a ConfigItem);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(it) = &mut self.it {
            it.next().map(|v| (&v.0, &v.1))
        } else {
            None
        }
    }
}

/// Bare enumeration for the different classes of ConfigItems.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ConfigItemType {
    Value,
    Set,
    Vec,
    Map,
    Entry,
}

impl From<&ConfigItem> for ConfigItemType {
    fn from(item: &ConfigItem) -> Self {
        match item {
            ConfigItem::Value(_) => ConfigItemType::Value,
            ConfigItem::Set(_) => ConfigItemType::Set,
            ConfigItem::Vec(_) => ConfigItemType::Vec,
            ConfigItem::Map(_) => ConfigItemType::Map,
            ConfigItem::Entry(_) => ConfigItemType::Entry,
        }
    }
}

impl From<&mut ConfigItem> for ConfigItemType {
    fn from(item: &mut ConfigItem) -> Self {
        match item {
            ConfigItem::Value(_) => ConfigItemType::Value,
            ConfigItem::Set(_) => ConfigItemType::Set,
            ConfigItem::Vec(_) => ConfigItemType::Vec,
            ConfigItem::Map(_) => ConfigItemType::Map,
            ConfigItem::Entry(_) => ConfigItemType::Entry,
        }
    }
}

impl PartialEq<ConfigItem> for ConfigItemType {
    fn eq(&self, other: &ConfigItem) -> bool {
        other == self
    }
}

impl PartialEq<ConfigItemType> for ConfigItem {
    fn eq(&self, other: &ConfigItemType) -> bool {
        match self {
            ConfigItem::Value(_) => matches!(other, ConfigItemType::Value),
            ConfigItem::Set(_) => matches!(other, ConfigItemType::Set),
            ConfigItem::Vec(_) => matches!(other, ConfigItemType::Vec),
            ConfigItem::Map(_) => matches!(other, ConfigItemType::Map),
            ConfigItem::Entry(_) => matches!(other, ConfigItemType::Entry),
        }
    }
}

/// Unifies values and sets of values. The branch structure of the tree.
#[derive(Debug, Clone, PartialEq, GetSize)]
pub(crate) enum ConfigItem {
    Value(ConfigValue),
    Set(ConfigMap),
    Vec(ConfigMap),
    Map(ConfigMap),
    Entry(ConfigMap),
}

/// Nice conversion for everything that can be converted to a ConfigValue
/// can directly be converted to a ConfigItem too.
impl<T> From<T> for ConfigItem
where
    ConfigValue: From<T>,
{
    fn from(v: T) -> Self {
        ConfigItem::Value(ConfigValue::from(v))
    }
}

impl Default for ConfigItem {
    fn default() -> Self {
        ConfigItem::new_set()
    }
}

impl ConfigItem {
    /// New ConfigItem.
    ///
    /// Panics
    /// This doesn't work for ConfigItemType::Value.
    pub(crate) fn new(itype: ConfigItemType) -> Self {
        match itype {
            ConfigItemType::Value => panic!("new with type works only for map-types"),
            ConfigItemType::Set => ConfigItem::Set(ConfigMap::new()),
            ConfigItemType::Vec => ConfigItem::Vec(ConfigMap::new()),
            ConfigItemType::Map => ConfigItem::Map(ConfigMap::new()),
            ConfigItemType::Entry => ConfigItem::Entry(ConfigMap::new()),
        }
    }

    /// New set.
    pub(crate) fn new_set() -> Self {
        Self::Set(ConfigMap::new())
    }

    /// New vec.
    pub(crate) fn new_vec() -> Self {
        Self::Vec(ConfigMap::new())
    }

    /// New map.
    pub(crate) fn new_map() -> Self {
        Self::Map(ConfigMap::new())
    }

    /// New map entry oder vec entry.
    pub(crate) fn new_entry() -> Self {
        Self::Entry(ConfigMap::new())
    }

    /// Returns the contained ConfigValue if any.
    fn as_value(&self) -> Option<&ConfigValue> {
        match self {
            ConfigItem::Value(v) => Some(v),
            ConfigItem::Set(_) => None,
            ConfigItem::Vec(_) => None,
            ConfigItem::Map(_) => None,
            ConfigItem::Entry(_) => None,
        }
    }

    /// Is this any map-like ConfigItem.
    fn is_map(&self) -> bool {
        match self {
            ConfigItem::Value(_) => false,
            ConfigItem::Set(_) => true,
            ConfigItem::Vec(_) => true,
            ConfigItem::Map(_) => true,
            ConfigItem::Entry(_) => true,
        }
    }

    /// Returns the contained ConfigMap if this is a map-like ConfigItem.
    fn as_map(&self) -> Option<&ConfigMap> {
        match self {
            ConfigItem::Value(_) => None,
            ConfigItem::Set(m) => Some(m),
            ConfigItem::Vec(m) => Some(m),
            ConfigItem::Map(m) => Some(m),
            ConfigItem::Entry(m) => Some(m),
        }
    }

    /// Returns the contained ConfigMap if this is a map-like ConfigItem.
    fn as_map_mut(&mut self) -> Option<&mut ConfigMap> {
        match self {
            ConfigItem::Value(_) => None,
            ConfigItem::Set(m) => Some(m),
            ConfigItem::Vec(m) => Some(m),
            ConfigItem::Map(m) => Some(m),
            ConfigItem::Entry(m) => Some(m),
        }
    }

    /// Iterate over (k,v) pairs.
    pub(crate) fn iter(&self) -> ConfigIter<'_> {
        if let Some(m) = self.as_map() {
            m.iter()
        } else {
            ConfigIter { it: None }
        }
    }

    /// Adds a new ConfigItem into this map.
    ///
    /// Panics
    /// If this is not a map-like ConfigItem.
    pub(crate) fn insert<S, V>(&mut self, name: S, item: V)
    where
        S: AsRef<str>,
        V: Into<ConfigItem>,
    {
        if let Some(m) = self.as_map_mut() {
            m.insert(name, item.into());
        } else {
            panic!();
        }
    }

    /// Returns a ConfigItem.
    ///
    /// Panics
    /// If this is not a map-like ConfigItem.
    pub(crate) fn get<S>(&self, name: S) -> Option<&ConfigItem>
    where
        S: AsRef<str>,
    {
        if let Some(m) = self.as_map() {
            m.get(name.as_ref())
        } else {
            panic!()
        }
    }

    /// Recursively creates all maps along the given path and
    /// returns the last map-like ConfigItem.
    ///
    /// Panics
    /// If the given map-types along the path don't match with what
    /// exists in the structure.
    /// If the last element in the path is a ConfigValue.
    pub(crate) fn create_path<S>(&mut self, names: &[(S, ConfigItemType)]) -> &mut ConfigItem
    where
        S: AsRef<str>,
    {
        if self.is_map() {
            // some name parts left?
            if let Some(((name, itype), rest)) = names.split_first() {
                // create if non existent
                let item = self
                    .as_map_mut()
                    .expect("is_map() and as_map_mut() should agree")
                    .get_or_create(name, || ConfigItem::new(*itype));

                if !(item == itype) {
                    // close, but not good enough
                    panic!(
                        "types don't match {:?} <> {:?}",
                        ConfigItemType::from(item),
                        itype
                    );
                } else {
                    // recurse
                    item.create_path(rest)
                }
            } else {
                // last path element is what we want
                self
            }
        } else {
            // not a map
            panic!("path ends in a value");
        }
    }

    /// Recursive get for any ConfigItem.
    pub(crate) fn get_rec<S>(&self, names: &[S]) -> Option<&ConfigItem>
    where
        S: AsRef<str>,
    {
        if let Some(map) = self.as_map() {
            if let Some((name, rest)) = names.split_first() {
                if let Some(item) = map.get(name.as_ref()) {
                    item.get_rec(rest)
                } else {
                    None
                }
            } else {
                Some(self)
            }
        } else {
            // no deeper nesting, ok
            if names.is_empty() {
                Some(self)
            } else {
                None
            }
        }
    }

    /// Recursive get for only the ConfigValue leaves.
    pub(crate) fn get_value_rec<S>(&self, names: &[S]) -> Option<&ConfigValue>
    where
        S: AsRef<str>,
    {
        if let Some(map) = self.as_map() {
            if let Some((name, rest)) = names.split_first() {
                if let Some(item) = map.get(name.as_ref()) {
                    item.get_value_rec(rest)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // no deeper nesting, ok
            if names.is_empty() {
                self.as_value()
            } else {
                None
            }
        }
    }
}

/// Basic wrapper around a ConfigSet. Root of the config tree.
#[derive(Debug, Clone, GetSize)]
pub(crate) struct Config {
    config: ConfigItem,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    /// Iterate over the (k,v) pairs.
    pub(crate) fn iter(&self) -> ConfigIter<'_> {
        self.config.iter()
    }

    /// Add an item.
    pub(crate) fn insert<S, V>(&mut self, name: S, item: V)
    where
        S: AsRef<str>,
        V: Into<ConfigItem>,
    {
        self.config.insert(name, item.into());
    }

    /// Recursive get.
    pub(crate) fn get<S>(&self, names: &[S]) -> Option<&ConfigItem>
    where
        S: AsRef<str>,
    {
        self.config.get_rec(names)
    }

    /// Recursive get, only for ConfigValue leaves.
    pub(crate) fn get_value<S>(&self, names: &[S]) -> Option<&ConfigValue>
    where
        S: AsRef<str>,
    {
        self.config.get_value_rec(names)
    }

    pub(crate) fn create_path<S>(&mut self, names: &[(S, ConfigItemType)]) -> &mut ConfigItem
    where
        S: AsRef<str>,
    {
        self.config.create_path(names)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, ConfigItem, ConfigItemType, ConfigMap, ConfigValue};

    fn setup_config() -> Config {
        let mut config = Config::new();
        {
            let mut view_settings = ConfigItem::new_set();
            view_settings.insert("VisibleAreaTop", 903);
            config.insert("ooo:view-settings", view_settings);
        }
        {
            let mut configuration_settings = ConfigItem::new_set();
            configuration_settings.insert("HasSheetTabs".to_string(), true);
            configuration_settings.insert("ShowNotes", true);
            configuration_settings.insert("GridColor", 12632256);
            configuration_settings.insert("LinkUpdateMode", 3i16);
            configuration_settings.insert(
                "PrinterSetup",
                ConfigValue::Base64Binary("unknown_garbage".to_string()),
            );
            {
                let mut script_configuration = ConfigItem::new_map();
                {
                    let mut tabelle1 = ConfigItem::new_entry();
                    tabelle1.insert("CodeName", "Tabelle1");
                    script_configuration.insert("Tabelle1", tabelle1);
                }
                configuration_settings.insert("ScriptConfiguration", script_configuration);
            }
            config.insert("ooo:configuration-settings", configuration_settings);
        }

        config
    }

    #[test]
    fn test_config() {
        let mut config = setup_config();

        assert_eq!(config.get_value(&["ooo:view-settings", "ShowNotes"]), None);
        assert_eq!(config.get_value(&["ooo:view-settings", "ShowNotes"]), None);
        assert_eq!(
            config.get_value(&["ooo:view-settings", "VisibleAreaTop"]),
            Some(&ConfigValue::Int(903))
        );
        assert_eq!(
            config.get_value(&["ooo:configuration-settings", "ShowNotes"]),
            Some(&ConfigValue::Boolean(true))
        );
        assert_eq!(
            config.get_value(&[
                "ooo:configuration-settings",
                "ScriptConfiguration",
                "Tabelle1",
                "CodeName"
            ]),
            Some(&ConfigValue::String("Tabelle1".to_string()))
        );

        let v = config.create_path(&[
            ("ooo:configuration-settings", ConfigItemType::Set),
            ("ScriptConfiguration", ConfigItemType::Map),
            ("Tabelle2", ConfigItemType::Entry),
        ]);
        assert_eq!(v, &ConfigItem::Entry(ConfigMap::new()));
    }

    #[test]
    #[should_panic]
    fn test_create_path() {
        let mut config = setup_config();
        let _v = config.create_path(&[
            ("ooo:configuration-settings", ConfigItemType::Map), // here
            ("ScriptConfiguration", ConfigItemType::Map),
            ("Tabelle2", ConfigItemType::Entry),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_create_path2() {
        let mut config = setup_config();
        let _v = config.create_path(&[
            ("ooo:configuration-settings", ConfigItemType::Set),
            ("ScriptConfiguration", ConfigItemType::Map),
            ("Tabelle1", ConfigItemType::Entry),
            ("CodeName", ConfigItemType::Value), // here
        ]);
    }
}
