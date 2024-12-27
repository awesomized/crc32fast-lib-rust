#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct HasherHandle {
  Hasher *_0;
};

extern "C" {

HasherHandle *hasher_new();

/// # Safety
///
/// Uses unsafe method calls
void hasher_write(HasherHandle *handle, const char *data, uintptr_t len);

/// # Safety
///
/// Uses unsafe method calls
uint32_t hasher_finalize(HasherHandle *handle);

uint32_t crc32_hash(const char *data, uintptr_t len);

}  // extern "C"
