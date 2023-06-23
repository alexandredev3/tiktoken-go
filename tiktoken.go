//go:build !windows

package tiktoken

//go:generate bash -c "cd tiktokenrs && cargo build --release"

// #cgo linux LDFLAGS: ${SRCDIR}/tiktokenrs/target/release/libtiktoken.a -ldl
// #cgo darwin LDFLAGS: ${SRCDIR}/tiktokenrs/target/release/libtiktoken.a -framework Security -framework CoreFoundation
// extern char *hello_to_my_name(const char*);
// extern unsigned int get_qtd_tokens(const char*, const char*);
// #include <stdio.h>
// #include <errno.h>
// #include <stdlib.h>
import "C"
import "unsafe"

func HelloToMyName(name string) string {
	n := C.CString(name)
	result := C.hello_to_my_name(n) // We're getting the function hello_to_my_name from the Rust lib
	goString := C.GoString(result)
	C.free(unsafe.Pointer(n)) // We need to free the memory manually because we're using the unsafe.
	C.free(unsafe.Pointer(result))

	return goString
}

func CountTokens(model_name, txt string) int {
	t := C.CString(txt)
	m := C.CString(model_name)
	count := C.get_qtd_tokens(m, t)
	C.free(unsafe.Pointer(t))
	C.free(unsafe.Pointer(m))

	return int(count)
}
