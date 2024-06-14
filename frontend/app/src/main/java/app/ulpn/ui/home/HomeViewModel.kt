package app.ulpn.ui.home

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import app.ulpn.ApiManager

class HomeViewModel(private val apiManager: ApiManager) : ViewModel() {

    // Private mutable LiveData for title
    private val _title = MutableLiveData<String?>()

    // Public immutable LiveData for title
    val title: MutableLiveData<String?>
        get() = _title

    init {
        // Initially set a placeholder value
        _title.value = "Loading..."
    }

    // Function to update settings and update _title
    fun updateSettings(settings: Map<String, String>) {
        val newTitle = settings["home_title"]
        Log.d("HomeViewModel", "Updating title to: $newTitle")
        _title.value = newTitle.toString()
    }
}
