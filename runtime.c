#include <raylib.h>
#include "runtime.h"
#include "output.h"

void draw_actor(ActorState *a) {
    Sprite *sprite = &a->sprites[a->sprite_index];
    Rectangle source = { .x = 0, .y = 0, .width = sprite->texture.width, .height = sprite->texture.height };
    Rectangle dest = { .x = a->x + 240, .y = -a->y + 180, .width = source.width * a->size / 100, .height = source.height * a->size / 100 };
    Vector2 origin = { sprite->rotation_center_x*source.width/100, sprite->rotation_center_y*source.height/100 };

	// scratch direction is in degrees
	// top is 0
	// goes clockwise

	// raylib direction is in degrees
	// right is 0
	// goes counter-clockwise

	// raylib_dir = -scratch_dir + 90

    DrawTexturePro(sprite->texture, source, dest, origin, -a->direction + 90, WHITE);

	// readjust `dest` based on origin (so it lines up with the texture drawn above)
	dest.x -= sprite->rotation_center_x*dest.width/100.0;
	dest.y -= sprite->rotation_center_y*dest.height/100.0;

	if (a->saying != NULL) {
		float text_x = dest.x + dest.width + 10.0;
		float text_y = dest.y - 30.0;

		float width = MeasureText(a->saying, 20);
		DrawRectangle(text_x - 5, text_y - 5, width + 10, 30, LIGHTGRAY);

		DrawText(a->saying, text_x, text_y, 20, BLACK);

		if (GetTime() > a->say_end) {
			if (a->say_should_free) free(a->saying);
			a->saying = NULL;
			a->say_end = INFINITY;
		}
	}
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
