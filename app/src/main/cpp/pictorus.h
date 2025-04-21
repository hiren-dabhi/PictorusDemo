typedef struct AppInterface AppInterface;

typedef struct AppDataOutput {
    double CurrentMaxSpeed;
    double CurrentAvgSpeed;
    double Current_distancetravelledraw;
    double Current_Mintime0to60kmph;
    double Current_RideDuration;
    double CO2cons_current;
    double Current_EnergykWhr;
    double Current_RideEffWhkm;
    double Amperehour_Calc;
    double FallDetect_Stationary;
    double FallDetect_Motion;
    double gForce;
    double LeanAngle;
    double Leftguageeff;
    double power;
    double Range;
    double CTFullcharge;
    double FullchargeECORange;
    double CT25percharge;
    double CT50percharge;
    double CT75percharge;
    double EcoRange25percharge;
    double EcoRange50percharge;
    double EcoRange75percharge;
    double SuggestedGear;
    double RIndiflag;
    double pitchangle;
} AppDataOutput;

typedef struct AppDataInput {
    double Voltage;
    double Current;
    double Speed;
    double VehicleMode;
    double SOC;
    double BMSRemCap;
    double Subridemodes;
    double RPM;
    double CurrentGear;
    double ClutchStatus;
    double Torque;
    double DeAcc;
    double Ax;
    double Ay;
    double Az;
    double motorswitchstatus;
    double VICFlag;
    double Vacationmodeoff;
    double RightIndicator;
} AppDataInput;

struct AppInterface *app_interface_new(void);

void app_interface_free(struct AppInterface *app);

struct AppDataOutput app_interface_update(struct AppInterface *app,
                                          double app_time_s,
                                          struct AppDataInput *input_data);