package app.ulpn.ui.admin

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import app.ulpn.ApiManager
import org.json.JSONArray

class AdminViewModel(private val apiManager: ApiManager) : ViewModel() {

    private val _settings = MutableLiveData<List<Pair<String, String>>>()
    val settings: LiveData<List<Pair<String, String>>> = _settings

    init {
        fetchSettings()
    }

    private fun fetchSettings() {
        apiManager.fetchSettings { response ->
            val settingsList = mutableListOf<Pair<String, String>>()
            for (i in 0 until response.length()) {
                val setting = response.getJSONObject(i)
                val key = setting.getString("key")
                val value = setting.getString("value")
                settingsList.add(Pair(key, value))
            }
            _settings.postValue(settingsList)
        }
    }

    fun saveSetting(key: String, newValue: String) {
        apiManager.saveSetting(key, newValue) { success ->
            if (success) {
                val updatedSettings = _settings.value?.map {
                    if (it.first == key) Pair(key, newValue) else it
                }
                _settings.postValue(updatedSettings!!)
            }
        }
    }
}
