// Generated by gir (https://github.com/gtk-rs/gir @ 05fe12c0b7e7)
// from gir-files (https://github.com/gtk-rs/gir-files @ b827978e7d18)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ eabb1f9cac5b)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT(GST_NET_TIME_PACKET_SIZE);
    PRINT_CONSTANT(GST_PTP_CLOCK_ID_NONE);
    PRINT_CONSTANT(GST_PTP_STATISTICS_BEST_MASTER_CLOCK_SELECTED);
    PRINT_CONSTANT(GST_PTP_STATISTICS_NEW_DOMAIN_FOUND);
    PRINT_CONSTANT(GST_PTP_STATISTICS_PATH_DELAY_MEASURED);
    PRINT_CONSTANT(GST_PTP_STATISTICS_TIME_UPDATED);
    return 0;
}
