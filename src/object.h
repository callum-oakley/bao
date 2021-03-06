#ifndef bao_object_h
#define bao_object_h

#include "common.h"
#include "chunk.h"
#include "value.h"

#define OBJ_TYPE(value)   (AS_OBJ(value)->type)

#define IS_FUNCTION(value)  is_obj_type(value, OBJ_FUNCTION)
#define IS_STRING(value)  is_obj_type(value, OBJ_STRING)

#define AS_FUNCTION(value)  ((ObjFunction*)AS_OBJ(value))
#define AS_STRING(value)  ((ObjString*)AS_OBJ(value))
#define AS_CSTRING(value) (((ObjString*)AS_OBJ(value))->chars)

typedef enum {
    OBJ_FUNCTION,
    OBJ_STRING,
} ObjType;

struct Obj {
    ObjType type;
    struct Obj* next;
};

typedef struct {
    Obj obj;
    int arity;
    Chunk chunk;
    ObjString* name;
} ObjFunction;

struct ObjString {
    Obj obj;
    int length;
    // TODO store chars as a flexible array member to save a dereference
    char* chars;
    uint32_t hash;
};

ObjFunction* new_function();
ObjString* copy_string(const char* chars, int length);
ObjString* take_string(char* chars, int length);
void print_object(Value value);

static inline bool is_obj_type(Value value, ObjType type) {
    return IS_OBJ(value) && AS_OBJ(value)->type == type;
}

#endif
