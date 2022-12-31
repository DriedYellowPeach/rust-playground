# ch3 Basic Types

## Introduction
the design goal of rust basic types is: safety, efficiency, and concision.

safety:
* use reference rather than raw pointers, reference is never null, never dangling
* no unions, use enum instead

efficiency:
* the bit trick, `Option<&T>` takes up only one Machine Word
* prefer compile-time cost than runtime, prefer generics over abstract object(trait object)

concision:
* the length of integer type is clear, `i32`, `u8`, better than `long long`
* array, vector, slice are different types

## Type Inference
in most situation, you don't need to spell out the type of variables:
```rust
let _a = 5;
let _b: i8 = 127;
```

but in some situation, type must be clearly spell out:
* function parameter
* return type
* struct field type
* when compiler can't infer the type

```rust
let v = (1..5).into_iter().collect::<Vec<i32>>();
```

## Primitive Types
type list:
* numeric
* tuple
* struct, unit struct, tuple-like struct
* enum
* box
* reference(shared and mutable)
* String
* str &str
* array
* slice
* vector
* trait object 
* closure
* function

## Pointer Types
there are basically 3 pointer types:
* reference, shared and mutable
* all kinds of boxes, `box, rc, arc, cell`
* raw pointers, `* const T` `* mut T`

## Sequence Types
there are basically 3 sequence types:
* array, `[T;N]`, element type and lenght are both parts of the type
* vector, `Vec<T>`, heap allocated buffer, dynamical growing
* slice, `&[T], &mut [T]`, fat pointer, referece to sequence

array is always initialized, there is no notation for uninitalized array.
 
## Vector
resizable, element on the heap.
basic operation: `insert, pop, swap_remove, with_capacity`

# Slice
slice is an unsized object, you can only use reference to slice, reference to slice is fat pointer: address + size

can make slice from array or vector, `&v[..]`.

function always consider using slice as arguments, this make function easy to use, slice can point to stack, heap. Many method belong to slice.




