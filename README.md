# Irony

## IR infra

### Overview

#### Basic Components

| Objects     | Traits & Structs     | Macros               | Tests                |
| ----------- | -------------------- | -------------------- | -------------------- |
| Entity      | :white_check_mark:   | :white_check_mark:   | :white_large_square: |
| Operation   | :white_check_mark:   | :white_check_mark:   | :white_large_square: |
| Constraint  | :white_check_mark:   | :white_check_mark:   | :white_large_square: |
| Environ     | :white_check_mark:   | :white_check_mark:   | :white_large_square: |
| Interpret   | :white_large_square: | :white_large_square: | :white_large_square: |
| Parse/Print | :white_large_square: | :white_large_square: | :white_large_square: |

#### Planned Features

- [ ] Pass and Pass Manager;
- [ ] Type inference support;
- [ ] More casual entity definition: flexible data storage;
- [ ] Dialect support: combination of Enums?

### Details

loot at `irony` package

> package `irony_macros` is useless now, which has been moved to `.gitignore`;

## Demo

### CIRCT core dialects

- common:
  - [x] Attributes:
    - [x] StringAttr
    - [x] ArrayAttr
    - [x] TypeAttr
- `hw` dialect:
  - [x] Data types [DataType]
    - [x] IntType: signless, with width parameter;
    - [x] StructType: `!hw.struct<fieldName1: Type1, fieldName2: Type2>`
    - [x] ArrayType: `!hw.array<k * elementType>`
    - [x] UnpackedArrayType: `!hw.uarray<k * elementType>`
  - [ ] Module structure [Operation]
    - [ ] ModuleOp: `!hw.module<name: StringAttr, body: Region>`
      - [x] attributes: name, arg_names, output_names, arg_types, output_types
      - [ ] constraints: `ModuleTypeMatch`
    - [ ] InstanceOp: `!hw.instance<name: StringAttr, module: SymbolRefAttr, operands: ArrayAttr>`
      - [x] attrbutes: target_name & id, instance_name, arg_names, output_names, arg_types, output_types
      - [ ] constraints: `InstanceTypeMatch`
    - [x] InputOp: **this is additional in Irony, since Irony doen't introduce region arguments**
    - [x] OutputOp: `!hw.output<operands: ArrayAttr>`
  - [x] Miscellaneous [Operation]
    - [x] BitCastOp: `!hw.bitcast<operand: TypeAttr>`
    - [x] ConstantOp
    - [ ] Wire: **Optional**
  - [x] Aggregate
    - [x] AggregateConstantOp: this is hard to discribe,need **ArrayAttr**
    - [x] Array things: ArrayConcatOp ArrayCreateOp ArrayGetOp ArraySliceOp
    - [x] Struct things: StructCreateOp StructExplodeOp StructExtractOp StructInjectOp
