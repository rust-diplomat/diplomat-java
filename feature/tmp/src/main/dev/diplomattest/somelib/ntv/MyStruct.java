// Generated by jextract

package dev.diplomattest.somelib.ntv;

import java.lang.invoke.*;
import java.lang.foreign.*;
import java.nio.ByteOrder;
import java.util.*;
import java.util.function.*;
import java.util.stream.*;

import static java.lang.foreign.ValueLayout.*;
import static java.lang.foreign.MemoryLayout.PathElement.*;

/**
 * {@snippet lang=c :
 * struct MyStruct {
 *     uint8_t a;
 *     bool b;
 *     uint8_t c;
 *     uint64_t d;
 *     int32_t e;
 *     char32_t f;
 *     MyEnum g;
 * }
 * }
 */
public class MyStruct {

    MyStruct() {
        // Should not be called directly
    }

    private static final GroupLayout $LAYOUT = MemoryLayout.structLayout(
        somelib_h.C_CHAR.withName("a"),
        somelib_h.C_BOOL.withName("b"),
        somelib_h.C_CHAR.withName("c"),
        MemoryLayout.paddingLayout(5),
        somelib_h.C_LONG_LONG.withName("d"),
        somelib_h.C_INT.withName("e"),
        somelib_h.C_INT.withName("f"),
        somelib_h.C_INT.withName("g"),
        MemoryLayout.paddingLayout(4)
    ).withName("MyStruct");

    /**
     * The layout of this struct
     */
    public static final GroupLayout layout() {
        return $LAYOUT;
    }

