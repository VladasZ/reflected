mod test_enum;

use chrono::Duration;
use reflected::{Reflected, ToReflectedVal};
use rust_decimal::Decimal;
use sqlx::Type;

mod sercli {
    pub type Decimal = rust_decimal::Decimal;
    pub type DateTime = chrono::NaiveDateTime;
}

#[derive(strum::Display, strum::EnumString, Type, Copy, Clone, Default, PartialEq, Debug)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
enum SomeEnum {
    #[default]
    A,
    B,
}

impl ToReflectedVal<SomeEnum> for &str {
    fn to_reflected_val(&self) -> Result<SomeEnum, String> {
        use std::str::FromStr;
        Ok(SomeEnum::from_str(self).unwrap())
    }
}

#[derive(Reflected, Clone, Default, PartialEq, Debug)]
pub struct User {
    id:    usize,
    name:  String,
    email: String,

    birthday:             sercli::DateTime,
    age:                  usize,
    custom_id:            usize,
    cash:                 Decimal,
    sercli_cash:          sercli::Decimal,
    is_poros:             bool,
    height:               f64,
    dogs_count:           i16,
    enum_field:           SomeEnum,
    spent_eating_hotdogs: Duration,

    str_opt:     Option<String>,
    usize_opt:   Option<usize>,
    bool_opt:    Option<bool>,
    decimal_opt: Option<Decimal>,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::{Duration, NaiveDateTime, TimeDelta, Utc};
    use fake::{Fake, faker::internet::en::SafeEmail};
    use reflected::{Reflected, ReflectedEq};
    use rust_decimal::Decimal;

    use crate::{User, sercli};

    #[test]
    fn convert_date() {
        let date = Utc::now().naive_utc();

        dbg!(&date);

        let date_string = date.to_string();

        dbg!(&date_string);

        let parsed_date = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S%.9f");

        dbg!(&parsed_date);
    }

    #[test]
    fn fields() {
        assert!(User::ID.is_id());
        assert!(User::CUSTOM_ID.is_foreign_id());
        assert!(User::BIRTHDAY.is_date());
        assert!(User::CASH.is_decimal());
        assert!(User::SERCLI_CASH.is_decimal());
        assert!(User::IS_POROS.is_bool());
        assert!(User::HEIGHT.is_float());
        assert!(User::DOGS_COUNT.is_integer());
        assert!(User::SPENT_EATING_HOTDOGS.is_duration());

        assert!(User::STR_OPT.is_optional());
        assert!(User::STR_OPT.is_text());

        assert!(User::USIZE_OPT.is_optional());
        assert!(User::USIZE_OPT.is_integer());

        assert!(User::BOOL_OPT.is_optional());
        assert!(User::BOOL_OPT.is_bool());

        assert!(User::DECIMAL_OPT.is_optional());
        assert!(User::DECIMAL_OPT.is_decimal());

        assert_eq!(User::fields().len(), 17);
    }

    #[test]
    fn types() {
        assert_eq!(User::ID.type_name, "usize");
        assert_eq!(User::BIRTHDAY.type_name, "DateTime");
        assert_eq!(User::CASH.type_name, "Decimal");
        assert_eq!(User::IS_POROS.type_name, "bool");
        assert_eq!(User::HEIGHT.type_name, "f64");
        assert_eq!(User::DOGS_COUNT.type_name, "i16");
        assert_eq!(User::SPENT_EATING_HOTDOGS.type_name, "Duration");
        assert_eq!(User::STR_OPT.type_name, "String");
        assert_eq!(User::USIZE_OPT.type_name, "usize");
        assert_eq!(User::BOOL_OPT.type_name, "bool");
        assert_eq!(User::DECIMAL_OPT.type_name, "Decimal");
    }

    #[test]
    fn get() {
        let birthday = Utc::now().naive_utc();

        let mut user = User {
            id: 0,
            name: "peter".into(),
            email: SafeEmail().fake(),
            birthday,
            age: 15,
            custom_id: 0,
            cash: Decimal::from_str("100.25").unwrap(),
            sercli_cash: Decimal::from_str("25.45").unwrap(),
            is_poros: false,
            height: 6.45,
            dogs_count: 5,
            enum_field: Default::default(),
            spent_eating_hotdogs: Duration::new(200, 0).unwrap(),
            str_opt: None,
            usize_opt: None,
            bool_opt: None,
            decimal_opt: None,
        };

        assert_eq!(user.get_value(User::NAME), "peter".to_string());
        assert_eq!(user.get_value(User::AGE), "15".to_string());
        assert_eq!(user.get_value(User::BIRTHDAY), birthday.to_string());
        assert_eq!(user.get_value(User::CASH), "100.25".to_string());
        assert_eq!(user.get_value(User::IS_POROS), "0".to_string());
        assert_eq!(user.get_value(User::HEIGHT), "6.45".to_string());
        assert_eq!(user.get_value(User::DOGS_COUNT), "5".to_string());
        assert_eq!(user.get_value(User::SPENT_EATING_HOTDOGS), "200".to_string());

        assert_eq!(user.get_value(User::STR_OPT), "NULL".to_string());
        assert_eq!(user.get_value(User::USIZE_OPT), "NULL".to_string());
        assert_eq!(user.get_value(User::BOOL_OPT), "NULL".to_string());
        assert_eq!(user.get_value(User::DECIMAL_OPT), "NULL".to_string());

        user.str_opt = Some("stre".to_string());
        user.usize_opt = Some(222);
        user.bool_opt = Some(false);
        user.decimal_opt = Some(Decimal::from_str("100.25").unwrap());

        assert_eq!(user.get_value(User::STR_OPT), "stre".to_string());
        assert_eq!(user.get_value(User::USIZE_OPT), "222".to_string());
        assert_eq!(user.get_value(User::BOOL_OPT), "0".to_string());
        assert_eq!(user.get_value(User::DECIMAL_OPT), "100.25".to_string());
    }

