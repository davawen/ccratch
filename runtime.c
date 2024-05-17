#include <raylib.h>
#include "runtime.h"

void draw_actor(ActorState *a) {
    Sprite *sprite = &a->sprites[a->sprite_index];
    Rectangle source = { .x = 0, .y = 0, .width = sprite->texture.width, .height = sprite->texture.height };
    Rectangle dest = { .x = a->x, .y = a->y, .width = source.width * a->size / 100, .height = source.height * a->size / 100 };
    Vector2 origin = { sprite->rotation_center_x*source.width/100, sprite->rotation_center_y*source.height/100 };
    DrawTexturePro(sprite->texture, source, dest, origin, a->angle, WHITE);
}

int main() {
    InitWindow(1200, 900, "Hello, world!");
    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        BeginDrawing();

        ClearBackground(WHITE);
        DrawText("Hello, world!", 600, 400, 20, BLACK);

        EndDrawing();
    }
}
