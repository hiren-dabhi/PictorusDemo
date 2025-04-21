// Builds with the following command from crate root:
// gcc src/example_app.c src/libtrialappaddition_67e380d06a4093c501660122.a -o example_app -lpthread -Wl,--no-as-needed -ldl -lm
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <jni.h>
#include "pictorus.h"
#include <android/log.h>
#define  TAG    "PictorusDemoApp1"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, TAG, __VA_ARGS__)
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, TAG, __VA_ARGS__)

JNIEXPORT jlong JNICALL Java_in_matter_pictorusdemo1_SimulationModelAppInterface_appInterfaceNew(JNIEnv* env, jobject obj) {
    return (jlong)app_interface_new();
}

JNIEXPORT void JNICALL Java_in_matter_pictorusdemo1_SimulationModelAppInterface_appInterfaceFree(JNIEnv* env, jobject obj, jlong handle) {
    app_interface_free((struct AppInterface*)handle);
}

JNIEXPORT jobject JNICALL Java_in_matter_pictorusdemo1_SimulationModelAppInterface_appInterfaceUpdate(
        JNIEnv* env, jobject obj,
        jlong handle,
        jdouble appTimeS,
        jobject inputData
) {
    // Marshal Java object to C struct
    jclass inputClass = (*env)->GetObjectClass(env, inputData);
    struct AppDataInput cInput = {
            .speed = (*env)->GetDoubleField(env, inputData, (*env)->GetFieldID(env, inputClass, "speed", "D")),
    };

    struct AppDataOutput cOutput = app_interface_update(
            (struct AppInterface*)handle,
            appTimeS,
            &cInput
    );

    // Marshal back to Java
    jclass outputClass = (*env)->FindClass(env, "in/matter/pictorusdemo1/SimulationModelAppInterface$AppDataOutput");
    jobject output = (*env)->NewObject(env, outputClass,
                                       (*env)->GetMethodID(env, outputClass, "<init>", "(D)V"),
                                       cOutput.Distance
    );
    return output;
}



//void print_data(double app_time_s, AppDataOutput *data) {
//    LOGD("Time: %f, sinewave_out: %f\n", app_time_s, data->sinewave_out);
//}
//void Java_in_matter_pictorusdemo1_PictorusAppInterface_main(JNIEnv* env,
//                                                  jobject  this ) {
//    LOGD("Starting app");
//
////    AppInterface *app_iface = app_interface_new();
//
//
////    double timestep_s = 0.1;
////    double app_time_s = 0.0;
////    double max_time_s = 10.0;
////
////    for(double app_time_s=0.0; app_time_s < max_time_s; app_time_s += timestep_s) {
////        AppDataOutput output = app_interface_update(app_iface, app_time_s);
////        print_data(app_time_s, &output);
////    }
//
////    app_interface_free(app_iface);
//
//    LOGD("Done running app");
//}
//AppInterface *app_iface;
//JNIEXPORT jlong JNICALL
//Java_in_matter_pictorusdemo1_SimulationModelAppInterface_appInterfaceNew(JNIEnv *env, jobject thiz) {
//    app_iface = app_interface_new();
//}
//
//JNIEXPORT void JNICALL
//Java_in_matter_pictorusdemo1_SimulationModelAppInterface_appInterfaceFree(JNIEnv *env, jobject thiz,
//                                                                          jlong app) {
//    app_interface_free(app_iface);
//}
//
//JNIEXPORT void JNICALL
//Java_in_matter_pictorusdemo1_SimulationModelAppInterface_appInterfaceUpdate(JNIEnv *env, jobject thiz,
//                                                                            jlong app, jdouble app_time_s/*,
//                                                                     jobject app_data_output*/) {
//    AppDataOutput output = app_interface_update(app_iface, app_time_s);
//    print_data(app_time_s, &output);
////    app_data_output. = output
//}