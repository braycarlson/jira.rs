#[derive(Clone, Copy, Debug)]
pub struct JSONNode;

impl JSONNode {
    pub const ARRAY: crate::Field<Self, crate::Boolean> = crate::Field::new("array");
    pub const BIG_DECIMAL: crate::Field<Self, crate::Boolean> = crate::Field::new("bigDecimal");
    pub const BIG_INTEGER: crate::Field<Self, crate::Boolean> = crate::Field::new("bigInteger");

    pub const BIG_INTEGER_VALUE: crate::Field<Self, crate::Integer> =
        crate::Field::new("bigIntegerValue");

    pub const BINARY: crate::Field<Self, crate::Boolean> = crate::Field::new("binary");

    pub const BINARY_VALUE: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("binaryValue");

    pub const BOOLEAN: crate::Field<Self, crate::Boolean> = crate::Field::new("boolean");
    pub const BOOLEAN_VALUE: crate::Field<Self, crate::Boolean> = crate::Field::new("booleanValue");

    pub const CONTAINER_NODE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("containerNode");

    pub const DECIMAL_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("decimalValue");
    pub const DOUBLE: crate::Field<Self, crate::Boolean> = crate::Field::new("double");
    pub const DOUBLE_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("doubleValue");
    pub const ELEMENTS: crate::Field<Self, crate::Any> = crate::Field::new("elements");
    pub const FIELDS: crate::Field<Self, crate::Any> = crate::Field::new("fields");
    pub const FIELD_NAMES: crate::Field<Self, crate::Any> = crate::Field::new("fieldNames");

    pub const FLOATING_POINT_NUMBER: crate::Field<Self, crate::Boolean> =
        crate::Field::new("floatingPointNumber");

    pub const INT: crate::Field<Self, crate::Boolean> = crate::Field::new("int");

    pub const INTEGRAL_NUMBER: crate::Field<Self, crate::Boolean> =
        crate::Field::new("integralNumber");

    pub const INT_VALUE: crate::Field<Self, crate::Integer> = crate::Field::new("intValue");
    pub const LONG: crate::Field<Self, crate::Boolean> = crate::Field::new("long");
    pub const LONG_VALUE: crate::Field<Self, crate::Integer> = crate::Field::new("longValue");
    pub const MISSING_NODE: crate::Field<Self, crate::Boolean> = crate::Field::new("missingNode");
    pub const NULL: crate::Field<Self, crate::Boolean> = crate::Field::new("null");
    pub const NUMBER: crate::Field<Self, crate::Boolean> = crate::Field::new("number");
    pub const NUMBER_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("numberType");
    pub const NUMBER_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("numberValue");
    pub const OBJECT: crate::Field<Self, crate::Boolean> = crate::Field::new("object");
    pub const POJO: crate::Field<Self, crate::Boolean> = crate::Field::new("pojo");
    pub const TEXTUAL: crate::Field<Self, crate::Boolean> = crate::Field::new("textual");
    pub const TEXT_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("textValue");

    pub const VALUE_AS_BOOLEAN: crate::Field<Self, crate::Boolean> =
        crate::Field::new("valueAsBoolean");

    pub const VALUE_AS_DOUBLE: crate::Field<Self, crate::Number> =
        crate::Field::new("valueAsDouble");

    pub const VALUE_AS_INT: crate::Field<Self, crate::Integer> = crate::Field::new("valueAsInt");
    pub const VALUE_AS_LONG: crate::Field<Self, crate::Integer> = crate::Field::new("valueAsLong");
    pub const VALUE_AS_TEXT: crate::Field<Self, crate::Text> = crate::Field::new("valueAsText");
    pub const VALUE_NODE: crate::Field<Self, crate::Boolean> = crate::Field::new("valueNode");
}
