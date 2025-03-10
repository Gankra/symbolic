//! C++ Itanium Demangling Tests
//! We use cpp_demangle under the hood which runs the libiberty test suite
//! Still, we run some basic regression tests here to detect demangling differences.

#![cfg(feature = "cpp")]

#[macro_use]
mod utils;

use symbolic_common::Language;
use symbolic_demangle::DemangleOptions;

#[test]
fn test_demangle_cpp() {
    assert_demangle!(Language::Cpp, DemangleOptions::name_only().parameters(true), {
        "_Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE" => "JS_GetPropertyDescriptorById(JSContext*, JS::Handle<JSObject*>, JS::Handle<jsid>, JS::MutableHandle<JS::PropertyDescriptor>)",
        "_ZN12_GLOBAL__N_15startEv" => "(anonymous namespace)::start()",
        "__ZN12_GLOBAL__N_15startEv" => "(anonymous namespace)::start()",
        "_ZZN12_GLOBAL__N_15helloEvENK3$_0clEv" => "(anonymous namespace)::hello()::$_0::operator()() const",
        "_Z3MinIiiEDTqultfp_fp0_cl7forwardIT_Efp_Ecl7forwardIT0_Efp0_EEOS0_OS1_" => "Min<int, int>(int&&, int&&)",
        "___ZN19URLConnectionClient33_clientInterface_cancelConnectionEP16dispatch_queue_sU13block_pointerFvvE_block_invoke14" => "invocation function for block in URLConnectionClient::_clientInterface_cancelConnection(dispatch_queue_s*, void () block_pointer)",

        // Broken in cpp_demangle
        // "_ZN4base8internal13FunctorTraitsIPFvvEvE6InvokeIJEEEvS3_DpOT_" => "void base::internal::FunctorTraits<void (*)(), void>::Invoke<>(void (*)())",
    });
}

#[test]
fn test_demangle_cpp_no_args() {
    assert_demangle!(Language::Cpp, DemangleOptions::name_only(), {
        "_Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE" => "JS_GetPropertyDescriptorById",
        "_ZN12_GLOBAL__N_15startEv" => "(anonymous namespace)::start",
        "_ZZN12_GLOBAL__N_15helloEvENK3$_0clEv" => "(anonymous namespace)::hello()::$_0::operator() const",
        "___ZN19URLConnectionClient33_clientInterface_cancelConnectionEP16dispatch_queue_sU13block_pointerFvvE_block_invoke14" => "invocation function for block in URLConnectionClient::_clientInterface_cancelConnection",

        // Broken in cpp_demangle
        // "_ZN4base8internal13FunctorTraitsIPFvvEvE6InvokeIJEEEvS3_DpOT_" => "void base::internal::FunctorTraits<void (*)(), void>::Invoke<>",
        // "_Z3MinIiiEDTqultfp_fp0_cl7forwardIT_Efp_Ecl7forwardIT0_Efp0_EEOS0_OS1_" => "decltype (({parm#1}<{parm#2})?((forward<int>)({parm#1})) : ((forward<int>)({parm#2}))) Min<int, int>",
    });
}

#[test]
fn test_demangle_cpp_hash_suffix() {
    assert_demangle!(Language::Cpp, DemangleOptions::complete(), {
    "__ZZN3xxx12xxxxxxxxxxxx9xxxxxxxxxILNS0_16xxxxxxxxxxxxxxxxE0EZNKS_6xxxxxx16xxxxxxxxxxxxxxxxEPjbbE4$_76EEvRKT0_PS3_PNS_7xxxxxxxENS0_13xxxxxxxxxxxxxEbbEN18xxxxxxxxxxxxxxxxxx10xxxxxxxxxxEv$57c34bde3fedbd1a4bf6fbbe5453ff24" =>
    "void xxx::xxxxxxxxxxxx::xxxxxxxxx<(xxx::xxxxxxxxxxxx::xxxxxxxxxxxxxxxx)0, xxx::xxxxxx::xxxxxxxxxxxxxxxx(unsigned int*, bool, bool) const::$_76>(xxx::xxxxxx::xxxxxxxxxxxxxxxx(unsigned int*, bool, bool) const::$_76 const&, xxx::xxxxxx*, xxx::xxxxxxx*, xxx::xxxxxxxxxxxx::xxxxxxxxxxxxx, bool, bool)::xxxxxxxxxxxxxxxxxx::xxxxxxxxxx()"
    });
}
