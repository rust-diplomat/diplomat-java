#[diplomat::bridge]
#[diplomat::abi_rename = "namespace_{0}"]
#[diplomat::attr(not(any(c, kotlin)), rename = "Renamed{0}")]
#[diplomat::attr(auto, namespace = "ns")]
pub mod ffi {
    #[diplomat::opaque]
    #[diplomat::attr(not(supports = iterators), disable)]
    pub struct MyIterable(Vec<u8>);

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = iterators), disable)]
    pub struct MyIterator<'a>(std::slice::Iter<'a, u8>);

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = indexing), disable)]
    pub struct MyIndexer(Vec<String>);

    impl MyIterable {
        #[diplomat::attr(auto, constructor)]
        pub fn new(x: &[u8]) -> Box<Self> {
            Box::new(Self(x.into()))
        }
        #[diplomat::attr(auto, iterable)]
        pub fn iter<'a>(&'a self) -> Box<MyIterator<'a>> {
            Box::new(MyIterator(self.0.iter()))
        }
    }

    impl<'a> MyIterator<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&mut self) -> Option<u8> {
            self.0.next().copied()
        }
    }
}
