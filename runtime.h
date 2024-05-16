#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>

#include <raylib.h>

typedef struct {
    int x;
    int y;
    int size;
    int angle;
    int sprite_index;
	Texture *sprites;
} ActorState;

enum ValueType {
    VALUE_NUM,
    VALUE_COLOR,
    VALUE_STRING,
	VALUE_BOOL
};

typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} ValueColor;

typedef struct {
    enum ValueType type;
    union {
        float n;
        ValueColor c;
        char *s;
		bool b;
    };
} Value;

static float value_as_number(Value v) {
	if (v.type == VALUE_NUM) return v.n;
	else if (v.type == VALUE_BOOL) return v.b;
	else if (v.type == VALUE_STRING) {
		char *end;
		float n = strtof(v.s, &end);
		if (*end == '\0') return n;
	}
	return 0.0;
}

static bool value_as_bool(Value v) {
	if (v.type == VALUE_BOOL) return v.b;
	else if (v.type == VALUE_NUM) return v.n != 0;
	else if (v.type == VALUE_STRING) {
		return strcmp(v.s, "true") == 0;
	}
	return false;
}

static void convert_to_number(Value *v) {
	v->n = value_as_number(*v);
	v->type = VALUE_NUM;
}

static void convert_to_bool(Value *v) {
	v->b = value_as_bool(*v);
	v->type = VALUE_BOOL;
}
