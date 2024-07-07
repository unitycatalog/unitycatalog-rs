// @generated
impl serde::Serialize for CreateFunction {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.CreateFunction", len)?;
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
            let v = super::super::v1::ColumnTypeName::try_from(self.data_type)
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFunction {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateFunction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.CreateFunction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFunction, V::Error>
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
                            data_type__ = Some(map_.next_value::<super::super::v1::ColumnTypeName>()? as i32);
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
                    }
                }
                Ok(CreateFunction {
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
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.CreateFunction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFunctionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.function_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.CreateFunctionRequest", len)?;
        if let Some(v) = self.function_info.as_ref() {
            struct_ser.serialize_field("functionInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "function_info",
            "functionInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FunctionInfo,
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
                            "functionInfo" | "function_info" => Ok(GeneratedField::FunctionInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.CreateFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut function_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FunctionInfo => {
                            if function_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionInfo"));
                            }
                            function_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateFunctionRequest {
                    function_info: function_info__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.CreateFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFunctionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.CreateFunctionResponse", len)?;
        if let Some(v) = self.function.as_ref() {
            struct_ser.serialize_field("function", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFunctionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.CreateFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateFunctionResponse {
                    function: function__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.CreateFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteFunctionRequest {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.DeleteFunctionRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.DeleteFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteFunctionRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.DeleteFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteFunctionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.DeleteFunctionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteFunctionResponse {
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
            type Value = DeleteFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.DeleteFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteFunctionResponse {
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.DeleteFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFunctionRequest {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.GetFunctionRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.GetFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFunctionRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.GetFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFunctionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.GetFunctionResponse", len)?;
        if let Some(v) = self.function.as_ref() {
            struct_ser.serialize_field("function", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFunctionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.GetFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetFunctionResponse {
                    function: function__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.GetFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFunctionsRequest {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.ListFunctionsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListFunctionsRequest {
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
            type Value = ListFunctionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.ListFunctionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFunctionsRequest, V::Error>
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
                Ok(ListFunctionsRequest {
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.ListFunctionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFunctionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.functions.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.functions.v1.ListFunctionsResponse", len)?;
        if !self.functions.is_empty() {
            struct_ser.serialize_field("functions", &self.functions)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFunctionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "functions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Functions,
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
                            "functions" => Ok(GeneratedField::Functions),
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
            type Value = ListFunctionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.functions.v1.ListFunctionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFunctionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut functions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Functions => {
                            if functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functions"));
                            }
                            functions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListFunctionsResponse {
                    functions: functions__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.functions.v1.ListFunctionsResponse", FIELDS, GeneratedVisitor)
    }
}
