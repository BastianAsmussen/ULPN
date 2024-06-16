package app.ulpn.ui.home

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import app.ulpn.ApiManager

class HomeViewModel(private val apiManager: ApiManager) : ViewModel() {

    private val _title = MutableLiveData<String?>()
    val title: LiveData<String?> get() = _title

    private val _description = MutableLiveData<String?>()
    val description: LiveData<String?> get() = _description

    init {
        _title.value = "Loading..."
        _description.value = ""
    }

    fun updateSettings(settings: Map<String, String>) {
        _title.value = settings["home_title"]
        _description.value = settings["home_desc"]
    }
}
