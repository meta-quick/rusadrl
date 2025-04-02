package odrl

import "C"

// #cgo LDFLAGS: -L ../../../ffi/libs/ -ladrlcffi
// #include "../../../ffi/include/odrl.h"
import "C"
import "unsafe"

type Engine struct {
	handle *C.int64_t
}

func NewEngine(verbose bool, jsonld string) *Engine {
	e := new(Engine)
	json := C.CString(jsonld)
	defer C.free(unsafe.Pointer(json))

	if verbose {
		C.enable_verbose(1)
	} else {
		C.enable_verbose(0)
	}

	e.handle = C.create_odrl_world(json)

	if e.handle == nil {
		return nil
	}

	return e
}

func (e *Engine) Close() {
	if e.handle == nil {
		return
	}
	C.delete_odrl_world(e.handle)
	e.handle = nil
}

func (e *Engine) Eval(action string, target string, assigner string, assignee string) int32 {
	if e.handle == nil {
		return -1
	}

	act := C.CString(action)
	tar := C.CString(target)
	as := C.CString(assigner)
	ae := C.CString(assignee)

	defer func() {
		C.free(unsafe.Pointer(act))
		C.free(unsafe.Pointer(tar))
		C.free(unsafe.Pointer(as))
		C.free(unsafe.Pointer(ae))
	}()

	return int32(C.eval_odrl_world(e.handle, act, tar, as, ae))
}

func (e *Engine) UpdateWorld(key string, value string) int32 {
	if e.handle == nil {
		return -1
	}

	k := C.CString(key)
	v := C.CString(value)
	defer func() {
		C.free(unsafe.Pointer(k))
		C.free(unsafe.Pointer(v))
	}()

	return int32(C.update_odrl_world(e.handle, k, v))
}

func (e *Engine) FetchWorld(key string) string {
	if e.handle == nil {
		return ""
	}

	k := C.CString(key)
	defer C.free(unsafe.Pointer(k))

	return C.GoString(C.fetch_odrl_world(e.handle, k))
}

func (e *Engine) RemoveWorld(key string) int32 {
	if e.handle == nil {
		return -1
	}

	k := C.CString(key)
	defer C.free(unsafe.Pointer(k))

	return int32(C.remove_odrl_world(e.handle, k))
}
