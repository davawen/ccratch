#include <inttypes.h>
#include <stdbool.h>

struct ActorState {
    int x;
    int y;
    int size;
    int direction;
    int sprite_index;
};

enum ValueType {
    VALUE_NUM,
    VALUE_COLOR,
    VALUE_STRING
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
    };
} Value;

typedef struct {
    bool flag_clicked;
} GlobalState;
