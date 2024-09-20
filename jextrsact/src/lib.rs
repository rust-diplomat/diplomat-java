use std::borrow::Cow;

///  The goal of this library is to generate native binding code similar to jextract
///  but focusing only on functionality needed to make diplomat work
///  ```java
///    public class MyStruct {
///
///        MyStruct() {
///            // Should not be called directly
///        }
///
///        private static final GroupLayout $LAYOUT = MemoryLayout.structLayout(
///            somelib_h.C_CHAR.withName("a"),
///            somelib_h.C_BOOL.withName("b"),
///            somelib_h.C_CHAR.withName("c"),
///            MemoryLayout.paddingLayout(5),
///            somelib_h.C_LONG_LONG.withName("d"),
///            somelib_h.C_INT.withName("e"),
///            somelib_h.C_INT.withName("f"),
///            somelib_h.C_INT.withName("g"),
///            MemoryLayout.paddingLayout(4)
///        ).withName("MyStruct");
///     }
///  ```
///
pub fn gen_struct_layout<'a>() -> Cow<'a, str> {
    todo!()
}
