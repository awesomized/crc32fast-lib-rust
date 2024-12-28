#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Opaque type for C for use in FFI
struct HasherHandle {
  Hasher *_0;
};

extern "C" {

/// Creates a new Hasher to compute CRC32 checksums
HasherHandle *hasher_new();

/// Writes data to the Hasher
///
/// # Safety
///
/// Uses unsafe method calls
void hasher_write(HasherHandle *handle, const char *data, uintptr_t len);

/// Calculates the CRC32 checksum for data that's been written to the Hasher
///
/// # Safety
///
/// Uses unsafe method calls
uint32_t hasher_finalize(HasherHandle *handle);

/// Helper method to just calculate a CRC32 checksum directly for a string
uint32_t crc32_hash(const char *data, uintptr_t len);

}  // extern "C"
