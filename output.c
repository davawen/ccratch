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
} SequenceIyahUXbaLYKWState;

void sequenceIyahUXbaLYKW(ActorSprite1 *a, SequenceIyahUXbaLYKWState *s, GlobalState *g) {
	switch (s->state) {
	case 0: {
		// Starting WhenFlagClicked
		if (g->flag_clicked) s->state = 1;
	}
	break;
	case 1: {
		// Starting SetVariableTo { value: Number(10.0), var: Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" } }
		Value JcRAUDLQrnnm = (Value){ .type = VALUE_NUM, .n = 10 };
		g->var_my_variable = JcRAUDLQrnnm; // setting my variable
		s->state = 2;
	}
	break;
	case 2: {
		// Starting IfCondition { condition: Block(GreaterThan { lhs: Number(1.0), rhs: Number(0.0) }), branch: Sequence([SetVariableTo { value: Block(And { lhs: Block(Not { operand: Block(Equals { lhs: Number(19.0), rhs: Number(20.0) }) }), rhs: Block(GreaterThan { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(5.0) }) }), var: Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" } }, SetVariableTo { value: Block(Add { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(1.0) }), var: Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" } }, SayForSecs { message: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), secs: Number(2.0) }]) }
		// Starting GreaterThan { lhs: Number(1.0), rhs: Number(0.0) }
		Value lNSOKchAKpXJ = (Value){ .type = VALUE_NUM, .n = 1 };
		Value WHtZvWyElpzR = (Value){ .type = VALUE_NUM, .n = 0 };
		lNSOKchAKpXJ.b = value_as_number(lNSOKchAKpXJ) > value_as_number(WHtZvWyElpzR);
		lNSOKchAKpXJ.type = VALUE_BOOL;
		convert_to_bool(&lNSOKchAKpXJ);
		if (lNSOKchAKpXJ.b) s->state = 3;
		else s->state = 7;
	}
	break;
	case 3: {
		// Starting SetVariableTo { value: Block(And { lhs: Block(Not { operand: Block(Equals { lhs: Number(19.0), rhs: Number(20.0) }) }), rhs: Block(GreaterThan { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(5.0) }) }), var: Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" } }
		// Starting And { lhs: Block(Not { operand: Block(Equals { lhs: Number(19.0), rhs: Number(20.0) }) }), rhs: Block(GreaterThan { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(5.0) }) }
		// Starting Not { operand: Block(Equals { lhs: Number(19.0), rhs: Number(20.0) }) }
		// Starting Equals { lhs: Number(19.0), rhs: Number(20.0) }
		Value guVNHuDDXkDi = (Value){ .type = VALUE_NUM, .n = 19 };
		Value YszCPaTmbzKH = (Value){ .type = VALUE_NUM, .n = 20 };
		guVNHuDDXkDi.b = value_as_number(guVNHuDDXkDi) == value_as_number(YszCPaTmbzKH);
		guVNHuDDXkDi.type = VALUE_BOOL;
		convert_to_bool(&guVNHuDDXkDi);
		guVNHuDDXkDi.b = !guVNHuDDXkDi.b;
		// Starting GreaterThan { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(5.0) }
		Value YYpkbUXlqyxw = g->var_my_variable; // my variable
		Value stfSDSgHQqlz = (Value){ .type = VALUE_NUM, .n = 5 };
		YYpkbUXlqyxw.b = value_as_number(YYpkbUXlqyxw) > value_as_number(stfSDSgHQqlz);
		YYpkbUXlqyxw.type = VALUE_BOOL;
		guVNHuDDXkDi.b = value_as_bool(guVNHuDDXkDi) && value_as_bool(YYpkbUXlqyxw);
		guVNHuDDXkDi.type = VALUE_BOOL;
		g->var_my_variable = guVNHuDDXkDi; // setting my variable
		s->state = 4;
	}
	break;
	case 4: {
		// Starting SetVariableTo { value: Block(Add { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(1.0) }), var: Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" } }
		// Starting Add { lhs: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), rhs: Number(1.0) }
		Value yGzvBGBxybbL = g->var_my_variable; // my variable
		Value gmrAMBZaUond = (Value){ .type = VALUE_NUM, .n = 1 };
		yGzvBGBxybbL.n = value_as_number(yGzvBGBxybbL) + value_as_number(gmrAMBZaUond);
		yGzvBGBxybbL.type = VALUE_NUM;
		g->var_my_variable = yGzvBGBxybbL; // setting my variable
		s->state = 5;
	}
	break;
	case 5: {
		// Starting SayForSecs { message: Variable(Variable { name: "my variable", id: "`jEk@4|i[#Fk?(8x)AV.-my variable" }), secs: Number(2.0) }
		Value kwzGDjHuZYgC = g->var_my_variable; // my variable
		if (kwzGDjHuZYgC.type == VALUE_NUM) printf("%f\n", kwzGDjHuZYgC.n);
		else if (kwzGDjHuZYgC.type == VALUE_STRING) printf("%s\n", kwzGDjHuZYgC.s);
		else if (kwzGDjHuZYgC.type == VALUE_COLOR) printf("#%02X%02X%02X\n", kwzGDjHuZYgC.c.r, kwzGDjHuZYgC.c.g, kwzGDjHuZYgC.c.b);
		else if (kwzGDjHuZYgC.type == VALUE_BOOL) {
			if (kwzGDjHuZYgC.b) printf("true\n");
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

