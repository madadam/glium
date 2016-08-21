//! Allow to use nalgebra types as attributes and uniforms.

extern crate nalgebra;

use uniforms::{AsUniformValue, UniformValue};
use vertex::{Attribute, AttributeType};
use self::nalgebra::{Matrix2, Matrix3, Matrix4,
                     Point1, Point2, Point3, Point4,
                     Vector1, Vector2, Vector3, Vector4};

macro_rules! impl_attribute {
    ($t:ty, $var:ident) => {
        unsafe impl Attribute for $t {
            #[inline]
            fn get_type() -> AttributeType {
                AttributeType::$var
            }
        }
    }
}

impl_attribute!(Vector1<i8>,  I8);
impl_attribute!(Vector1<i16>, I16);
impl_attribute!(Vector1<i32>, I32);
impl_attribute!(Vector1<i64>, I64);
impl_attribute!(Vector1<u8>,  U8);
impl_attribute!(Vector1<u16>, U16);
impl_attribute!(Vector1<u32>, U32);
impl_attribute!(Vector1<u64>, U64);
impl_attribute!(Vector1<f32>, F32);
impl_attribute!(Vector1<f64>, F64);

impl_attribute!(Vector2<i8>,  I8I8);
impl_attribute!(Vector2<i16>, I16I16);
impl_attribute!(Vector2<i32>, I32I32);
impl_attribute!(Vector2<i64>, I64I64);
impl_attribute!(Vector2<u8>,  U8U8);
impl_attribute!(Vector2<u16>, U16U16);
impl_attribute!(Vector2<u32>, U32U32);
impl_attribute!(Vector2<u64>, U64U64);
impl_attribute!(Vector2<f32>, F32F32);
impl_attribute!(Vector2<f64>, F64F64);

impl_attribute!(Vector3<i8>,  I8I8I8);
impl_attribute!(Vector3<i16>, I16I16I16);
impl_attribute!(Vector3<i32>, I32I32I32);
impl_attribute!(Vector3<i64>, I64I64I64);
impl_attribute!(Vector3<u8>,  U8U8U8);
impl_attribute!(Vector3<u16>, U16U16U16);
impl_attribute!(Vector3<u32>, U32U32U32);
impl_attribute!(Vector3<u64>, U64U64U64);
impl_attribute!(Vector3<f32>, F32F32F32);
impl_attribute!(Vector3<f64>, F64F64F64);

impl_attribute!(Vector4<i8>,  I8I8I8I8);
impl_attribute!(Vector4<i16>, I16I16I16I16);
impl_attribute!(Vector4<i32>, I32I32I32I32);
impl_attribute!(Vector4<i64>, I64I64I64I64);
impl_attribute!(Vector4<u8>,  U8U8U8U8);
impl_attribute!(Vector4<u16>, U16U16U16U16);
impl_attribute!(Vector4<u32>, U32U32U32U32);
impl_attribute!(Vector4<u64>, U64U64U64U64);
impl_attribute!(Vector4<f32>, F32F32F32F32);
impl_attribute!(Vector4<f64>, F64F64F64F64);

impl_attribute!(Point1<i8>,  I8);
impl_attribute!(Point1<i16>, I16);
impl_attribute!(Point1<i32>, I32);
impl_attribute!(Point1<i64>, I64);
impl_attribute!(Point1<u8>,  U8);
impl_attribute!(Point1<u16>, U16);
impl_attribute!(Point1<u32>, U32);
impl_attribute!(Point1<u64>, U64);
impl_attribute!(Point1<f32>, F32);
impl_attribute!(Point1<f64>, F64);

impl_attribute!(Point2<i8>,  I8I8);
impl_attribute!(Point2<i16>, I16I16);
impl_attribute!(Point2<i32>, I32I32);
impl_attribute!(Point2<i64>, I64I64);
impl_attribute!(Point2<u8>,  U8U8);
impl_attribute!(Point2<u16>, U16U16);
impl_attribute!(Point2<u32>, U32U32);
impl_attribute!(Point2<u64>, U64U64);
impl_attribute!(Point2<f32>, F32F32);
impl_attribute!(Point2<f64>, F64F64);

impl_attribute!(Point3<i8>,  I8I8I8);
impl_attribute!(Point3<i16>, I16I16I16);
impl_attribute!(Point3<i32>, I32I32I32);
impl_attribute!(Point3<i64>, I64I64I64);
impl_attribute!(Point3<u8>,  U8U8U8);
impl_attribute!(Point3<u16>, U16U16U16);
impl_attribute!(Point3<u32>, U32U32U32);
impl_attribute!(Point3<u64>, U64U64U64);
impl_attribute!(Point3<f32>, F32F32F32);
impl_attribute!(Point3<f64>, F64F64F64);

impl_attribute!(Point4<i8>,  I8I8I8I8);
impl_attribute!(Point4<i16>, I16I16I16I16);
impl_attribute!(Point4<i32>, I32I32I32I32);
impl_attribute!(Point4<i64>, I64I64I64I64);
impl_attribute!(Point4<u8>,  U8U8U8U8);
impl_attribute!(Point4<u16>, U16U16U16U16);
impl_attribute!(Point4<u32>, U32U32U32U32);
impl_attribute!(Point4<u64>, U64U64U64U64);
impl_attribute!(Point4<f32>, F32F32F32F32);
impl_attribute!(Point4<f64>, F64F64F64F64);

macro_rules! impl_as_uniform_value {
    ($t:ty, $u:ident) => {
        impl AsUniformValue for $t {
            #[inline]
            fn as_uniform_value(&self) -> UniformValue {
                UniformValue::$u(*self.as_ref())
            }
        }
    }
}

impl_as_uniform_value!(Vector2<f32>, Vec2);
impl_as_uniform_value!(Vector2<f64>, DoubleVec2);

impl_as_uniform_value!(Vector3<f32>, Vec3);
impl_as_uniform_value!(Vector3<f64>, DoubleVec3);

impl_as_uniform_value!(Vector4<f32>, Vec4);
impl_as_uniform_value!(Vector4<f64>, DoubleVec4);

impl_as_uniform_value!(Point2<f32>,  Vec2);
impl_as_uniform_value!(Point2<f64>,  DoubleVec2);

impl_as_uniform_value!(Point3<f32>,  Vec3);
impl_as_uniform_value!(Point3<f64>,  DoubleVec3);

impl_as_uniform_value!(Point4<f32>,  Vec4);
impl_as_uniform_value!(Point4<f64>,  DoubleVec4);

impl_as_uniform_value!(Matrix2<f32>, Mat2);
impl_as_uniform_value!(Matrix2<f64>, DoubleMat2);

impl_as_uniform_value!(Matrix3<f32>, Mat3);
impl_as_uniform_value!(Matrix3<f64>, DoubleMat3);

impl_as_uniform_value!(Matrix4<f32>, Mat4);
impl_as_uniform_value!(Matrix4<f64>, DoubleMat4);