    #[test]
    fn set() {
        let mut user = User {
            id:                   0,
            name:                 "peter".into(),
            email:                "".to_string(),
            birthday:             Default::default(),
            age:                  15,
            custom_id:            0,
            cash:                 Default::default(),
            sercli_cash:          Default::default(),
            is_poros:             false,
            height:               6.45,
            dogs_count:           5,
            enum_field:           Default::default(),
            spent_eating_hotdogs: Duration::new(200, 0).unwrap(),
            str_opt:              None,
            usize_opt:            None,
            bool_opt:             None,
            decimal_opt:          None,
        };

        let new_bd = Utc::now().naive_utc();

        user.set_value(User::NAME, "parker".into());
        user.set_value(User::AGE, "19".into());
        user.set_value(User::BIRTHDAY, Some(&new_bd.to_string()));
        user.set_value(User::CASH, "100.71".into());
        user.set_value(User::SERCLI_CASH, "33.23".into());
        user.set_value(User::SPENT_EATING_HOTDOGS, "555".into());
        user.set_value(User::IS_POROS, "1".into());
        user.set_value(User::HEIGHT, "5.467".into());
        user.set_value(User::DOGS_COUNT, "17".into());

        assert_eq!(user.get_value(User::NAME), "parker".to_string());
        assert_eq!(user.get_value(User::AGE), "19".to_string());
        assert_eq!(user.get_value(User::BIRTHDAY), new_bd.to_string());
        assert_eq!(user.get_value(User::CASH), "100.71".to_string());
        assert_eq!(user.get_value(User::SERCLI_CASH), "33.23".to_string());
        assert_eq!(user.get_value(User::SPENT_EATING_HOTDOGS), "555".to_string());
        assert_eq!(user.get_value(User::IS_POROS), "1".to_string());
        assert_eq!(user.get_value(User::HEIGHT), "5.467".to_string());
        assert_eq!(user.get_value(User::DOGS_COUNT), "17".to_string());

        user.set_value(User::STR_OPT, "sokol".into());
        user.set_value(User::USIZE_OPT, "555".into());
        user.set_value(User::BOOL_OPT, "1".into());
        user.set_value(User::DECIMAL_OPT, "100.71".into());

        assert_eq!(user.get_value(User::STR_OPT), "sokol".to_string());
        assert_eq!(user.get_value(User::USIZE_OPT), "555".to_string());
        assert_eq!(user.get_value(User::BOOL_OPT), "1".to_string());
        assert_eq!(user.get_value(User::DECIMAL_OPT), "100.71".to_string());

        user.set_value(User::STR_OPT, None);
        user.set_value(User::USIZE_OPT, None);
        user.set_value(User::BOOL_OPT, None);
        user.set_value(User::DECIMAL_OPT, None);

        assert_eq!(user.get_value(User::STR_OPT), "NULL".to_string());
        assert_eq!(user.get_value(User::USIZE_OPT), "NULL".to_string());
        assert_eq!(user.get_value(User::BOOL_OPT), "NULL".to_string());
        assert_eq!(user.get_value(User::DECIMAL_OPT), "NULL".to_string());

        assert_eq!(
            user,
            User {
                id:                   0,
                name:                 "parker".into(),
                email:                "".to_string(),
                birthday:             new_bd,
                age:                  19,
                custom_id:            0,
                cash:                 Decimal::from_str("100.71").unwrap(),
                sercli_cash:          Decimal::from_str("33.23").unwrap(),
                is_poros:             true,
                height:               5.467,
                dogs_count:           17,
                enum_field:           Default::default(),
                spent_eating_hotdogs: Duration::new(555, 0).unwrap(),
                str_opt:              None,
                usize_opt:            None,
                bool_opt:             None,
                decimal_opt:          None,
            }
        );
    }

    #[test]
    fn random() {
        let _user = User::random();
        dbg!(_user);
    }

    #[test]
    fn reflected_eq() {
        #[derive(Default, Reflected, Clone)]
        struct Test {
            id:   usize,
            name: String,

            birthday:  sercli::DateTime,
            age:       usize,
            custom_id: usize,
            cash:      Decimal,
            is_poros:  bool,
            height:    f64,
        }

        let user_1 = Test::random();
        let mut user_2 = user_1.clone();

        user_1.assert_eq(&user_2);

        user_2.height += 0.0001;

        user_1.assert_eq(&user_2);
    }

    #[test]
    fn get_float() {
        #[derive(Default, Reflected)]
        struct Data {
            float32: f32,
            float64: f64,
        }

        let mut data = Data {
            float32: 5.0,
            float64: 1.0,
        };

        assert_eq!(data.get_value(Data::FLOAT32), "5.0");
        assert_eq!(data.get_value(Data::FLOAT64), "1.0");

        data.float32 = 0.42332;
        data.float64 = 0.438297489;

        assert_eq!(data.get_value(Data::FLOAT32), "0.42332");
        assert_eq!(data.get_value(Data::FLOAT64), "0.438297489");
    }

    #[test]
    fn test_duration() {
        let _5_min = Duration::from(TimeDelta::minutes(5) + TimeDelta::seconds(25));

        assert_eq!(
            format!(
                "{}:{}",
                _5_min.num_minutes(),
                _5_min.num_seconds() - _5_min.num_minutes() * 60
            ),
            "5:25"
        );
    }
}
