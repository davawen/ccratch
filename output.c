#include <stdio.h>
#include <stdlib.h>
#include "runtime.h"

typedef struct {
	bool flag_clicked;
} GlobalState;
typedef struct {
} ActorStage;


typedef struct {
	Value var_my_variable;
} ActorSprite1;

typedef struct {
	int state;
	float time;
	int loop_dbVBQmjlGE_J;
} SequenceUBblFnKXFcQPState;

void sequenceUBblFnKXFcQP(ActorSprite1 *a, SequenceUBblFnKXFcQPState *s, const GlobalState *g) {
	switch (s->state) {
	case 0: {
		if (g->flag_clicked) s->state = 1;
	}
	break;
	case 1: {
		Value iACxNGZNDHKC = (Value){ .type = VALUE_NUM, .n = 10 };
		a->var_my_variable = iACxNGZNDHKC; // setting my variable
		s->state = 2;
	}
	break;
	case 2: {
		Value KCICwsOaxAoc = a->var_my_variable; // my variable
		float DWujBoOzQaWl = 0.0;
		if (KCICwsOaxAoc.type == VALUE_NUM) DWujBoOzQaWl = KCICwsOaxAoc.n;
		else if (KCICwsOaxAoc.type == VALUE_STRING) {
			char *end;
			DWujBoOzQaWl = strtof(KCICwsOaxAoc.s, &end);
			if (*end != '\0') DWujBoOzQaWl = 0.0;
		}
		if ((int)DWujBoOzQaWl <= 0) s->state = 6;
		else {
			s->state = 3;
			s->loop_dbVBQmjlGE_J = DWujBoOzQaWl;
		}
	}
	break;
	case 3: {
		Value UavBtZSVJEZh = a->var_my_variable; // my variable
		if (UavBtZSVJEZh.type == VALUE_NUM) printf("%f\n", UavBtZSVJEZh.n);
		else if (UavBtZSVJEZh.type == VALUE_STRING) printf("%s\n", UavBtZSVJEZh.s);
		else if (UavBtZSVJEZh.type == VALUE_COLOR) printf("#%02X%02X%02X\n", UavBtZSVJEZh.c.r, UavBtZSVJEZh.c.g, UavBtZSVJEZh.c.b);
		s->state = 4;
	}
	break;
	case 4: {
		Value HTKaDJaYmWPo = a->var_my_variable; // my variable
		Value KsHaN_fONDqR = (Value){ .type = VALUE_NUM, .n = 1 };
		float ujsUonBGOgLw = 0.0;
		if (HTKaDJaYmWPo.type == VALUE_NUM) ujsUonBGOgLw = HTKaDJaYmWPo.n;
		else if (HTKaDJaYmWPo.type == VALUE_STRING) {
			char *end;
			ujsUonBGOgLw = strtof(HTKaDJaYmWPo.s, &end);
			if (*end != '\0') ujsUonBGOgLw = 0.0;
		}
		float OSrXlOFQGImB = 0.0;
		if (KsHaN_fONDqR.type == VALUE_NUM) OSrXlOFQGImB = KsHaN_fONDqR.n;
		else if (KsHaN_fONDqR.type == VALUE_STRING) {
			char *end;
			OSrXlOFQGImB = strtof(KsHaN_fONDqR.s, &end);
			if (*end != '\0') OSrXlOFQGImB = 0.0;
		}
		HTKaDJaYmWPo.type = VALUE_NUM;
		HTKaDJaYmWPo.n = ujsUonBGOgLw + OSrXlOFQGImB;
		a->var_my_variable = HTKaDJaYmWPo; // setting my variable
		s->state = 5;
	}
	break;
	case 5: {
		s->loop_dbVBQmjlGE_J--;
		if (s->loop_dbVBQmjlGE_J > 0) s->state = 3;
		else s->state = 6;
	}
	break;
	}
}
