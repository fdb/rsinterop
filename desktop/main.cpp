#include <stdio.h>

typedef float f32;

struct Mesh;
struct Vector3 {
    f32 x;
    f32 y;
    f32 z;
};

extern "C" {
    Mesh* mesh_create_triangle();
    Vector3** mesh_get_positions(Mesh* mesh); // TODO need position out parameter
}

void main() {
    Mesh* mesh = mesh_create_triangle();
    Vector3* positions = *mesh_get_positions(mesh);
    for (int i = 0; i < 3; i++) {
        Vector3 pos = positions[i];
        printf("%d: [%.2f, %.2f, %.2f]\n", i, pos.x, pos.y, pos.z);
    }
}
