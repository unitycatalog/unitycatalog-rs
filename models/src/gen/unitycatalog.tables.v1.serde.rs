// @generated
impl serde::Serialize for ColumnInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.type_text.is_empty() {
            len += 1;
        }
        if !self.type_json.is_empty() {
            len += 1;
        }
        if self.type_name != 0 {
            len += 1;
        }
        if self.type_precision != 0 {
            len += 1;
        }
        if self.type_scale != 0 {
            len += 1;
        }
        if !self.type_interval_type.is_empty() {
            len += 1;
        }
        if self.position != 0 {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.nullable.is_some() {
            len += 1;
        }
        if self.partition_index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.ColumnInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.type_text.is_empty() {
            struct_ser.serialize_field("typeText", &self.type_text)?;
        }
        if !self.type_json.is_empty() {
            struct_ser.serialize_field("typeJson", &self.type_json)?;
        }
        if self.type_name != 0 {
            let v = ColumnTypeName::try_from(self.type_name)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.type_name)))?;
            struct_ser.serialize_field("typeName", &v)?;
        }
        if self.type_precision != 0 {
            struct_ser.serialize_field("typePrecision", &self.type_precision)?;
        }
        if self.type_scale != 0 {
            struct_ser.serialize_field("typeScale", &self.type_scale)?;
        }
        if !self.type_interval_type.is_empty() {
            struct_ser.serialize_field("typeIntervalType", &self.type_interval_type)?;
        }
        if self.position != 0 {
            struct_ser.serialize_field("position", &self.position)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.nullable.as_ref() {
            struct_ser.serialize_field("nullable", v)?;
        }
        if self.partition_index != 0 {
            struct_ser.serialize_field("partitionIndex", &self.partition_index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type_text",
            "typeText",
            "type_json",
            "typeJson",
            "type_name",
            "typeName",
            "type_precision",
            "typePrecision",
            "type_scale",
            "typeScale",
            "type_interval_type",
            "typeIntervalType",
            "position",
            "comment",
            "nullable",
            "partition_index",
            "partitionIndex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypeText,
            TypeJson,
            TypeName,
            TypePrecision,
            TypeScale,
            TypeIntervalType,
            Position,
            Comment,
            Nullable,
            PartitionIndex,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "typeText" | "type_text" => Ok(GeneratedField::TypeText),
                            "typeJson" | "type_json" => Ok(GeneratedField::TypeJson),
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            "typePrecision" | "type_precision" => Ok(GeneratedField::TypePrecision),
                            "typeScale" | "type_scale" => Ok(GeneratedField::TypeScale),
                            "typeIntervalType" | "type_interval_type" => Ok(GeneratedField::TypeIntervalType),
                            "position" => Ok(GeneratedField::Position),
                            "comment" => Ok(GeneratedField::Comment),
                            "nullable" => Ok(GeneratedField::Nullable),
                            "partitionIndex" | "partition_index" => Ok(GeneratedField::PartitionIndex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.ColumnInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ColumnInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut type_text__ = None;
                let mut type_json__ = None;
                let mut type_name__ = None;
                let mut type_precision__ = None;
                let mut type_scale__ = None;
                let mut type_interval_type__ = None;
                let mut position__ = None;
                let mut comment__ = None;
                let mut nullable__ = None;
                let mut partition_index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeText => {
                            if type_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeText"));
                            }
                            type_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeJson => {
                            if type_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeJson"));
                            }
                            type_json__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = Some(map_.next_value::<ColumnTypeName>()? as i32);
                        }
                        GeneratedField::TypePrecision => {
                            if type_precision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typePrecision"));
                            }
                            type_precision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TypeScale => {
                            if type_scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeScale"));
                            }
                            type_scale__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TypeIntervalType => {
                            if type_interval_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeIntervalType"));
                            }
                            type_interval_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Nullable => {
                            if nullable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullable"));
                            }
                            nullable__ = map_.next_value()?;
                        }
                        GeneratedField::PartitionIndex => {
                            if partition_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionIndex"));
                            }
                            partition_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ColumnInfo {
                    name: name__.unwrap_or_default(),
                    type_text: type_text__.unwrap_or_default(),
                    type_json: type_json__.unwrap_or_default(),
                    type_name: type_name__.unwrap_or_default(),
                    type_precision: type_precision__.unwrap_or_default(),
                    type_scale: type_scale__.unwrap_or_default(),
                    type_interval_type: type_interval_type__.unwrap_or_default(),
                    position: position__.unwrap_or_default(),
                    comment: comment__,
                    nullable: nullable__,
                    partition_index: partition_index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.ColumnInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnTypeName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ColumnTypeUnspecified => "COLUMN_TYPE_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ColumnTypeName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COLUMN_TYPE_UNSPECIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnTypeName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COLUMN_TYPE_UNSPECIFIED" => Ok(ColumnTypeName::ColumnTypeUnspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTableRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if !self.schema_name.is_empty() {
            len += 1;
        }
        if self.table_type != 0 {
            len += 1;
        }
        if self.data_source_format != 0 {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if !self.storage_location.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if !self.properties.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.CreateTableRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if self.table_type != 0 {
            let v = TableType::try_from(self.table_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.table_type)))?;
            struct_ser.serialize_field("tableType", &v)?;
        }
        if self.data_source_format != 0 {
            let v = DataSourceFormat::try_from(self.data_source_format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_source_format)))?;
            struct_ser.serialize_field("dataSourceFormat", &v)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if !self.storage_location.is_empty() {
            struct_ser.serialize_field("storageLocation", &self.storage_location)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if !self.properties.is_empty() {
            struct_ser.serialize_field("properties", &self.properties)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "catalog_name",
            "catalogName",
            "schema_name",
            "schemaName",
            "table_type",
            "tableType",
            "data_source_format",
            "dataSourceFormat",
            "columns",
            "storage_location",
            "storageLocation",
            "comment",
            "properties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            SchemaName,
            TableType,
            DataSourceFormat,
            Columns,
            StorageLocation,
            Comment,
            Properties,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "schemaName" | "schema_name" => Ok(GeneratedField::SchemaName),
                            "tableType" | "table_type" => Ok(GeneratedField::TableType),
                            "dataSourceFormat" | "data_source_format" => Ok(GeneratedField::DataSourceFormat),
                            "columns" => Ok(GeneratedField::Columns),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.CreateTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut schema_name__ = None;
                let mut table_type__ = None;
                let mut data_source_format__ = None;
                let mut columns__ = None;
                let mut storage_location__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaName => {
                            if schema_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaName"));
                            }
                            schema_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableType => {
                            if table_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableType"));
                            }
                            table_type__ = Some(map_.next_value::<TableType>()? as i32);
                        }
                        GeneratedField::DataSourceFormat => {
                            if data_source_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataSourceFormat"));
                            }
                            data_source_format__ = Some(map_.next_value::<DataSourceFormat>()? as i32);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageLocation => {
                            if storage_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocation"));
                            }
                            storage_location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(CreateTableRequest {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    table_type: table_type__.unwrap_or_default(),
                    data_source_format: data_source_format__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                    storage_location: storage_location__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.CreateTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTableResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.CreateTableResponse", len)?;
        if let Some(v) = self.table.as_ref() {
            struct_ser.serialize_field("table", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTableResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Table,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "table" => Ok(GeneratedField::Table),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTableResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.CreateTableResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTableResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTableResponse {
                    table: table__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.CreateTableResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataSourceFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DATA_SOURCE_FORMAT_UNSPECIFIED",
            Self::Delta => "DATA_SOURCE_FORMAT_DELTA",
            Self::Parquet => "DATA_SOURCE_FORMAT_PARQUET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DataSourceFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DATA_SOURCE_FORMAT_UNSPECIFIED",
            "DATA_SOURCE_FORMAT_DELTA",
            "DATA_SOURCE_FORMAT_PARQUET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataSourceFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DATA_SOURCE_FORMAT_UNSPECIFIED" => Ok(DataSourceFormat::Unspecified),
                    "DATA_SOURCE_FORMAT_DELTA" => Ok(DataSourceFormat::Delta),
                    "DATA_SOURCE_FORMAT_PARQUET" => Ok(DataSourceFormat::Parquet),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTableRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.full_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.DeleteTableRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.DeleteTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteTableRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.DeleteTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTableResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.DeleteTableResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTableResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTableResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.DeleteTableResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTableResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteTableResponse {
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.DeleteTableResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.full_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.GetTableRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.GetTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetTableRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.GetTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.GetTableResponse", len)?;
        if let Some(v) = self.table.as_ref() {
            struct_ser.serialize_field("table", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Table,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "table" => Ok(GeneratedField::Table),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.GetTableResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTableResponse {
                    table: table__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.GetTableResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTablesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if !self.schema_name.is_empty() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.ListTablesRequest", len)?;
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTablesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "catalog_name",
            "catalogName",
            "schema_name",
            "schemaName",
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CatalogName,
            SchemaName,
            MaxResults,
            PageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "schemaName" | "schema_name" => Ok(GeneratedField::SchemaName),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.ListTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut catalog_name__ = None;
                let mut schema_name__ = None;
                let mut max_results__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaName => {
                            if schema_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaName"));
                            }
                            schema_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListTablesRequest {
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.ListTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTablesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tables.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.ListTablesResponse", len)?;
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTablesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tables",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tables,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tables" => Ok(GeneratedField::Tables),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTablesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.ListTablesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTablesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tables__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTablesResponse {
                    tables: tables__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.ListTablesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if !self.schema_name.is_empty() {
            len += 1;
        }
        if self.table_type != 0 {
            len += 1;
        }
        if self.data_source_format != 0 {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if !self.storage_location.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if !self.properties.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.table_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.tables.v1.TableInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if self.table_type != 0 {
            let v = TableType::try_from(self.table_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.table_type)))?;
            struct_ser.serialize_field("tableType", &v)?;
        }
        if self.data_source_format != 0 {
            let v = DataSourceFormat::try_from(self.data_source_format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_source_format)))?;
            struct_ser.serialize_field("dataSourceFormat", &v)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if !self.storage_location.is_empty() {
            struct_ser.serialize_field("storageLocation", &self.storage_location)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if !self.properties.is_empty() {
            struct_ser.serialize_field("properties", &self.properties)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.table_id.is_empty() {
            struct_ser.serialize_field("tableId", &self.table_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "catalog_name",
            "catalogName",
            "schema_name",
            "schemaName",
            "table_type",
            "tableType",
            "data_source_format",
            "dataSourceFormat",
            "columns",
            "storage_location",
            "storageLocation",
            "comment",
            "properties",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "table_id",
            "tableId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            SchemaName,
            TableType,
            DataSourceFormat,
            Columns,
            StorageLocation,
            Comment,
            Properties,
            CreatedAt,
            UpdatedAt,
            TableId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "schemaName" | "schema_name" => Ok(GeneratedField::SchemaName),
                            "tableType" | "table_type" => Ok(GeneratedField::TableType),
                            "dataSourceFormat" | "data_source_format" => Ok(GeneratedField::DataSourceFormat),
                            "columns" => Ok(GeneratedField::Columns),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.tables.v1.TableInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut schema_name__ = None;
                let mut table_type__ = None;
                let mut data_source_format__ = None;
                let mut columns__ = None;
                let mut storage_location__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut table_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaName => {
                            if schema_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaName"));
                            }
                            schema_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableType => {
                            if table_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableType"));
                            }
                            table_type__ = Some(map_.next_value::<TableType>()? as i32);
                        }
                        GeneratedField::DataSourceFormat => {
                            if data_source_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataSourceFormat"));
                            }
                            data_source_format__ = Some(map_.next_value::<DataSourceFormat>()? as i32);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageLocation => {
                            if storage_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocation"));
                            }
                            storage_location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TableInfo {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    table_type: table_type__.unwrap_or_default(),
                    data_source_format: data_source_format__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                    storage_location: storage_location__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    table_id: table_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.tables.v1.TableInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TABLE_TYPE_UNSPECIFIED",
            Self::Table => "TABLE_TYPE_TABLE",
            Self::View => "TABLE_TYPE_VIEW",
            Self::External => "TABLE_TYPE_EXTERNAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TableType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TABLE_TYPE_UNSPECIFIED",
            "TABLE_TYPE_TABLE",
            "TABLE_TYPE_VIEW",
            "TABLE_TYPE_EXTERNAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TABLE_TYPE_UNSPECIFIED" => Ok(TableType::Unspecified),
                    "TABLE_TYPE_TABLE" => Ok(TableType::Table),
                    "TABLE_TYPE_VIEW" => Ok(TableType::View),
                    "TABLE_TYPE_EXTERNAL" => Ok(TableType::External),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
