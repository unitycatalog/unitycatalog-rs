// @generated
impl serde::Serialize for CreateVolumeRequest {
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
        if self.volume_type != 0 {
            len += 1;
        }
        if !self.storage_location.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.CreateVolumeRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if self.volume_type != 0 {
            let v = VolumeType::try_from(self.volume_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.volume_type)))?;
            struct_ser.serialize_field("volumeType", &v)?;
        }
        if !self.storage_location.is_empty() {
            struct_ser.serialize_field("storageLocation", &self.storage_location)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateVolumeRequest {
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
            "volume_type",
            "volumeType",
            "storage_location",
            "storageLocation",
            "comment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            SchemaName,
            VolumeType,
            StorageLocation,
            Comment,
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
                            "volumeType" | "volume_type" => Ok(GeneratedField::VolumeType),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "comment" => Ok(GeneratedField::Comment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateVolumeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.CreateVolumeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateVolumeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut schema_name__ = None;
                let mut volume_type__ = None;
                let mut storage_location__ = None;
                let mut comment__ = None;
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
                        GeneratedField::VolumeType => {
                            if volume_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volumeType"));
                            }
                            volume_type__ = Some(map_.next_value::<VolumeType>()? as i32);
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
                    }
                }
                Ok(CreateVolumeRequest {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    volume_type: volume_type__.unwrap_or_default(),
                    storage_location: storage_location__.unwrap_or_default(),
                    comment: comment__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.CreateVolumeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateVolumeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.volume.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.CreateVolumeResponse", len)?;
        if let Some(v) = self.volume.as_ref() {
            struct_ser.serialize_field("volume", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateVolumeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Volume,
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
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateVolumeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.CreateVolumeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateVolumeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut volume__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateVolumeResponse {
                    volume: volume__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.CreateVolumeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteVolumeRequest {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.DeleteVolumeRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteVolumeRequest {
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
            type Value = DeleteVolumeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.DeleteVolumeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteVolumeRequest, V::Error>
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
                Ok(DeleteVolumeRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.DeleteVolumeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteVolumeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.DeleteVolumeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteVolumeResponse {
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
            type Value = DeleteVolumeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.DeleteVolumeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteVolumeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteVolumeResponse {
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.DeleteVolumeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVolumeRequest {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.GetVolumeRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVolumeRequest {
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
            type Value = GetVolumeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.GetVolumeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVolumeRequest, V::Error>
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
                Ok(GetVolumeRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.GetVolumeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVolumeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.volume.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.GetVolumeResponse", len)?;
        if let Some(v) = self.volume.as_ref() {
            struct_ser.serialize_field("volume", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVolumeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Volume,
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
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVolumeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.GetVolumeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVolumeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut volume__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetVolumeResponse {
                    volume: volume__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.GetVolumeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListVolumesRequest {
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
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.ListVolumesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListVolumesRequest {
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
            type Value = ListVolumesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.ListVolumesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListVolumesRequest, V::Error>
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
                Ok(ListVolumesRequest {
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.ListVolumesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateVolumeRequest {
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
        if !self.new_name.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.UpdateVolumeRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        if !self.new_name.is_empty() {
            struct_ser.serialize_field("newName", &self.new_name)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateVolumeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
            "new_name",
            "newName",
            "comment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
            NewName,
            Comment,
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
                            "newName" | "new_name" => Ok(GeneratedField::NewName),
                            "comment" => Ok(GeneratedField::Comment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateVolumeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.UpdateVolumeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateVolumeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                let mut new_name__ = None;
                let mut comment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewName => {
                            if new_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newName"));
                            }
                            new_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateVolumeRequest {
                    full_name: full_name__.unwrap_or_default(),
                    new_name: new_name__.unwrap_or_default(),
                    comment: comment__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.UpdateVolumeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateVolumeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.volume.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.UpdateVolumeResponse", len)?;
        if let Some(v) = self.volume.as_ref() {
            struct_ser.serialize_field("volume", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateVolumeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Volume,
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
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateVolumeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.UpdateVolumeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateVolumeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut volume__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateVolumeResponse {
                    volume: volume__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.UpdateVolumeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VolumeInfo {
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
        if self.volume_type != 0 {
            len += 1;
        }
        if !self.storage_location.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("unitycatalog.volumes.v1.VolumeInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if self.volume_type != 0 {
            let v = VolumeType::try_from(self.volume_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.volume_type)))?;
            struct_ser.serialize_field("volumeType", &v)?;
        }
        if !self.storage_location.is_empty() {
            struct_ser.serialize_field("storageLocation", &self.storage_location)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VolumeInfo {
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
            "volume_type",
            "volumeType",
            "storage_location",
            "storageLocation",
            "comment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            SchemaName,
            VolumeType,
            StorageLocation,
            Comment,
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
                            "volumeType" | "volume_type" => Ok(GeneratedField::VolumeType),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "comment" => Ok(GeneratedField::Comment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VolumeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct unitycatalog.volumes.v1.VolumeInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VolumeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut schema_name__ = None;
                let mut volume_type__ = None;
                let mut storage_location__ = None;
                let mut comment__ = None;
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
                        GeneratedField::VolumeType => {
                            if volume_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volumeType"));
                            }
                            volume_type__ = Some(map_.next_value::<VolumeType>()? as i32);
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
                    }
                }
                Ok(VolumeInfo {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    volume_type: volume_type__.unwrap_or_default(),
                    storage_location: storage_location__.unwrap_or_default(),
                    comment: comment__,
                })
            }
        }
        deserializer.deserialize_struct("unitycatalog.volumes.v1.VolumeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VolumeOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VOLUME_OPERATION_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VolumeOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VOLUME_OPERATION_UNSPECIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VolumeOperation;

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
                    "VOLUME_OPERATION_UNSPECIFIED" => Ok(VolumeOperation::Unspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VolumeType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VOLUME_TYPE_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VolumeType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VOLUME_TYPE_UNSPECIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VolumeType;

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
                    "VOLUME_TYPE_UNSPECIFIED" => Ok(VolumeType::Unspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
