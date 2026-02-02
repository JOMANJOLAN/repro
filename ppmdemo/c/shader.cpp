#include <math.h>
#include <stdio.h>

struct vec4 {
    float x, y, z, w;
    // vec4() : x(1.), y(1.), z(1.), w(1.) {}
    vec4() : x(0.), y(0.), z(0.), w(0.) {}
    vec4(float x, float y, float z, float w) : x(x), y(y), z(z), w(w) {}
};

struct vec2 {
    float x, y;
    // vec2() : x(1.), y(1.) {}
    vec2() : x(0.), y(0.) {}
    vec2(float x, float y) : x(x), y(y) {}
    vec2 yx() const { return vec2(y, x); }
    vec4 xyyx() const { return vec4(x, y, y, x); }
};

// vector v & scalar s
vec2 operator*(const vec2& v, float s) { return vec2(v.x * s, v.y * s); }
vec2 operator+(const vec2& v, float s) { return vec2(v.x + s, v.y + s); }
vec2 operator*(float s, const vec2& v) { return v * s; }
vec2 operator-(const vec2& v1, const vec2& v2) { return vec2(v1.x - v2.x, v1.y - v2.y); }
vec2 operator+(const vec2& v1, const vec2& v2) { return vec2(v1.x + v2.x, v1.y + v2.y); }
vec2 operator*(const vec2& v1, const vec2& v2) { return vec2(v1.x * v2.x, v1.y * v2.y); }
vec2 operator/(const vec2& v, float s) { return vec2(v.x / s, v.y / s); }
float dot(const vec2& v1, const vec2& v2) { return v1.x * v2.x + v1.y * v2.y; }
// vec2 abs(const vec2 &v) { return vec2(fabsf(v.x), fabsf(v.y)); }
vec2& operator+=(vec2& v1, const vec2& v2) {
    v1 = v1 + v2;
    return v1;
}
vec2& operator+=(vec2& v, float s) {
    v = v + s;
    return v;
}
vec2 cos(const vec2& v) { return vec2(cosf(v.x), cosf(v.y)); }
vec4 sin(const vec4& v) { return vec4(sinf(v.x), sinf(v.y), sinf(v.z), sinf(v.w)); }
vec4 exp(const vec4& v) { return vec4(expf(v.x), expf(v.y), expf(v.z), expf(v.w)); }
vec4 tanh(const vec4& v) { return vec4(tanhf(v.x), tanhf(v.y), tanhf(v.z), tanhf(v.w)); }
vec4 operator+(const vec4& v, float s) { return vec4(v.x + s, v.y + s, v.z + s, v.w + s); }
vec4 operator*(const vec4& v, float s) { return vec4(v.x * s, v.y * s, v.z * s, v.w * s); }
vec4 operator*(float s, const vec4& v) { return v * s; }
vec4 operator+(const vec4& v1, const vec4& v2) { return vec4(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w); }
vec4& operator+=(vec4& v1, const vec4& v2) {
    v1 = v1 + v2;
    return v1;
}
vec4 operator-(float s, const vec4& v) { return vec4(s - v.x, s - v.y, s - v.z, s - v.w); }
vec4 operator/(const vec4& v1, const vec4& v2) { return vec4(v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w); }

int main() {
    char buf[256];
    int fps = 60;
    int cell = 60;
    int rgb_max = 255;
    int w = 16 * cell;
    int h = 9 * cell;
    vec2 r(w, h);  // resolution of screen
    for (int i = 0; i < fps; i++) {
        snprintf(buf, sizeof(buf), "output_%02d.ppm", i);
        const char* output_path = buf;
        FILE* file = fopen(output_path, "wb");
        fprintf(file, "P6\n");           // format
        fprintf(file, "%d %d\n", w, h);  // width / height
        fprintf(file, "%d\n", rgb_max);  // the maximum value of the color component, usually 255 for P6
        float t = (float)i / fps;
        for (int y = 0; y < h; y++) {
            for (int x = 0; x < w; x++) {
                vec4 o;         // an output of a single pixel
                vec2 fc(x, y);  // fragment coordinate
                vec2 p = (fc * 2. - r) / r.y;
                vec2 l;
                l += 4. - 4. * abs(.7 - dot(p, p));
                vec2 v = p * l;
                for (vec2 i; i.y++ < 8.; o += (sin(v.xyyx()) + 1.) * abs(v.x - v.y)) {
                    v += cos(v.yx() * i.y + i + t) / i.y + .7;
                }
                o = tanh(5. * exp(l.x - 4. - p.y * vec4(-1, 1, 2, 0)) / o);
                fputc(o.x * rgb_max, file);
                fputc(o.y * rgb_max, file);
                fputc(o.z * rgb_max, file);
            }
        }
        fclose(file);
        printf("Generated %s\n", output_path);
    }
    return 0;
}
