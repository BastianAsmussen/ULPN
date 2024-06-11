package app.ulpn.ui.home

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

        // Fetch data from API and update title
        apiManager.fetchSettings { settings ->
            _title.postValue(settings["home_title"])
        }
    }
}
