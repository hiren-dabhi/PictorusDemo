typedef struct AppInterface AppInterface;
// Data structure containing all inputs
typedef struct AppDataInput {
    double Speed;
    double Curr;
    double Ay;
    double EntropyDiff;
} AppDataInput;

// Data structure containing all outputs
typedef struct AppDataOutput {
    double CrashFlag;
} AppDataOutput;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

AppInterface *app_interface_new(void);

void app_interface_free(struct AppInterface *app);

AppDataOutput app_interface_update(struct AppInterface *app, double app_time_s, struct AppDataInput *input_data);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
