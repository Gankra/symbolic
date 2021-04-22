#include "cpp/symbolic_cfi_frame_info.h"

#include "cpp/data_structures.h"
#include "google_breakpad/processor/minidump.h"

extern "C" {
void cfi_frame_info_free(void *cfi_frame_info);
regval_t *find_caller_regs_32(void *cfi_frame_info,
                              uint64_t memory_base,
                              size_t memory_len,
                              const void *memory_bytes,
                              const regval_t *registers,
                              size_t registers_len,
                              size_t *caller_registers_len_out);
regval_t *find_caller_regs_64(void *cfi_frame_info,
                              uint64_t memory_base,
                              size_t memory_len,
                              const void *memory_bytes,
                              const regval_t *registers,
                              size_t registers_len,
                              size_t *caller_registers_len_out);
void regvals_free(regval_t *reg_vals, size_t len);
}

SymbolicCFIFrameInfo::SymbolicCFIFrameInfo(void *cfi_frame_info) {
    cfi_frame_info_ = cfi_frame_info;
}

SymbolicCFIFrameInfo::~SymbolicCFIFrameInfo() {
    cfi_frame_info_free(cfi_frame_info_);
}

// Provide implementations for CFIFrameInfo to change FindCallerRegs, which
// cannot be declared virtual.
namespace google_breakpad {

// Dummy generic implementation
template <typename V>
bool CFIFrameInfo::FindCallerRegs(const RegisterValueMap<V> &registers,
                                  const MemoryRegion &memory,
                                  RegisterValueMap<V> *caller_registers) const {
    return false;
};

// Explicit specializations for 32-bit and 64-bit architectures.
template <>
bool CFIFrameInfo::FindCallerRegs<uint32_t>(
    const RegisterValueMap<uint32_t> &registers,
    const MemoryRegion &memory,
    RegisterValueMap<uint32_t> *caller_registers) const {
    caller_registers->clear();

    void *cfi_frame_info =
        dynamic_cast<const SymbolicCFIFrameInfo *>(this)->cfi_frame_info_;
    const MinidumpMemoryRegion *minidump_memory =
        static_cast<const MinidumpMemoryRegion *>(&memory);
    std::vector<regval_t> register_vec;
    for (typename RegisterValueMap<uint32_t>::const_iterator it =
             registers.begin();
         it != registers.end(); it++) {
        regval_t regval = {it->first.c_str(), it->second, 4};
        register_vec.push_back(regval);
    }
    size_t caller_registers_size = 0;
    regval_t *caller_registers_vec = find_caller_regs_32(
        cfi_frame_info, minidump_memory->GetBase(), minidump_memory->GetSize(),
        minidump_memory->GetMemory(), register_vec.data(), register_vec.size(),
        &caller_registers_size);
    if (caller_registers_vec == NULL) {
        return false;
    }

    for (size_t i = 0; i < caller_registers_size; i++) {
        regval_t reg = caller_registers_vec[i];
        (*caller_registers)[reg.name] = reg.value;
    }

    regvals_free(caller_registers_vec, caller_registers_size);
    return true;
}

template <>
bool CFIFrameInfo::FindCallerRegs<uint64_t>(
    const RegisterValueMap<uint64_t> &registers,
    const MemoryRegion &memory,
    RegisterValueMap<uint64_t> *caller_registers) const {
    caller_registers->clear();

    void *cfi_frame_info =
        dynamic_cast<const SymbolicCFIFrameInfo *>(this)->cfi_frame_info_;
    const MinidumpMemoryRegion *minidump_memory =
        static_cast<const MinidumpMemoryRegion *>(&memory);
    std::vector<regval_t> register_vec;
    for (typename RegisterValueMap<uint64_t>::const_iterator it =
             registers.begin();
         it != registers.end(); it++) {
        regval_t regval = {it->first.c_str(), it->second, 8};
        register_vec.push_back(regval);
    }
    size_t caller_registers_size = 0;
    regval_t *caller_registers_vec = find_caller_regs_64(
        cfi_frame_info, minidump_memory->GetBase(), minidump_memory->GetSize(),
        minidump_memory->GetMemory(), register_vec.data(), register_vec.size(),
        &caller_registers_size);
    if (caller_registers_vec == NULL) {
        return false;
    }

    for (size_t i = 0; i < caller_registers_size; i++) {
        regval_t reg = caller_registers_vec[i];
        (*caller_registers)[reg.name] = reg.value;
    }

    regvals_free(caller_registers_vec, caller_registers_size);
    return true;
}

// noop implementations for unused functions

bool CFIRuleParser::Parse(const string &rule_set) {
    return false;
}
void CFIFrameInfoParseHandler::CFARule(const string &expression) {
}
void CFIFrameInfoParseHandler::RARule(const string &expression) {
}
void CFIFrameInfoParseHandler::RegisterRule(const string &name,
                                            const string &expression) {
}

}  // namespace google_breakpad
