\# Binding To leveldb

In this exercise, we will create a binding to the `leveldb` database
library.

You will learn:

-   How to use raw C bindings (also called "sys"-crates)

-   How to manage and interact with foreign pointers

-   How to use Rusts Ownership system to provide additional guarantees

All necessary C functions will be described in this document. Their full
interface can be seen in the
[documentation](https://docs.rs/leveldb-sys/2.0.8/leveldb_sys/).

\#\# Setup

Create a new cargo library project called "leveldb".

Change the "Dependencies" section in `Cargo.toml` like this:

    [dependencies]
    leveldb-sys = "2"
    libc = "0.2"

    [dev-dependencies]
    tempdir = "0.3"

\#\# Opening and closing a database

Needed API:

-   `leveldb_t`: an opaque handle representing an opened database. The
    handle is thread-safe.

-   `leveldb_options_t`: an opaque handled representing the options to
    the database

-   `leveldb_options_create`: function creating an options struct

-   `leveldb_options_set_create_if_missing`: sets the
    "create\_if\_missing" flag on `leveldb_options_t`.

-   `leveldb_options_destroy`: Removes an `Options` struct from memory

-   `leveldb_open`: opens a database

-   `leveldb_close`: closes it properly

\# Exercise

1.  Implement a simple function that

    -   Opens a database

        -   "create if missing" is a good setting there

        -   Closes it

2.  Create Handle types for both Database and Options

3.  Create a `Database` type wrapping the handle

\# Help

Use the following base harness:

    use libc::{c_void, size_t};
    use std::ffi::CString;
    use std::path::Path;
    use std::ptr;
    use std::ptr::NonNull;

    use leveldb_sys::{
        leveldb_close, leveldb_create_iterator, leveldb_free, leveldb_get, leveldb_iter_destroy,
        leveldb_iter_next, leveldb_iter_seek_to_first, leveldb_iter_valid, leveldb_iter_value,
        leveldb_iterator_t, leveldb_open, leveldb_options_create, leveldb_options_destroy,
        leveldb_options_set_create_if_missing, leveldb_options_t, leveldb_put,
        leveldb_readoptions_create, leveldb_readoptions_destroy, leveldb_readoptions_t, leveldb_t,
        leveldb_writeoptions_create, leveldb_writeoptions_destroy, leveldb_writeoptions_t,
    };

    #[derive(Debug, Eq, PartialEq)]
    pub enum Error {
        OpenFail,
        GetFail,
        PutFail,
        NonUtf8Path,
    }

    #[test]
    fn test() -> Result<(), Error> {
        let tmp = tempdir::TempDir::new("leveldb_test").unwrap();
        let path = tmp.path().join("database");

        let mut error = ptr::null_mut();

        let c_string = CString::new(
            path.to_str().ok_or(Error::NonUtf8Path)?
        ).map_err(|_| Error::NonUtf8Path)?;

        unsafe {
            let options = leveldb_options_create();
            leveldb_options_set_create_if_missing(options, true as u8);

            let db = leveldb_open(options, c_string.as_ptr(), &mut error);

            if error == ptr::null_mut() {
                eprintln!("Opening worked!")
            } else {
                panic!("Error opening!")
            }

        }

        Ok(())
    }

\#\# Put and Get

Needed API:

-   `leveldb_readoptions_t`: Opaque type representing options for all
    read oerpations

-   `leveldb_writeoptions_t`: Opaque type representing options for all
    write operations

-   `leveldb_put`: Writes a binary value to a binary key in the database

-   `leveldb_get`: Reads a binary value from a binary key in the
    database

-   `leveldb_readoptions_create`: Create a default `readoptions_t`

-   `leveldb_writeoptions_create`: Create a default `writeoptions_t`

-   `leveldb_free`: Free the value allocation

\# Exercise

1.  Implement two functions on your `Database` type:

    -   For simplicity, create write/readoptions within the functions

    -   `pub fn put(&self, key: &[u8], data: &[u8]) \-> Result<(), Error>`

    -   `pub fn get(&self, key: &[u8]) \-> Result<Option<Box<[u8]>>, Error>`

    -   Take care to free the value returned by `get` properly

2.  Write a test

\#\# Iteration over database values

Needed API:

-   `leveldb_iterator_t`: The opaque handle to an iterator

-   `leveldb_iter_seek_to_first`: Seek the first item to iterate over

-   `leveldb_iter_next`: Advance the iterator by one element

-   `leveldb_iter_value`: Read the value at the current iterator
    position

-   `leveldb_iter_valid`: Check the iterator pointer for validity

\# Notes

The iterator protocol is a little thorny:

-   Before the first call to `leveldb_iter_seek_to_first`, the iterator
    pointer is invalid

-   End of the iteration is marked by the iterator pointer becoming
    invalid again

-   Note that iterators free their memory themselves and require you to
    copy out data

\# Help

To correctly create a heap allocated slice, use:

    let slice = std::slice::from_raw_parts(data as *mut u8, len);
    let result = Box::from(slice);

    leveldb_free(data as *mut c_void);
    Ok(Some(result))

\# Exercise

1.  Implement an iterator handle

2.  Implement an `Iterator` type that holds the necessary state on top
    of the handle

3.  Implement `pub fn iter(&self) \-> Iterator` on the Database

4.  Implement `std::iter::Iterator` for the `Iterator` type

    -   Iterate over `Box<[u8]>`

5.  Make the `Iterator` type reference the Database it was constructed
    from

    -   Discuss: what changes and how does it help us?

Write an appropriate test.

\#\# Help

\# Full solution

    use libc::{c_void, size_t};
    use std::ffi::CString;
    use std::path::Path;
    use std::ptr;
    use std::ptr::NonNull;

    use leveldb_sys::{
        leveldb_close, leveldb_create_iterator, leveldb_free, leveldb_get, leveldb_iter_destroy,
        leveldb_iter_next, leveldb_iter_seek_to_first, leveldb_iter_valid, leveldb_iter_value,
        leveldb_iterator_t, leveldb_open, leveldb_options_create, leveldb_options_destroy,
        leveldb_options_set_create_if_missing, leveldb_options_t, leveldb_put,
        leveldb_readoptions_create, leveldb_readoptions_destroy, leveldb_readoptions_t, leveldb_t,
        leveldb_writeoptions_create, leveldb_writeoptions_destroy, leveldb_writeoptions_t,
    };

    struct DBHandle {
        ptr: NonNull<leveldb_t>,
    }

    impl Drop for DBHandle {
        fn drop(&mut self) {
            unsafe { leveldb_close(self.ptr.as_ptr()) }
        }
    }

    struct IteratorHandle {
        ptr: NonNull<leveldb_iterator_t>,
    }

    impl IteratorHandle {
        fn new(database: &Database, read_options: ReadOptions) -> IteratorHandle {
            unsafe {
                let iterator_ptr =
                    leveldb_create_iterator(database.handle.ptr.as_ptr(), read_options.ptr.as_ptr());

                leveldb_iter_seek_to_first(iterator_ptr);

                IteratorHandle {
                    ptr: NonNull::new_unchecked(iterator_ptr),
                }
            }
        }

        fn next(&self) {
            unsafe { leveldb_iter_next(self.ptr.as_ptr()) };
        }

        fn valid(&self) -> bool {
            unsafe { leveldb_iter_valid(self.ptr.as_ptr()) != 0 }
        }

        fn value(&self) -> (*const i8, usize) {
            unsafe {
                let mut len = 0;

                let data = leveldb_iter_value(self.ptr.as_ptr(), &mut len);

                (data, len)
            }
        }
    }

    impl Drop for IteratorHandle {
        fn drop(&mut self) {
            unsafe { leveldb_iter_destroy(self.ptr.as_ptr()) }
        }
    }

    pub struct Options {
        ptr: NonNull<leveldb_options_t>,
    }

    impl Options {
        fn as_ptr(&self) -> *mut leveldb_options_t {
            self.ptr.as_ptr()
        }
    }

    impl Drop for Options {
        fn drop(&mut self) {
            unsafe { leveldb_options_destroy(self.ptr.as_ptr()) }
        }
    }

    impl Options {
        pub fn create() -> Options {
            unsafe {
                let ptr = leveldb_options_create();
                Options {
                    ptr: NonNull::new_unchecked(ptr),
                }
            }
        }

        pub fn create_if_missing(&mut self, value: bool) {
            unsafe { leveldb_options_set_create_if_missing(self.as_ptr(), value as u8) }
        }
    }

    pub struct WriteOptions {
        ptr: NonNull<leveldb_writeoptions_t>,
    }

    impl WriteOptions {
        pub fn new() -> WriteOptions {
            unsafe {
                let ptr = leveldb_writeoptions_create();
                WriteOptions {
                    ptr: NonNull::new_unchecked(ptr),
                }
            }
        }
    }

    impl Drop for WriteOptions {
        fn drop(&mut self) {
            unsafe { leveldb_writeoptions_destroy(self.ptr.as_ptr()) }
        }
    }

    pub struct ReadOptions {
        ptr: NonNull<leveldb_readoptions_t>,
    }

    impl ReadOptions {
        pub fn new() -> ReadOptions {
            unsafe {
                let ptr = leveldb_readoptions_create();
                ReadOptions {
                    ptr: NonNull::new_unchecked(ptr),
                }
            }
        }
    }

    impl Drop for ReadOptions {
        fn drop(&mut self) {
            unsafe { leveldb_readoptions_destroy(self.ptr.as_ptr()) }
        }
    }

    pub struct Database {
        handle: DBHandle,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Error {
        OpenFail,
        GetFail,
        PutFail,
        NonUtf8Path,
    }

    impl Database {
        pub fn open<P: AsRef<Path>>(path: P, options: Options) -> Result<Database, Error> {
            let mut error = ptr::null_mut();

            let c_string = CString::new(
                path.as_ref().to_str().ok_or(Error::NonUtf8Path)?
            ).map_err(|_| Error::NonUtf8Path)?;
            unsafe {
                let db = leveldb_open(options.as_ptr(), c_string.as_ptr(), &mut error);

                if error == ptr::null_mut() {
                    Ok(Database {
                        handle: DBHandle {
                            ptr: NonNull::new_unchecked(db),
                        },
                    })
                } else {
                    Err(Error::OpenFail)
                }
            }
        }

        pub fn get(&self, key: &[u8]) -> Result<Option<Box<[u8]>>, Error> {
            unsafe {
                let read_options = ReadOptions::new();
                let mut len: size_t = 0;
                let mut error = ptr::null_mut();

                let data = leveldb_get(
                    self.handle.ptr.as_ptr(),
                    read_options.ptr.as_ptr(),
                    key.as_ptr() as *const i8,
                    key.len(),
                    &mut len,
                    &mut error,
                );


                if error == ptr::null_mut() {
                    if data == ptr::null_mut() {
                        Ok(None)
                    } else {
                        let slice = std::slice::from_raw_parts(data as *mut u8, len);

                        let result = Box::from(slice);

                        leveldb_free(data as *mut c_void);

                        Ok(Some(result))
                    }
                } else {
                    leveldb_free(*error as *mut c_void);
                    Err(Error::GetFail)
                }
            }
        }

        pub fn put(&self, key: &[u8], data: &[u8]) -> Result<(), Error> {
            unsafe {
                let write_options = WriteOptions::new();
                let mut error = ptr::null_mut();

                leveldb_put(
                    self.handle.ptr.as_ptr(),
                    write_options.ptr.as_ptr(),
                    key.as_ptr() as *const i8,
                    key.len(),
                    data.as_ptr() as *const i8,
                    data.len(),
                    &mut error,
                );

                if error == ptr::null_mut() {
                    Ok(())
                } else {
                    leveldb_free(*error as *mut c_void);
                    Err(Error::PutFail)
                }
            }
        }

        pub fn iter(&self) -> Iterator<'_> {
            let read_options = ReadOptions::new();

            let handle = IteratorHandle::new(self, read_options);

            Iterator {
                handle: handle,
                start: true,
                database: self,
            }
        }
    }

    pub struct Iterator<'database> {
        handle: IteratorHandle,
        start: bool,
        #[allow(unused)]
        database: &'database Database,
    }

    impl<'database> Iterator<'database> {
        fn read_current(&self) -> Option<Box<[u8]>> {
            unsafe {
                if !self.handle.valid() {
                    return None;
                };

                let data = self.handle.value();

                let slice = std::slice::from_raw_parts(data.0 as *mut u8, data.1);

                Some(Box::from(slice))
            }
        }
    }

    impl<'database> std::iter::Iterator for Iterator<'database> {
        type Item = Box<[u8]>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.start {
                self.start = false;

                self.read_current()
            } else {
                self.handle.next();

                self.read_current()
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use tempdir::TempDir;

        #[test]
        fn test_open() {
            let tmp = TempDir::new("test_open").unwrap();

            let mut options = Options::create();
            options.create_if_missing(true);

            let database = Database::open(tmp.path().join("database"), options);
            assert!(database.is_ok());
        }

        #[test]
        fn test_read_write() {
            let tmp = TempDir::new("test_read_write").unwrap();

            let mut options = Options::create();
            options.create_if_missing(true);

            let database = Database::open(tmp.path().join("database"), options).unwrap();

            let key: &[u8] = b"test";
            let missing_key: &[u8] = b"test_missing";
            let value: &[u8] = b"test";

            database.put(key, value).unwrap();

            let result = database.get(key);
            assert_eq!(result, Ok(Some(Box::from(value))));

            let result = database.get(missing_key);
            assert_eq!(result, Ok(None));
        }

        #[test]
        fn test_iter() {
            let tmp = TempDir::new("test_iter").unwrap();

            let mut options = Options::create();
            options.create_if_missing(true);

            let database = Database::open(tmp.path().join("database"), options).unwrap();

            let key1: &[u8] = b"test1";
            let key2: &[u8] = b"test2";
            let key3: &[u8] = b"test3";

            let value1: &[u8] = b"value1";
            let value2: &[u8] = b"value2";
            let value3: &[u8] = b"value3";

            database.put(key1, value1).unwrap();
            database.put(key2, value2).unwrap();
            database.put(key3, value3).unwrap();

            let mut iter = database.iter();

            assert_eq!(iter.next(), Some(Box::from(value1)));
            assert_eq!(iter.next(), Some(Box::from(value2)));
            assert_eq!(iter.next(), Some(Box::from(value3)));
            assert_eq!(iter.next(), None);
        }
    }
