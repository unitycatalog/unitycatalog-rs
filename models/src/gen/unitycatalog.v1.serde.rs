// @generated
impl serde::Serialize for CatalogInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.CatalogInfo", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CatalogInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "comment",
            "properties",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Comment,
            Properties,
            CreatedAt,
            UpdatedAt,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CatalogInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.CatalogInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CatalogInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(CatalogInfo {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.CatalogInfo", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for Dependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dependency.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.Dependency", len)?;
        if let Some(v) = self.dependency.as_ref() {
            match v {
                dependency::Dependency::Table(v) => {
                    struct_ser.serialize_field("table", v)?;
                }
                dependency::Dependency::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table",
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Table,
            Function,
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
                            "function" => Ok(GeneratedField::Function),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.Dependency")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dependency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Table => {
                            if dependency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            dependency__ = map_.next_value::<::std::option::Option<_>>()?.map(dependency::Dependency::Table)
;
                        }
                        GeneratedField::Function => {
                            if dependency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            dependency__ = map_.next_value::<::std::option::Option<_>>()?.map(dependency::Dependency::Function)
;
                        }
                    }
                }
                Ok(Dependency {
                    dependency: dependency__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.Dependency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DependencyList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dependencies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.DependencyList", len)?;
        if !self.dependencies.is_empty() {
            struct_ser.serialize_field("dependencies", &self.dependencies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DependencyList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dependencies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dependencies,
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
                            "dependencies" => Ok(GeneratedField::Dependencies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DependencyList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.DependencyList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DependencyList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dependencies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Dependencies => {
                            if dependencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dependencies"));
                            }
                            dependencies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DependencyList {
                    dependencies: dependencies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.DependencyList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionDependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.function_full_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.FunctionDependency", len)?;
        if !self.function_full_name.is_empty() {
            struct_ser.serialize_field("functionFullName", &self.function_full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionDependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "function_full_name",
            "functionFullName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FunctionFullName,
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
                            "functionFullName" | "function_full_name" => Ok(GeneratedField::FunctionFullName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionDependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.FunctionDependency")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionDependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut function_full_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FunctionFullName => {
                            if function_full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionFullName"));
                            }
                            function_full_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionDependency {
                    function_full_name: function_full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.FunctionDependency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionInfo {
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
        if self.input_params.is_some() {
            len += 1;
        }
        if self.data_type != 0 {
            len += 1;
        }
        if !self.full_data_type.is_empty() {
            len += 1;
        }
        if self.return_params.is_some() {
            len += 1;
        }
        if !self.routine_body.is_empty() {
            len += 1;
        }
        if !self.routine_definition.is_empty() {
            len += 1;
        }
        if self.routine_dependencies.is_some() {
            len += 1;
        }
        if !self.parameter_style.is_empty() {
            len += 1;
        }
        if self.is_deterministic {
            len += 1;
        }
        if !self.sql_data_access.is_empty() {
            len += 1;
        }
        if self.is_null_call {
            len += 1;
        }
        if !self.security_type.is_empty() {
            len += 1;
        }
        if !self.specific_name.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if !self.properties.is_empty() {
            len += 1;
        }
        if !self.full_name.is_empty() {
            len += 1;
        }
        if !self.external_language.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.function_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.FunctionInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if let Some(v) = self.input_params.as_ref() {
            struct_ser.serialize_field("inputParams", v)?;
        }
        if self.data_type != 0 {
            let v = ColumnTypeName::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        if !self.full_data_type.is_empty() {
            struct_ser.serialize_field("fullDataType", &self.full_data_type)?;
        }
        if let Some(v) = self.return_params.as_ref() {
            struct_ser.serialize_field("returnParams", v)?;
        }
        if !self.routine_body.is_empty() {
            struct_ser.serialize_field("routineBody", &self.routine_body)?;
        }
        if !self.routine_definition.is_empty() {
            struct_ser.serialize_field("routineDefinition", &self.routine_definition)?;
        }
        if let Some(v) = self.routine_dependencies.as_ref() {
            struct_ser.serialize_field("routineDependencies", v)?;
        }
        if !self.parameter_style.is_empty() {
            struct_ser.serialize_field("parameterStyle", &self.parameter_style)?;
        }
        if self.is_deterministic {
            struct_ser.serialize_field("isDeterministic", &self.is_deterministic)?;
        }
        if !self.sql_data_access.is_empty() {
            struct_ser.serialize_field("sqlDataAccess", &self.sql_data_access)?;
        }
        if self.is_null_call {
            struct_ser.serialize_field("isNullCall", &self.is_null_call)?;
        }
        if !self.security_type.is_empty() {
            struct_ser.serialize_field("securityType", &self.security_type)?;
        }
        if !self.specific_name.is_empty() {
            struct_ser.serialize_field("specificName", &self.specific_name)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if !self.properties.is_empty() {
            struct_ser.serialize_field("properties", &self.properties)?;
        }
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        if !self.external_language.is_empty() {
            struct_ser.serialize_field("externalLanguage", &self.external_language)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.function_id.is_empty() {
            struct_ser.serialize_field("functionId", &self.function_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionInfo {
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
            "input_params",
            "inputParams",
            "data_type",
            "dataType",
            "full_data_type",
            "fullDataType",
            "return_params",
            "returnParams",
            "routine_body",
            "routineBody",
            "routine_definition",
            "routineDefinition",
            "routine_dependencies",
            "routineDependencies",
            "parameter_style",
            "parameterStyle",
            "is_deterministic",
            "isDeterministic",
            "sql_data_access",
            "sqlDataAccess",
            "is_null_call",
            "isNullCall",
            "security_type",
            "securityType",
            "specific_name",
            "specificName",
            "comment",
            "properties",
            "full_name",
            "fullName",
            "external_language",
            "externalLanguage",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "function_id",
            "functionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            SchemaName,
            InputParams,
            DataType,
            FullDataType,
            ReturnParams,
            RoutineBody,
            RoutineDefinition,
            RoutineDependencies,
            ParameterStyle,
            IsDeterministic,
            SqlDataAccess,
            IsNullCall,
            SecurityType,
            SpecificName,
            Comment,
            Properties,
            FullName,
            ExternalLanguage,
            CreatedAt,
            UpdatedAt,
            FunctionId,
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
                            "inputParams" | "input_params" => Ok(GeneratedField::InputParams),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "fullDataType" | "full_data_type" => Ok(GeneratedField::FullDataType),
                            "returnParams" | "return_params" => Ok(GeneratedField::ReturnParams),
                            "routineBody" | "routine_body" => Ok(GeneratedField::RoutineBody),
                            "routineDefinition" | "routine_definition" => Ok(GeneratedField::RoutineDefinition),
                            "routineDependencies" | "routine_dependencies" => Ok(GeneratedField::RoutineDependencies),
                            "parameterStyle" | "parameter_style" => Ok(GeneratedField::ParameterStyle),
                            "isDeterministic" | "is_deterministic" => Ok(GeneratedField::IsDeterministic),
                            "sqlDataAccess" | "sql_data_access" => Ok(GeneratedField::SqlDataAccess),
                            "isNullCall" | "is_null_call" => Ok(GeneratedField::IsNullCall),
                            "securityType" | "security_type" => Ok(GeneratedField::SecurityType),
                            "specificName" | "specific_name" => Ok(GeneratedField::SpecificName),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            "externalLanguage" | "external_language" => Ok(GeneratedField::ExternalLanguage),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "functionId" | "function_id" => Ok(GeneratedField::FunctionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.FunctionInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut schema_name__ = None;
                let mut input_params__ = None;
                let mut data_type__ = None;
                let mut full_data_type__ = None;
                let mut return_params__ = None;
                let mut routine_body__ = None;
                let mut routine_definition__ = None;
                let mut routine_dependencies__ = None;
                let mut parameter_style__ = None;
                let mut is_deterministic__ = None;
                let mut sql_data_access__ = None;
                let mut is_null_call__ = None;
                let mut security_type__ = None;
                let mut specific_name__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut full_name__ = None;
                let mut external_language__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut function_id__ = None;
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
                        GeneratedField::InputParams => {
                            if input_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputParams"));
                            }
                            input_params__ = map_.next_value()?;
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<ColumnTypeName>()? as i32);
                        }
                        GeneratedField::FullDataType => {
                            if full_data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullDataType"));
                            }
                            full_data_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReturnParams => {
                            if return_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnParams"));
                            }
                            return_params__ = map_.next_value()?;
                        }
                        GeneratedField::RoutineBody => {
                            if routine_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routineBody"));
                            }
                            routine_body__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoutineDefinition => {
                            if routine_definition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routineDefinition"));
                            }
                            routine_definition__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoutineDependencies => {
                            if routine_dependencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routineDependencies"));
                            }
                            routine_dependencies__ = map_.next_value()?;
                        }
                        GeneratedField::ParameterStyle => {
                            if parameter_style__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameterStyle"));
                            }
                            parameter_style__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDeterministic => {
                            if is_deterministic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDeterministic"));
                            }
                            is_deterministic__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SqlDataAccess => {
                            if sql_data_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sqlDataAccess"));
                            }
                            sql_data_access__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsNullCall => {
                            if is_null_call__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isNullCall"));
                            }
                            is_null_call__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SecurityType => {
                            if security_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("securityType"));
                            }
                            security_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpecificName => {
                            if specific_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("specificName"));
                            }
                            specific_name__ = Some(map_.next_value()?);
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
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExternalLanguage => {
                            if external_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalLanguage"));
                            }
                            external_language__ = Some(map_.next_value()?);
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
                        GeneratedField::FunctionId => {
                            if function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionId"));
                            }
                            function_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionInfo {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    input_params: input_params__,
                    data_type: data_type__.unwrap_or_default(),
                    full_data_type: full_data_type__.unwrap_or_default(),
                    return_params: return_params__,
                    routine_body: routine_body__.unwrap_or_default(),
                    routine_definition: routine_definition__.unwrap_or_default(),
                    routine_dependencies: routine_dependencies__,
                    parameter_style: parameter_style__.unwrap_or_default(),
                    is_deterministic: is_deterministic__.unwrap_or_default(),
                    sql_data_access: sql_data_access__.unwrap_or_default(),
                    is_null_call: is_null_call__.unwrap_or_default(),
                    security_type: security_type__.unwrap_or_default(),
                    specific_name: specific_name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__.unwrap_or_default(),
                    full_name: full_name__.unwrap_or_default(),
                    external_language: external_language__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    function_id: function_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.FunctionInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionParameterInfo {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.FunctionParameterInfo", len)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionParameterInfo {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionParameterInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.FunctionParameterInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionParameterInfo, V::Error>
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
                    }
                }
                Ok(FunctionParameterInfo {
                    name: name__.unwrap_or_default(),
                    type_text: type_text__.unwrap_or_default(),
                    type_json: type_json__.unwrap_or_default(),
                    type_name: type_name__.unwrap_or_default(),
                    type_precision: type_precision__.unwrap_or_default(),
                    type_scale: type_scale__.unwrap_or_default(),
                    type_interval_type: type_interval_type__.unwrap_or_default(),
                    position: position__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.FunctionParameterInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionParameterInfos {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.FunctionParameterInfos", len)?;
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionParameterInfos {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parameters,
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
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionParameterInfos;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.FunctionParameterInfos")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionParameterInfos, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionParameterInfos {
                    parameters: parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.FunctionParameterInfos", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionParameterMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FUNCTION_PARAMETER_MODE_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FunctionParameterMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FUNCTION_PARAMETER_MODE_UNSPECIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionParameterMode;

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
                    "FUNCTION_PARAMETER_MODE_UNSPECIFIED" => Ok(FunctionParameterMode::Unspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionParameterType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FUNCTION_PARAMETER_TYPE_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FunctionParameterType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FUNCTION_PARAMETER_TYPE_UNSPECIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionParameterType;

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
                    "FUNCTION_PARAMETER_TYPE_UNSPECIFIED" => Ok(FunctionParameterType::Unspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SchemaInfo {
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
        if self.comment.is_some() {
            len += 1;
        }
        if !self.properties.is_empty() {
            len += 1;
        }
        if !self.full_name.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.schema_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.SchemaInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if !self.properties.is_empty() {
            struct_ser.serialize_field("properties", &self.properties)?;
        }
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.schema_id.is_empty() {
            struct_ser.serialize_field("schemaId", &self.schema_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SchemaInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "catalog_name",
            "catalogName",
            "comment",
            "properties",
            "full_name",
            "fullName",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "schema_id",
            "schemaId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            Comment,
            Properties,
            FullName,
            CreatedAt,
            UpdatedAt,
            SchemaId,
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
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "schemaId" | "schema_id" => Ok(GeneratedField::SchemaId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SchemaInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.SchemaInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SchemaInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut full_name__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut schema_id__ = None;
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
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
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
                        GeneratedField::SchemaId => {
                            if schema_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaId"));
                            }
                            schema_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SchemaInfo {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__.unwrap_or_default(),
                    full_name: full_name__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    schema_id: schema_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.SchemaInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableDependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_full_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.v1.TableDependency", len)?;
        if !self.table_full_name.is_empty() {
            struct_ser.serialize_field("tableFullName", &self.table_full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableDependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_full_name",
            "tableFullName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableFullName,
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
                            "tableFullName" | "table_full_name" => Ok(GeneratedField::TableFullName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableDependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.v1.TableDependency")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableDependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_full_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableFullName => {
                            if table_full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableFullName"));
                            }
                            table_full_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TableDependency {
                    table_full_name: table_full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.v1.TableDependency", FIELDS, GeneratedVisitor)
    }
}
