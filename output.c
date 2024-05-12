#include <stdio.h>
#include <stdlib.h>
#include "runtime.h"

typedef struct {
} ActorStage;


typedef struct {
	Value var_my_variable;
} ActorSprite1;

typedef struct {
	int state;
	float time;
	int loop_iYPjloWmMaNl;
} SequenceUmyRJogSROmmState;

void sequenceUmyRJogSROmm(ActorSprite1 *a, SequenceUmyRJogSROmmState *s, const GlobalState *g) {
	switch (s->state) {
	case 0: {
		if (g->flag_clicked) s->state = 1;
	}
	break;
	case 1: {
		Value VVJpbAqNNMfE = (Value){ .type = VALUE_NUM, .n = 10 };
		a->var_my_variable = VVJpbAqNNMfE; // setting my variable
		s->state = 2;
	}
	break;
	case 2: {
		Value l_TvdfozZufn = a->var_my_variable; // my variable
		if (l_TvdfozZufn.type != VALUE_NUM || (int)l_TvdfozZufn.n <= 0) s->state = 5;
		else s->loop_iYPjloWmMaNl = l_TvdfozZufn.n;
	}
	break;
	case 3: {
		printf("HEY, THIS ISN'T IMPLEMENTED YET ;)\n");
		s->state = 4;
	}
	break;
	case 4: {
		Value G_lloAIPPIJg = a->var_my_variable; // my variable
		Value PiNJvdBWEHYl = (Value){ .type = VALUE_NUM, .n = 1 };
		if (G_lloAIPPIJg.type != VALUE_NUM || PiNJvdBWEHYl.type != VALUE_NUM) {
			printf("WE DYING HERE");
			exit(-1);
		}
		G_lloAIPPIJg.n += PiNJvdBWEHYl.n;
		a->var_my_variable = G_lloAIPPIJg; // setting my variable
		s->state = 5;
	}
	break;
	case 5: {
		if (s->loop_iYPjloWmMaNl > 0) s->state = 3;
		s->state = 6;
	}
	break;
	}
}

