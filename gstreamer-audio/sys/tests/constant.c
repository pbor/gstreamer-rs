// Generated by gir (https://github.com/gtk-rs/gir @ df67128a87f0)
// from gir-files (https://github.com/gtk-rs/gir-files @ b827978e7d18)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ f8c393377c4e)
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
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_DISCONT_REASON_ALIGNMENT);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_DISCONT_REASON_DEVICE_FAILURE);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_DISCONT_REASON_FLUSH);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_DISCONT_REASON_NEW_CAPS);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_DISCONT_REASON_NO_DISCONT);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_DISCONT_REASON_SYNC_LATENCY);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_SLAVE_CUSTOM);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_SLAVE_NONE);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_SLAVE_RESAMPLE);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SINK_SLAVE_SKEW);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SRC_SLAVE_NONE);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SRC_SLAVE_RESAMPLE);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SRC_SLAVE_RE_TIMESTAMP);
    PRINT_CONSTANT((gint) GST_AUDIO_BASE_SRC_SLAVE_SKEW);
    PRINT_CONSTANT((gint) GST_AUDIO_CD_SRC_MODE_CONTINUOUS);
    PRINT_CONSTANT((gint) GST_AUDIO_CD_SRC_MODE_NORMAL);
    PRINT_CONSTANT(GST_AUDIO_CHANNELS_RANGE);
    PRINT_CONSTANT((guint) GST_AUDIO_CHANNEL_MIXER_FLAGS_NONE);
    PRINT_CONSTANT((guint) GST_AUDIO_CHANNEL_MIXER_FLAGS_NON_INTERLEAVED_IN);
    PRINT_CONSTANT((guint) GST_AUDIO_CHANNEL_MIXER_FLAGS_NON_INTERLEAVED_OUT);
    PRINT_CONSTANT((guint) GST_AUDIO_CHANNEL_MIXER_FLAGS_UNPOSITIONED_IN);
    PRINT_CONSTANT((guint) GST_AUDIO_CHANNEL_MIXER_FLAGS_UNPOSITIONED_OUT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_FRONT_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_INVALID);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_LFE1);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_LFE2);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_MONO);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_NONE);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_REAR_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_REAR_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_REAR_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_SIDE_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_SIDE_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_SURROUND_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_REAR_CENTER);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_REAR_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_REAR_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_WIDE_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_CHANNEL_POSITION_WIDE_RIGHT);
    PRINT_CONSTANT((guint) GST_AUDIO_CONVERTER_FLAG_IN_WRITABLE);
    PRINT_CONSTANT((guint) GST_AUDIO_CONVERTER_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_AUDIO_CONVERTER_FLAG_VARIABLE_RATE);
    PRINT_CONSTANT(GST_AUDIO_CONVERTER_OPT_DITHER_METHOD);
    PRINT_CONSTANT(GST_AUDIO_CONVERTER_OPT_MIX_MATRIX);
    PRINT_CONSTANT(GST_AUDIO_CONVERTER_OPT_NOISE_SHAPING_METHOD);
    PRINT_CONSTANT(GST_AUDIO_CONVERTER_OPT_QUANTIZATION);
    PRINT_CONSTANT(GST_AUDIO_CONVERTER_OPT_RESAMPLER_METHOD);
    PRINT_CONSTANT(GST_AUDIO_DECODER_MAX_ERRORS);
    PRINT_CONSTANT(GST_AUDIO_DECODER_SINK_NAME);
    PRINT_CONSTANT(GST_AUDIO_DECODER_SRC_NAME);
    PRINT_CONSTANT(GST_AUDIO_DEF_CHANNELS);
    PRINT_CONSTANT(GST_AUDIO_DEF_FORMAT);
    PRINT_CONSTANT(GST_AUDIO_DEF_RATE);
    PRINT_CONSTANT((gint) GST_AUDIO_DITHER_NONE);
    PRINT_CONSTANT((gint) GST_AUDIO_DITHER_RPDF);
    PRINT_CONSTANT((gint) GST_AUDIO_DITHER_TPDF);
    PRINT_CONSTANT((gint) GST_AUDIO_DITHER_TPDF_HF);
    PRINT_CONSTANT(GST_AUDIO_ENCODER_SINK_NAME);
    PRINT_CONSTANT(GST_AUDIO_ENCODER_SRC_NAME);
    PRINT_CONSTANT((guint) GST_AUDIO_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_AUDIO_FLAG_UNPOSITIONED);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_ENCODED);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_F32);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_F32BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_F32LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_F64);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_F64BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_F64LE);
    PRINT_CONSTANT((guint) GST_AUDIO_FORMAT_FLAG_COMPLEX);
    PRINT_CONSTANT((guint) GST_AUDIO_FORMAT_FLAG_FLOAT);
    PRINT_CONSTANT((guint) GST_AUDIO_FORMAT_FLAG_INTEGER);
    PRINT_CONSTANT((guint) GST_AUDIO_FORMAT_FLAG_SIGNED);
    PRINT_CONSTANT((guint) GST_AUDIO_FORMAT_FLAG_UNPACK);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S16);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S16BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S16LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S18);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S18BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S18LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S20);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S20BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S20LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S24);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S24BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S24LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S24_32);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S24_32BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S24_32LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S32);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S32BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S32LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_S8);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U16);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U16BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U16LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U18);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U18BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U18LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U20);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U20BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U20LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U24);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U24BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U24LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U24_32);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U24_32BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U24_32LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U32);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U32BE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U32LE);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_U8);
    PRINT_CONSTANT((gint) GST_AUDIO_FORMAT_UNKNOWN);
    PRINT_CONSTANT((gint) GST_AUDIO_LAYOUT_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_AUDIO_LAYOUT_NON_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_AUDIO_NOISE_SHAPING_ERROR_FEEDBACK);
    PRINT_CONSTANT((gint) GST_AUDIO_NOISE_SHAPING_HIGH);
    PRINT_CONSTANT((gint) GST_AUDIO_NOISE_SHAPING_MEDIUM);
    PRINT_CONSTANT((gint) GST_AUDIO_NOISE_SHAPING_NONE);
    PRINT_CONSTANT((gint) GST_AUDIO_NOISE_SHAPING_SIMPLE);
    PRINT_CONSTANT((guint) GST_AUDIO_PACK_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_AUDIO_PACK_FLAG_TRUNCATE_RANGE);
    PRINT_CONSTANT((guint) GST_AUDIO_QUANTIZE_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_AUDIO_QUANTIZE_FLAG_NON_INTERLEAVED);
    PRINT_CONSTANT(GST_AUDIO_RATE_RANGE);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_FILTER_INTERPOLATION_CUBIC);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_FILTER_INTERPOLATION_LINEAR);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_FILTER_INTERPOLATION_NONE);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_FILTER_MODE_AUTO);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_FILTER_MODE_FULL);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_FILTER_MODE_INTERPOLATED);
    PRINT_CONSTANT((guint) GST_AUDIO_RESAMPLER_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_AUDIO_RESAMPLER_FLAG_NON_INTERLEAVED_IN);
    PRINT_CONSTANT((guint) GST_AUDIO_RESAMPLER_FLAG_NON_INTERLEAVED_OUT);
    PRINT_CONSTANT((guint) GST_AUDIO_RESAMPLER_FLAG_VARIABLE_RATE);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_METHOD_BLACKMAN_NUTTALL);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_METHOD_CUBIC);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_METHOD_KAISER);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_METHOD_LINEAR);
    PRINT_CONSTANT((gint) GST_AUDIO_RESAMPLER_METHOD_NEAREST);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_CUBIC_B);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_CUBIC_C);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_CUTOFF);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_FILTER_INTERPOLATION);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_FILTER_MODE);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_FILTER_MODE_THRESHOLD);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_FILTER_OVERSAMPLE);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_MAX_PHASE_ERROR);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_N_TAPS);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_STOP_ATTENUATION);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_OPT_TRANSITION_BANDWIDTH);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_QUALITY_DEFAULT);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_QUALITY_MAX);
    PRINT_CONSTANT(GST_AUDIO_RESAMPLER_QUALITY_MIN);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_AC3);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_A_LAW);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_DTS);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_EAC3);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_FLAC);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_GSM);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IEC958);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IMA_ADPCM);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC_RAW);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC_RAW);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MU_LAW);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_FORMAT_TYPE_RAW);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_STATE_ERROR);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_STATE_PAUSED);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_STATE_STARTED);
    PRINT_CONSTANT((gint) GST_AUDIO_RING_BUFFER_STATE_STOPPED);
    PRINT_CONSTANT(GST_META_TAG_AUDIO_CHANNELS_STR);
    PRINT_CONSTANT(GST_META_TAG_AUDIO_RATE_STR);
    PRINT_CONSTANT(GST_META_TAG_AUDIO_STR);
    PRINT_CONSTANT((gint) GST_STREAM_VOLUME_FORMAT_CUBIC);
    PRINT_CONSTANT((gint) GST_STREAM_VOLUME_FORMAT_DB);
    PRINT_CONSTANT((gint) GST_STREAM_VOLUME_FORMAT_LINEAR);
    return 0;
}
