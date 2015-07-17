package main

import "fmt"

/*
#cgo LDFLAGS: -L./target/release -lhello -lm -ldl
void hello_from_rust(char *name);
*/
import "C"

//export HelloFromGo
func HelloFromGo(name *C.char) {
	fmt.Println("rust\t:", C.GoString(name))
}

func main() {
	C.hello_from_rust(C.CString("Привет Rust!"))
}
