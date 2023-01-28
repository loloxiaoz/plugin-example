use regex::Regex;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
trait SecurityFormat {
    fn format(&self) -> String {
        String::new()
    }
}

impl Debug for dyn SecurityFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

macro_rules! create_struct {
    ($struct_name:ident; $next:expr) => {
        #[derive(Clone, Default, PartialEq, Hash)]
        pub struct $struct_name(pub String);
        impl SecurityFormat for $struct_name {
            fn format(&self) -> String {
                $next(&self)
            }
        }
        impl Debug for $struct_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.format())
            }
        }
    };
}

create_struct!(IpFormat; |_obj: &IpFormat|{
    "*.*.*.*".to_string()
});
create_struct!(EmailFormat; |obj: &EmailFormat|{
    // ^[a-zA-Z0-9]{1}(?P<replace_value>([a-zA-Z0-9]?[_|\_|\.]?)*)@([a-zA-Z0-9]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$
    let regex = if let Ok(val) = Regex::new(
        r#"(?P<prefix>^[a-zA-Z0-9]{1})(?P<replace_value>([a-zA-Z0-9]?[_|.]?)*)(?P<suffix>@([a-zA-Z0-9]+[_|.]?)*[a-zA-Z0-9]+.[a-zA-Z]{2,3}$)"#,
    ) {
        val
    } else {
        return obj.0.clone();
    };
    if regex.is_match(&obj.0) {
        regex
            .replace_all(&obj.0, "$prefix******$suffix")
            .to_string()
    } else {
        obj.0.clone()
    }
});
create_struct!(PhoneFormat; |obj: &PhoneFormat|{
    // (\d{3})(?P<replace_value>\d*)(\d{4})
    let regex = if let Ok(val) =
    Regex::new(r#"(?P<prefix>\d{3})(?P<replace_value>\d*)(?P<suffix>\d{4})"#)
    {
        val
    } else {
        return obj.0.clone();
    };
    if regex.is_match(&obj.0) {
        regex
            .replace_all(&obj.0, "$prefix*****$suffix")
            .to_string()
    } else {
        obj.0.clone()
    }
});
create_struct!(PasswordFormat; |_obj: &PasswordFormat|{
    "********".to_string()
});

create_struct!(IdCardFormat; |obj: &IdCardFormat|{
    // (.{6})(?P<replace_value>.*)(.{4})
    let regex = if let Ok(val) =
    Regex::new(r#"(?P<prefix>.{3})(?P<replace_value>.*)(?P<suffix>.{4})"#)
    {
        val
    } else {
        return obj.0.clone();
    };
    if regex.is_match(&obj.0) {
        regex
            .replace_all(&obj.0, "$prefix*****$suffix")
            .to_string()
    } else {
        obj.0.clone()
    }
});

create_struct!(NameFormat; |obj: &NameFormat|{
    // (.{1})(?P<replace_value>.*)
    let regex = if let Ok(val) = Regex::new(r#"(?P<prefix>.{1})(?P<replace_value>.*)"#) {
        val
    } else {
        return obj.0.clone();
    };
    if regex.is_match(&obj.0) {
        regex.replace_all(&obj.0, "$prefix*").to_string()
    } else {
        obj.0.clone()
    }
});

impl SecurityFormat for i32 {}

impl SecurityFormat for &str {}

impl SecurityFormat for String {}

macro_rules! serializer_Deserialize_struct {
    ($struct_name:ident) => {
        impl Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }
        impl<'de> Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct StructVisitor;
                impl<'de> Visitor<'de> for StructVisitor {
                    type Value = $struct_name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(stringify!($struct_name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok($struct_name(value.to_string()))
                    }
                }
                deserializer.deserialize_str(StructVisitor)
            }
        }
    };
}

serializer_Deserialize_struct!(PasswordFormat);
serializer_Deserialize_struct!(IpFormat);
serializer_Deserialize_struct!(NameFormat);
serializer_Deserialize_struct!(IdCardFormat);
serializer_Deserialize_struct!(EmailFormat);
serializer_Deserialize_struct!(PhoneFormat);

macro_rules! deref_struct {
    ($struct_name:ident) => {
        impl Deref for $struct_name {
            type Target = String;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

deref_struct!(IpFormat);
deref_struct!(NameFormat);
deref_struct!(PasswordFormat);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    pub struct TestStruct {
        ip: IpFormat,
        phone: PhoneFormat,
        password: PasswordFormat,
        name: NameFormat,
        idcard: IdCardFormat,
        email: EmailFormat,
    }

    pub fn get_test_obj() -> TestStruct {
        TestStruct {
            ip: IpFormat("10.12.12.45".to_string()),
            phone: PhoneFormat("12345678783".to_string()),
            password: PasswordFormat("testzichencehn".to_string()),
            name: NameFormat("zichenchen".to_string()),
            idcard: IdCardFormat("43048111922291123".to_string()),
            email: EmailFormat("zichenchen@qq.com".to_string()),
        }
    }

    #[test]
    fn test_ip() {
        let obj = get_test_obj();
        assert_eq!(format!("{:?}", &obj.ip), "*.*.*.*");
    }

    #[test]
    fn test_phone() {
        let obj = get_test_obj();
        assert_eq!(format!("{:?}", &obj.phone), "123*****8783");
    }

    #[test]
    fn test_password() {
        let obj = get_test_obj();
        assert_eq!(format!("{:?}", &obj.password), "********");
    }

    #[test]
    fn test_name() {
        let obj = get_test_obj();
        assert_eq!(format!("{:?}", &obj.name), "z*");
    }

    #[test]
    fn test_idcard() {
        let obj = get_test_obj();
        assert_eq!(format!("{:?}", &obj.idcard), "430*****1123");
    }

    #[test]
    fn test_email() {
        let obj = get_test_obj();
        assert_eq!(format!("{:?}", &obj.email), "z******@qq.com");
    }

    #[test]
    fn test_passwordformat_serializer() {
        let obj = PasswordFormat("zichenchen".to_string());
        assert_eq!(serde_json::to_value(obj).unwrap(), json!("zichenchen"));
    }

    #[test]
    fn test_passwordformat_derializer() {
        let obj = json!("zichenchen");
        assert_eq!(
            serde_json::from_value::<PasswordFormat>(obj).unwrap().0,
            "zichenchen"
        );
    }

    #[test]
    fn test_passwordformat_deref() {
        let obj = PasswordFormat("zichenchen".to_string());
        assert_eq!(obj.deref(), "zichenchen");
    }
}