    private static final OfByte a$LAYOUT = (OfByte)$LAYOUT.select(groupElement("a"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * uint8_t a
     * }
     */
    public static final OfByte a$layout() {
        return a$LAYOUT;
    }

    private static final long a$OFFSET = 0;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * uint8_t a
     * }
     */
    public static final long a$offset() {
        return a$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * uint8_t a
     * }
     */
    public static byte a(MemorySegment struct) {
        return struct.get(a$LAYOUT, a$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * uint8_t a
     * }
     */
    public static void a(MemorySegment struct, byte fieldValue) {
        struct.set(a$LAYOUT, a$OFFSET, fieldValue);
    }

    private static final OfBoolean b$LAYOUT = (OfBoolean)$LAYOUT.select(groupElement("b"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * bool b
     * }
     */
    public static final OfBoolean b$layout() {
        return b$LAYOUT;
    }

    private static final long b$OFFSET = 1;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * bool b
     * }
     */
    public static final long b$offset() {
        return b$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * bool b
     * }
     */
    public static boolean b(MemorySegment struct) {
        return struct.get(b$LAYOUT, b$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * bool b
     * }
     */
    public static void b(MemorySegment struct, boolean fieldValue) {
        struct.set(b$LAYOUT, b$OFFSET, fieldValue);
    }

    private static final OfByte c$LAYOUT = (OfByte)$LAYOUT.select(groupElement("c"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * uint8_t c
     * }
     */
    public static final OfByte c$layout() {
        return c$LAYOUT;
    }

    private static final long c$OFFSET = 2;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * uint8_t c
     * }
     */
    public static final long c$offset() {
        return c$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * uint8_t c
     * }
     */
    public static byte c(MemorySegment struct) {
        return struct.get(c$LAYOUT, c$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * uint8_t c
     * }
     */
    public static void c(MemorySegment struct, byte fieldValue) {
        struct.set(c$LAYOUT, c$OFFSET, fieldValue);
    }

    private static final OfLong d$LAYOUT = (OfLong)$LAYOUT.select(groupElement("d"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * uint64_t d
     * }
     */
    public static final OfLong d$layout() {
        return d$LAYOUT;
    }

    private static final long d$OFFSET = 8;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * uint64_t d
     * }
     */
    public static final long d$offset() {
        return d$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * uint64_t d
     * }
     */
    public static long d(MemorySegment struct) {
        return struct.get(d$LAYOUT, d$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * uint64_t d
     * }
     */
    public static void d(MemorySegment struct, long fieldValue) {
        struct.set(d$LAYOUT, d$OFFSET, fieldValue);
    }

    private static final OfInt e$LAYOUT = (OfInt)$LAYOUT.select(groupElement("e"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * int32_t e
     * }
     */
    public static final OfInt e$layout() {
        return e$LAYOUT;
    }

    private static final long e$OFFSET = 16;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * int32_t e
     * }
     */
    public static final long e$offset() {
        return e$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * int32_t e
     * }
     */
    public static int e(MemorySegment struct) {
        return struct.get(e$LAYOUT, e$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * int32_t e
     * }
     */
    public static void e(MemorySegment struct, int fieldValue) {
        struct.set(e$LAYOUT, e$OFFSET, fieldValue);
    }

    private static final OfInt f$LAYOUT = (OfInt)$LAYOUT.select(groupElement("f"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * char32_t f
     * }
     */
    public static final OfInt f$layout() {
        return f$LAYOUT;
    }

    private static final long f$OFFSET = 20;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * char32_t f
     * }
     */
    public static final long f$offset() {
        return f$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * char32_t f
     * }
     */
    public static int f(MemorySegment struct) {
        return struct.get(f$LAYOUT, f$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * char32_t f
     * }
     */
    public static void f(MemorySegment struct, int fieldValue) {
        struct.set(f$LAYOUT, f$OFFSET, fieldValue);
    }

    private static final OfInt g$LAYOUT = (OfInt)$LAYOUT.select(groupElement("g"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * MyEnum g
     * }
     */
    public static final OfInt g$layout() {
        return g$LAYOUT;
    }

    private static final long g$OFFSET = 24;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * MyEnum g
     * }
     */
    public static final long g$offset() {
        return g$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * MyEnum g
     * }
     */
    public static int g(MemorySegment struct) {
        return struct.get(g$LAYOUT, g$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * MyEnum g
     * }
     */
    public static void g(MemorySegment struct, int fieldValue) {
        struct.set(g$LAYOUT, g$OFFSET, fieldValue);
    }

    /**
     * Obtains a slice of {@code arrayParam} which selects the array element at {@code index}.
     * The returned segment has address {@code arrayParam.address() + index * layout().byteSize()}
     */
    public static MemorySegment asSlice(MemorySegment array, long index) {
        return array.asSlice(layout().byteSize() * index);
    }

    /**
     * The size (in bytes) of this struct
     */
    public static long sizeof() { return layout().byteSize(); }

    /**
     * Allocate a segment of size {@code layout().byteSize()} using {@code allocator}
     */
    public static MemorySegment allocate(SegmentAllocator allocator) {
        return allocator.allocate(layout());
    }

    /**
     * Allocate an array of size {@code elementCount} using {@code allocator}.
     * The returned segment has size {@code elementCount * layout().byteSize()}.
     */
    public static MemorySegment allocateArray(long elementCount, SegmentAllocator allocator) {
        return allocator.allocate(MemoryLayout.sequenceLayout(elementCount, layout()));
    }

    /**
     * Reinterprets {@code addr} using target {@code arena} and {@code cleanupAction} (if any).
     * The returned segment has size {@code layout().byteSize()}
     */
    public static MemorySegment reinterpret(MemorySegment addr, Arena arena, Consumer<MemorySegment> cleanup) {
        return reinterpret(addr, 1, arena, cleanup);
    }

    /**
     * Reinterprets {@code addr} using target {@code arena} and {@code cleanupAction} (if any).
     * The returned segment has size {@code elementCount * layout().byteSize()}
     */
    public static MemorySegment reinterpret(MemorySegment addr, long elementCount, Arena arena, Consumer<MemorySegment> cleanup) {
        return addr.reinterpret(layout().byteSize() * elementCount, arena, cleanup);
    }
}

