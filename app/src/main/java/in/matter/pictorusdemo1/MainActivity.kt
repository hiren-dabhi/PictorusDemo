package `in`.matter.pictorusdemo1

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import `in`.matter.pictorusdemo1.databinding.ActivityMainBinding
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlin.coroutines.CoroutineContext
import kotlin.random.Random

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // Example of a call to a native method
//        binding.sampleText.text = stringFromJNI()
//        main()

        val timestep_s = 0.1
        var app_time_s = 0.0
        val max_time_s = 10.0

        /*val appDataInput = SimulationModelAppInterface.AppDataInput(23.3, 12.5, 1.3233, 3.2)

        lifecycleScope.launch(Dispatchers.IO) {
            while (*//*app_time_s < max_time_s*//*true) {
                appDataInput.Ay = Random.nextDouble(1.3,7.99)
                appDataInput.Curr = Random.nextDouble(1.3,7.99)
                appDataInput.Speed = Random.nextDouble(1.3,7.99)
                appDataInput.EntropyDiff = Random.nextDouble(1.3,7.99)
                SimulationModelAppInterface.update(app_time_s, appDataInput).let {
                    Log.d("crashmodel", "output=${it.CrashFlag}")
                }
//            print_data(app_time_s)
                app_time_s += timestep_s
                delay(100)
            }
        }*/

        val appDataInput = SimulationModelAppInterface.AppDataInput(0.0)

        lifecycleScope.launch(Dispatchers.IO) {
            while (/*app_time_s < max_time_s*/true) {
                appDataInput.speed = Random.nextDouble(100.0,200.0)
                SimulationModelAppInterface.update(app_time_s, appDataInput).let {
                    Log.d("distancecalcmodel", "speed:${appDataInput.speed}, output=${it.Distance}")
                }
//            print_data(app_time_s)
                app_time_s += timestep_s
                delay(100)
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        SimulationModelAppInterface.destroy()
    }

    /**
     * A native method that is implemented by the 'pictorusdemo1' native library,
     * which is packaged with this application.
     */
//    external fun stringFromJNI(): String
//    external fun main()
}