package `in`.matter.pictorusdemo1

object SimulationModelAppInterface {
    private var appInterfacePointer: Long = 0
//    private var appDataOutput: AppDataOutput

    init {
        System.loadLibrary("pictorusdemo1")
        appInterfacePointer = appInterfaceNew() // Create the Rust App
//        appDataOutput = AppDataOutput()
    }

    fun update(time: Double, appDataInput: AppDataInput) = appInterfaceUpdate(appInterfacePointer, time, appDataInput)

    fun destroy() {
        appInterfaceFree(appInterfacePointer)
        appInterfacePointer = 0
    }

    // JNI Declarations
    private external fun appInterfaceNew(): Long
    private external fun appInterfaceFree(handle: Long)
    private external fun appInterfaceUpdate(
        handle: Long,
        appTimeS: Double,
        inputData: AppDataInput
    ): AppDataOutput

    // Data classes mirroring C structs
    data class AppDataInput(
        var speed: Double
    )

    data class AppDataOutput(
        val Distance: Double
    )
}