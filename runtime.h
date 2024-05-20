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
    int x;
    int y;
    int size;
    int direction;
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

static float scratch_degrees_to_radians(int direction) {
	return (-direction + 90) * PI / 180.0;
}

void draw_actor(ActorState *a);
