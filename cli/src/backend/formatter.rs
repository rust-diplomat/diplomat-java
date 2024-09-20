use std::borrow::Cow;

use diplomat_core::hir::{
    self, FloatType, IntType, PrimitiveType, ReturnType, Slice, StructPathLike, SuccessType,
    TyPosition, Type, TypeContext,
};
use heck::ToLowerCamelCase;

pub(crate) struct JavaFormatter<'cx> {
    tcx: &'cx TypeContext,
}

const INVALID_NAMES: &[&str] = &[
    "new", "static", "default", "private", "internal", "toString",
];

const DISALLOWED_CORE_TYPES: &[&str] = &["Object", "String"];

impl<'cx> JavaFormatter<'cx> {
    pub fn new(tcx: &'cx TypeContext) -> Self {
        Self { tcx }
    }

    pub fn fmt_c_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        method.abi_name.as_str().into()
    }

    pub fn fmt_field_name<'a, TyP: TyPosition>(
        &self,
        field: &'a hir::StructField<TyP>,
    ) -> Cow<'a, str> {
        let name = field.name.as_str().to_lower_camel_case();
        if INVALID_NAMES.contains(&&*name) || DISALLOWED_CORE_TYPES.contains(&&*name) {
            format!("{name}_").into()
        } else {
            name.into()
        }
    }
    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        let name = method.name.as_str().to_lower_camel_case();
        let name = method.attrs.rename.apply(name.into());
        if INVALID_NAMES.contains(&&*name) || DISALLOWED_CORE_TYPES.contains(&&*name) {
            format!("{name}_").into()
        } else {
            name
        }
    }

    pub fn fmt_return_type_java<'a>(&self, return_ty: &'a ReturnType) -> Cow<'a, str> {
        match return_ty {
            ReturnType::Infallible(ref success) => self.fmt_success_type_java(success),
            ReturnType::Fallible(success, _) => {
                todo!("Don't yet support fallible returns. Return type was: {success:?}")
            }
            ReturnType::Nullable(success) => {
                todo!("Don't yet support Nullable returns. Return type was: {success:?}")
            }
        }
    }

    pub fn fmt_success_type_java<'a>(&self, success_ty: &'a SuccessType) -> Cow<'a, str> {
        match success_ty {
            SuccessType::Write => "String".into(),
            SuccessType::OutType(ref o) => self.fmt_java_type(o),
            SuccessType::Unit => "void".into(),
            _ => todo!(),
        }
    }

    pub fn fmt_java_type<'a, P: TyPosition>(&self, ty: &'a Type<P>) -> Cow<'a, str> {
        match ty {
            hir::Type::Primitive(ref p) => self.fmt_java_primitive(p),
            hir::Type::Opaque(o) => self.tcx.resolve_opaque(o.tcx_id).name.to_string().into(),
            hir::Type::Struct(s) => self.tcx.resolve_type(s.id()).name().to_string().into(),
            hir::Type::Enum(e) => self.tcx.resolve_enum(e.tcx_id).name.to_string().into(),
            hir::Type::Slice(Slice::Str(_, _)) => "String".into(),
            hir::Type::Slice(Slice::Primitive(_, p)) => {
                format!("{}[]", self.fmt_java_primitive(p)).into()
            }
            hir::Type::Slice(Slice::Strs(_)) => "String []".into(),
            ty => todo!("haven't implemented, {ty:?}"),
        }
    }

    pub fn fmt_java_primitive<'a>(&self, ty: &'a PrimitiveType) -> Cow<'a, str> {
        match ty {
            PrimitiveType::Bool => "boolean",
            PrimitiveType::Char => "int",
            PrimitiveType::Byte => "byte",
            PrimitiveType::Int(IntType::I8 | IntType::U8) => "byte",
            PrimitiveType::Int(IntType::I16 | IntType::U16) => "short",
            PrimitiveType::Int(IntType::I32 | IntType::U32) => "int",
            PrimitiveType::Int(IntType::I64 | IntType::U64) => "long",
            PrimitiveType::IntSize(_) => "long",
            PrimitiveType::Int128(_) => panic!("128 not supported by java"),
            PrimitiveType::Float(FloatType::F32) => "float",
            PrimitiveType::Float(FloatType::F64) => "double",
        }
        .into()
    }

    pub fn fmt_native_primitive<'a>(&self, ty: &'a PrimitiveType) -> Cow<'a, str> {
        match ty {
            PrimitiveType::Bool => "byte".into(),
            p => self.fmt_java_primitive(p),
        }
    }

    pub fn fmt_param_name(&self, name: &str) -> String {
        name.to_lower_camel_case()
    }
}
