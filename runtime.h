#pragma once

#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>

#include <raylib.h>

typedef struct {
	/// the allocated buffer for the string (holds both the reference count and the string)
	void *buf;
	/// the reference count
	int *rc;
	/// how many pointers exist to this string
	char *ptr;
} rcstr;

typedef struct {
    int rotation_center_x;
    int rotation_center_y;
    Texture texture;
} Sprite;

typedef struct {
    float x;
    float y;
    float size;
    float direction;
	bool visible;
    int sprite_index;
	Sprite *sprites;

	/// wether this actor is currently saying something (rcstr.ptr should be NULL if not)
	rcstr saying;
	/// time at which the text should be removed
	float say_end;
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
        rcstr s;
		bool b;
    };
} Value;


/// allocates space for a ref counted string of length `len`.
/// adds an extra byte for the null terminator.
/// null terminated by default.
static rcstr alloc_rcstr(size_t len) {
	void *buf = malloc(sizeof(int) + len + 1);
	rcstr ret = {
		.buf = buf,
		.rc = (int *)buf,
		.ptr = (char *)buf + sizeof(int)
	};
	*ret.rc = 1;
	*ret.ptr = '\0';
	return ret;
}

static rcstr create_rcstr(const char *s) {
	rcstr rcs = alloc_rcstr(strlen(s));
	strcpy(rcs.ptr, s);
	return rcs;
}

static rcstr copy_rcstr(rcstr s) {
	*s.rc += 1;
	return s;
}

/// removes one from an rcstr's reference count
/// free's it if it hit zero
static void free_rcstr(rcstr s) {
	*s.rc -= 1;
	if (*s.rc <= 0) {
		free(s.buf);
	}
}

void convert_to_number(Value *v);
void convert_to_bool(Value *v);
void convert_to_rcstr(Value *v);

static inline Value copy_value(Value v) {
	if (v.type == VALUE_STRING) {
		v.s = copy_rcstr(v.s);
	}
	return v;
}

static inline void free_value(Value v) {
	if (v.type == VALUE_STRING) {
		free_rcstr(v.s);
	}
}

// define operations on values
#define M_VALUE_ARITHMETIC_OP(opname, op) \
	static inline Value value_##opname(Value a, Value b) { \
		convert_to_number(&a); \
		convert_to_number(&b); \
		a.n = a.n op b.n; \
		return a; \
	}

M_VALUE_ARITHMETIC_OP(add, +);
M_VALUE_ARITHMETIC_OP(sub, -);
M_VALUE_ARITHMETIC_OP(mul, *);
M_VALUE_ARITHMETIC_OP(div, /);

#undef M_VALUE_ARITHMETIC_OP

#define M_VALUE_LOGIC_OP(opname, op) \
	static inline Value value_##opname(Value a, Value b) { \
		convert_to_bool(&a);\
		convert_to_bool(&b);\
		a.b = a.b op b.b; \
		return a; \
	}

M_VALUE_LOGIC_OP(and, &&);
M_VALUE_LOGIC_OP(or, ||);

static inline Value value_not(Value v) {
	convert_to_bool(&v);
	v.b = !v.b;
	return v;
}

#undef M_VALUE_LOGIC_OP

#define M_VALUE_COMPARISON_OP(opname, op) \
	static inline Value value_##opname(Value a, Value b) { \
		if (a.type == VALUE_STRING || b.type == VALUE_STRING) { \
			a = copy_value(a); \
			b = copy_value(b); \
			convert_to_rcstr(&a); \
			convert_to_rcstr(&b); \
			bool res = strcmp(a.s.ptr, b.s.ptr) op 0; \
			free_value(a); \
			free_value(b); \
			a.b = res; \
		} else { \
			convert_to_number(&a); \
			convert_to_number(&b); \
			a.b = a.n op b.n; \
		} \
		a.type = VALUE_BOOL; \
		return a; \
	}

M_VALUE_COMPARISON_OP(greater_than, >);
M_VALUE_COMPARISON_OP(lesser_than, <);
M_VALUE_COMPARISON_OP(equal, ==);

#undef M_VALUE_COMPARISON_OP

static float scratch_degrees_to_radians(int direction) {
	return (-direction + 90) * PI / 180.0;
}

void draw_actor(ActorState *a);
