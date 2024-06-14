// HomeViewModel.kt
package app.ulpn.ui.home

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import app.ulpn.ApiManager

class HomeViewModel(private val apiManager: ApiManager) : ViewModel() {
    private val _title = MutableLiveData<String>()
    val title: LiveData<String> = _title

    init {
        // Initially set a placeholder value
        _title.value = "Loading..."
    }

    // Function to update settings
    fun updateSettings(settings: Map<String, String>) {
        Log.d("TEST", "HUH")
        _title.postValue(settings["home_title"])
    }
}
