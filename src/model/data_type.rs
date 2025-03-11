// Copyright 2024 meduo <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lombok::{Builder, Getter, GetterMut, Setter};

#[derive(Debug,Default, Clone)]
pub enum DataType {
    #[default]
    String,
    Integer,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
}

impl TryFrom<String> for DataType {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();
        //remove xsd prefix if exists
        if value.starts_with("xsd:") {
            return Self::try_from(value.to_string());
        }

        match value {
            "string"
            | "hexBinary"
            | "base64Binary"
            | "anyURI"
            | "language"
            | "normalizedString"
            | "token"
            | "NMTOKEN"
             => Ok(Self::String),
            "integer"
            | "int"
            | "decimal"
            | "gYear"
            | "gMonth"
            | "gDay"
            | "gYearMonth"
            | "gMonthDay"
            | "duration"
            | "yearMonthDuration"
            | "dayTimeDuration"
            | "byte"
            | "short"
            | "long"
            | "unsignedByte"
            | "unsignedShort"
            | "unsignedInt"
            | "unsignedLong"
            | "positiveInteger"
            | "nonNegativeInteger"
            => Ok(Self::Integer),
            "float"
            | "double"
            => Ok(Self::Float),
            "boolean" => Ok(Self::Boolean),
            "date" => Ok(Self::Date),
            "time"
            | "dateTime"
            | "dateTimeStamp"
            => Ok(Self::Time),
            _ => Err(anyhow::anyhow!("Unsupported data type")),
        }
    }
}
