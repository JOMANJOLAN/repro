#include <stdio.h>

int main() {
    char buf[256];
    int fps = 60;
    int w = 16 * fps;
    int h = 9 * fps;
    for (int i = 0; i < fps; i++) {
        snprintf(buf, sizeof(buf), "output_%02d.ppm", i);
        const char* output_path = buf;
        FILE* file = fopen(output_path, "wb");
        fprintf(file, "P6\n");           // format
        fprintf(file, "%d %d\n", w, h);  // width / height
        fprintf(file, "255\n");          // the maximum value of the color component, usually 255 for P6
        for (int y = 0; y < h; y++) {
            for (int x = 0; x < w; x++) {
                if (((x + i) / fps + (y + i) / fps) % 2) {
                    fputc(0xFF, file);
                } else {
                    fputc(0x00, file);
                }
                fputc(0x00, file);
                fputc(0x00, file);
            }
        }
        fclose(file);
        printf("Generated %s\n", output_path);
    }
    return 0;
}
