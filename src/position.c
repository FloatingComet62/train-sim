#include "lib.h"

position lerp(position a, position b, float t) {
    position result;
    result.x = a.x + (b.x - a.x) * t;
    result.y = a.y + (b.y - a.y) * t;
    return result;
}