#include <stdio.h>
#include "../generated_by_buildrs.h"

int main() {
    Value v = create_number(5);
    puts_value(&v);
    return 0;
}
