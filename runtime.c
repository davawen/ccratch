#include <raylib.h>
#include "runtime.h"
#include "output.h"

void draw_actor(ActorState *a) {
    Sprite *sprite = &a->sprites[a->sprite_index];
    Rectangle source = { .x = 0, .y = 0, .width = sprite->texture.width, .height = sprite->texture.height };
    Rectangle dest = { .x = a->x + 240, .y = a->y + 180, .width = source.width * a->size / 100, .height = source.height * a->size / 100 };
    Vector2 origin = { sprite->rotation_center_x*source.width/100, sprite->rotation_center_y*source.height/100 };

	// scratch direction is in degrees
	// top is 0
	// goes clockwise

	// raylib direction is in degrees
	// right is 0
	// goes counter-clockwise

	// raylib_dir = -scratch_dir + 90

    DrawTexturePro(sprite->texture, source, dest, origin, -a->direction + 90, WHITE);
}

int main() {
    InitWindow(480, 360, "Hello, world!");
    SetTargetFPS(60);

	GlobalState g = init_global();

    while (!WindowShouldClose()) {
		if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
			Vector2 p = GetMousePosition();
			Vector2 dist = { p.x - 15, p.y - 15 };
			if (dist.x*dist.x + dist.y*dist.y < 10*10) {
				g.flag_clicked = true;
				printf("started!\n");
			}
		} else {
			g.flag_clicked = false;
		}

		run_global(&g);

        BeginDrawing();

        ClearBackground(WHITE);
		render_global(&g);

		DrawCircle(15, 15, 10, GREEN);

        EndDrawing();
    }
}
