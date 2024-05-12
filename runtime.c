#include <raylib.h>
#include "runtime.h"

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
