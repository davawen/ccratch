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
} SequencearkBCJyMkoHEState;

void sequencearkBCJyMkoHE(ActorSprite1 *a, SequencearkBCJyMkoHEState *s, GlobalState *g) {
	switch (s->state) {
	case 0: {
		if (g->flag_clicked) s->state = 1;
	}
	break;
	case 1: {
		Value oMYbeZXsprWW = (Value){ .type = VALUE_NUM, .n = 10 };
		g->var_my_variable = oMYbeZXsprWW; // setting my variable
		s->state = 2;
	}
	break;
	case 2: {
		Value DjPM_KQDclwP = (Value){ .type = VALUE_NUM, .n = 1 };
		Value OvNACUgkEsIz = (Value){ .type = VALUE_NUM, .n = 0 };
		DjPM_KQDclwP.b = value_as_number(DjPM_KQDclwP) > value_as_number(OvNACUgkEsIz);
		DjPM_KQDclwP.type = VALUE_BOOL;
		convert_to_bool(&DjPM_KQDclwP);
		if (DjPM_KQDclwP.b) s->state = 3;
		else s->state = 7;
	}
	break;
	case 3: {
		Value VoYXrtTISXrj = (Value){ .type = VALUE_NUM, .n = 19 };
		Value VmcsnwlQFVxb = (Value){ .type = VALUE_NUM, .n = 20 };
		VoYXrtTISXrj.b = value_as_number(VoYXrtTISXrj) == value_as_number(VmcsnwlQFVxb);
		VoYXrtTISXrj.type = VALUE_BOOL;
		convert_to_bool(&VoYXrtTISXrj);
		VoYXrtTISXrj.b = !VoYXrtTISXrj.b;
		Value KdFoldtkIYPG = g->var_my_variable; // my variable
		Value xgHCvpid_qVl = (Value){ .type = VALUE_NUM, .n = 5 };
		KdFoldtkIYPG.b = value_as_number(KdFoldtkIYPG) > value_as_number(xgHCvpid_qVl);
		KdFoldtkIYPG.type = VALUE_BOOL;
		VoYXrtTISXrj.b = value_as_bool(VoYXrtTISXrj) && value_as_bool(KdFoldtkIYPG);
		VoYXrtTISXrj.type = VALUE_BOOL;
		g->var_my_variable = VoYXrtTISXrj; // setting my variable
		s->state = 4;
	}
	break;
	case 4: {
		Value WIJgZTzsmWE_ = g->var_my_variable; // my variable
		Value eooPFnNlUBqt = (Value){ .type = VALUE_NUM, .n = 1 };
		WIJgZTzsmWE_.n = value_as_number(WIJgZTzsmWE_) + value_as_number(eooPFnNlUBqt);
		WIJgZTzsmWE_.type = VALUE_NUM;
		g->var_my_variable = WIJgZTzsmWE_; // setting my variable
		s->state = 5;
	}
	break;
	case 5: {
		Value AwfAmvXaRpdC = g->var_my_variable; // my variable
		if (AwfAmvXaRpdC.type == VALUE_NUM) printf("%f\n", AwfAmvXaRpdC.n);
		else if (AwfAmvXaRpdC.type == VALUE_STRING) printf("%s\n", AwfAmvXaRpdC.s);
		else if (AwfAmvXaRpdC.type == VALUE_COLOR) printf("#%02X%02X%02X\n", AwfAmvXaRpdC.c.r, AwfAmvXaRpdC.c.g, AwfAmvXaRpdC.c.b);
		else if (AwfAmvXaRpdC.type == VALUE_BOOL) {
			if (AwfAmvXaRpdC.b) printf("true\n");
			else printf("false\n");
		}
		s->state = 6;
	}
	break;
	case 6: {
		s->state = 7;
	}
	break;
	}
}

