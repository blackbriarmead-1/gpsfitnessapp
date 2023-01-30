pub mod my_date_format {
    use mongodb::bson::DateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.try_to_rfc3339_string().expect("");
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        mongodb::bson::DateTime::parse_rfc3339_str(&s).map_err(serde::de::Error::custom)
    }
}
