use sqlx_core::{
    database::{Database, HasArguments, HasValueRef},
    decode::Decode,
    encode::{Encode, IsNull},
    types::Type,
};

use crate::Brand;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

impl<Db, Tag, Raw> Type<Db> for Brand<Tag, Raw>
where
    Db: Database,
    Raw: Type<Db>,
{
    fn type_info() -> Db::TypeInfo {
        Raw::type_info()
    }
}

impl<'de, Db, Tag, Raw> Decode<'de, Db> for Brand<Tag, Raw>
where
    Db: Database,
    Raw: for<'a> Decode<'a, Db>,
{
    fn decode(value: <Db as HasValueRef<'de>>::ValueRef) -> Result<Brand<Tag, Raw>, BoxError> {
        let raw = <Raw as Decode<Db>>::decode(value)?;
        Ok(Brand::unchecked_from_raw(raw))
    }
}

impl<'en, Db, Tag, Raw> Encode<'en, Db> for Brand<Tag, Raw>
where
    Db: Database,
    Raw: for<'a> Encode<'a, Db>,
{
    fn encode_by_ref(&self, buf: &mut <Db as HasArguments<'en>>::ArgumentBuffer) -> IsNull {
        self.raw.encode_by_ref(buf)
    }
}
