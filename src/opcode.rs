#![allow(dead_code)]
pub const POP_TOP: u8 = 1;
pub const ROT_TWO: u8 = 2;
pub const ROT_THREE: u8 = 3;
pub const DUP_TOP: u8 = 4;
pub const DUP_TOP_TWO: u8 = 5;
pub const ROT_FOUR: u8 = 6;
pub const NOP: u8 = 9;
pub const UNARY_POSITIVE: u8 = 10;
pub const UNARY_NEGATIVE: u8 = 11;
pub const UNARY_NOT: u8 = 12;
pub const UNARY_INVERT: u8 = 15;
pub const BINARY_MATRIX_MULTIPLY: u8 = 16;
pub const INPLACE_MATRIX_MULTIPLY: u8 = 17;
pub const BINARY_POWER: u8 = 19;
pub const BINARY_MULTIPLY: u8 = 20;
pub const BINARY_MODULO: u8 = 22;
pub const BINARY_ADD: u8 = 23;
pub const BINARY_SUBTRACT: u8 = 24;
pub const BINARY_SUBSCR: u8 = 25;
pub const BINARY_FLOOR_DIVIDE: u8 = 26;
pub const BINARY_TRUE_DIVIDE: u8 = 27;
pub const INPLACE_FLOOR_DIVIDE: u8 = 28;
pub const INPLACE_TRUE_DIVIDE: u8 = 29;
pub const GET_LEN: u8 = 30;
pub const MATCH_MAPPING: u8 = 31;
pub const MATCH_SEQUENCE: u8 = 32;
pub const MATCH_KEYS: u8 = 33;
pub const COPY_DICT_WITHOUT_KEYS: u8 = 34;
pub const WITH_EXCEPT_START: u8 = 49;
pub const GET_AITER: u8 = 50;
pub const GET_ANEXT: u8 = 51;
pub const BEFORE_ASYNC_WITH: u8 = 52;
pub const END_ASYNC_FOR: u8 = 54;
pub const INPLACE_ADD: u8 = 55;
pub const INPLACE_SUBTRACT: u8 = 56;
pub const INPLACE_MULTIPLY: u8 = 57;
pub const INPLACE_MODULO: u8 = 59;
pub const STORE_SUBSCR: u8 = 60;
pub const DELETE_SUBSCR: u8 = 61;
pub const BINARY_LSHIFT: u8 = 62;
pub const BINARY_RSHIFT: u8 = 63;
pub const BINARY_AND: u8 = 64;
pub const BINARY_XOR: u8 = 65;
pub const BINARY_OR: u8 = 66;
pub const INPLACE_POWER: u8 = 67;
pub const GET_ITER: u8 = 68;
pub const GET_YIELD_FROM_ITER: u8 = 69;
pub const PRINT_EXPR: u8 = 70;
pub const LOAD_BUILD_CLASS: u8 = 71;
pub const YIELD_FROM: u8 = 72;
pub const GET_AWAITABLE: u8 = 73;
pub const LOAD_ASSERTION_ERROR: u8 = 74;
pub const INPLACE_LSHIFT: u8 = 75;
pub const INPLACE_RSHIFT: u8 = 76;
pub const INPLACE_AND: u8 = 77;
pub const INPLACE_XOR: u8 = 78;
pub const INPLACE_OR: u8 = 79;
pub const LIST_TO_TUPLE: u8 = 82;
pub const RETURN_VALUE: u8 = 83;
pub const IMPORT_STAR: u8 = 84;
pub const SETUP_ANNOTATIONS: u8 = 85;
pub const YIELD_VALUE: u8 = 86;
pub const POP_BLOCK: u8 = 87;
pub const POP_EXCEPT: u8 = 89;
pub const HAVE_ARGUMENT: u8 = 90;
pub const STORE_NAME: u8 = 90;
pub const DELETE_NAME: u8 = 91;
pub const UNPACK_SEQUENCE: u8 = 92;
pub const FOR_ITER: u8 = 93;
pub const UNPACK_EX: u8 = 94;
pub const STORE_ATTR: u8 = 95;
pub const DELETE_ATTR: u8 = 96;
pub const STORE_GLOBAL: u8 = 97;
pub const DELETE_GLOBAL: u8 = 98;
pub const ROT_N: u8 = 99;
pub const LOAD_CONST: u8 = 100;
pub const LOAD_NAME: u8 = 101;
pub const BUILD_TUPLE: u8 = 102;
pub const BUILD_LIST: u8 = 103;
pub const BUILD_SET: u8 = 104;
pub const BUILD_MAP: u8 = 105;
pub const LOAD_ATTR: u8 = 106;
pub const COMPARE_OP: u8 = 107;
pub const IMPORT_NAME: u8 = 108;
pub const IMPORT_FROM: u8 = 109;
pub const JUMP_FORWARD: u8 = 110;
pub const JUMP_IF_FALSE_OR_POP: u8 = 111;
pub const JUMP_IF_TRUE_OR_POP: u8 = 112;
pub const JUMP_ABSOLUTE: u8 = 113;
pub const POP_JUMP_IF_FALSE: u8 = 114;
pub const POP_JUMP_IF_TRUE: u8 = 115;
pub const LOAD_GLOBAL: u8 = 116;
pub const IS_OP: u8 = 117;
pub const CONTAINS_OP: u8 = 118;
pub const RERAISE: u8 = 119;
pub const JUMP_IF_NOT_EXC_MATCH: u8 = 121;
pub const SETUP_FINALLY: u8 = 122;
pub const LOAD_FAST: u8 = 124;
pub const STORE_FAST: u8 = 125;
pub const DELETE_FAST: u8 = 126;
pub const GEN_START: u8 = 129;
pub const RAISE_VARARGS: u8 = 130;
pub const CALL_FUNCTION: u8 = 131;
pub const MAKE_FUNCTION: u8 = 132;
pub const BUILD_SLICE: u8 = 133;
pub const LOAD_CLOSURE: u8 = 135;
pub const LOAD_DEREF: u8 = 136;
pub const STORE_DEREF: u8 = 137;
pub const DELETE_DEREF: u8 = 138;
pub const CALL_FUNCTION_KW: u8 = 141;
pub const CALL_FUNCTION_EX: u8 = 142;
pub const SETUP_WITH: u8 = 143;
pub const EXTENDED_ARG: u8 = 144;
pub const LIST_APPEND: u8 = 145;
pub const SET_ADD: u8 = 146;
pub const MAP_ADD: u8 = 147;
pub const LOAD_CLASSDEREF: u8 = 148;
pub const MATCH_CLASS: u8 = 152;
pub const SETUP_ASYNC_WITH: u8 = 154;
pub const FORMAT_VALUE: u8 = 155;
pub const BUILD_CONST_KEY_MAP: u8 = 156;
pub const BUILD_STRING: u8 = 157;
pub const LOAD_METHOD: u8 = 160;
pub const CALL_METHOD: u8 = 161;
pub const LIST_EXTEND: u8 = 162;
pub const SET_UPDATE: u8 = 163;
pub const DICT_MERGE: u8 = 164;
pub const DICT_UPDATE: u8 = 165;
