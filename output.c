#include <stdio.h>
#include "runtime.h"

typedef struct {
	bool flag_clicked;
	Value var_my_variable;
} GlobalState;
typedef struct {
} ActorStage;


typedef struct {
} ActorSprite1;

typedef struct {
	int state;
	float time;
	int loop_lvqvgXqhnicz;
} SequenceFuwPsKY_KJcxState;

void sequenceFuwPsKY_KJcx(ActorSprite1 *a, SequenceFuwPsKY_KJcxState *s, GlobalState *g) {
	switch (s->state) {
	case 0: {
		if (g->flag_clicked) s->state = 1;
	}
	break;
	case 1: {
		Value tkVgClOdOVGK = (Value){ .type = VALUE_NUM, .n = 10 };
		g->var_my_variable = tkVgClOdOVGK; // setting my variable
		s->state = 2;
	}
	break;
	case 2: {
		Value AIXarmrhOqVC = g->var_my_variable; // my variable
		AIXarmrhOqVC.n = value_as_number(AIXarmrhOqVC);
		AIXarmrhOqVC.type = VALUE_NUM;
		if ((int)AIXarmrhOqVC.n <= 0) s->state = 6;
		else {
			s->state = 3;
			s->loop_lvqvgXqhnicz = AIXarmrhOqVC.n;
		}
	}
	break;
	case 3: {
		Value kOMsZjqCxezh = g->var_my_variable; // my variable
		if (kOMsZjqCxezh.type == VALUE_NUM) printf("%f\n", kOMsZjqCxezh.n);
		else if (kOMsZjqCxezh.type == VALUE_STRING) printf("%s\n", kOMsZjqCxezh.s);
		else if (kOMsZjqCxezh.type == VALUE_COLOR) printf("#%02X%02X%02X\n", kOMsZjqCxezh.c.r, kOMsZjqCxezh.c.g, kOMsZjqCxezh.c.b);
		s->state = 4;
	}
	break;
	case 4: {
		Value qMKOQmYmyiTV = g->var_my_variable; // my variable
		Value duNxcOmGxNrc = (Value){ .type = VALUE_NUM, .n = 1 };
		qMKOQmYmyiTV.n = value_as_number(qMKOQmYmyiTV) - value_as_number(duNxcOmGxNrc);
		qMKOQmYmyiTV.type = VALUE_NUM;
		g->var_my_variable = qMKOQmYmyiTV; // setting my variable
		s->state = 5;
	}
	break;
	case 5: {
		s->loop_lvqvgXqhnicz--;
		if (s->loop_lvqvgXqhnicz > 0) s->state = 3;
		else s->state = 6;
	}
	break;
	}
}

