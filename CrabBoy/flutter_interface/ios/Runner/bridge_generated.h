#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_load_rom(int64_t port_,
                   struct wire_uint_8_list *rom_data,
                   struct wire_uint_8_list *ram_data);

void wire_unload_emulator(int64_t port_);

void wire_render_frame(int64_t port_);

void wire_set_buttons_state(int64_t port_, struct wire_uint_8_list *button_states);

void wire_load(int64_t port_, struct wire_uint_8_list *rom_data, struct wire_uint_8_list *ram_data);

void wire_unload(int64_t port_);

void wire_render(int64_t port_);

void wire_set_buttons(int64_t port_, struct wire_uint_8_list *button_states);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_load_rom);
    dummy_var ^= ((int64_t) (void*) wire_unload_emulator);
    dummy_var ^= ((int64_t) (void*) wire_render_frame);
    dummy_var ^= ((int64_t) (void*) wire_set_buttons_state);
    dummy_var ^= ((int64_t) (void*) wire_load);
    dummy_var ^= ((int64_t) (void*) wire_unload);
    dummy_var ^= ((int64_t) (void*) wire_render);
    dummy_var ^= ((int64_t) (void*) wire_set_buttons);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
