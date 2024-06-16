package app.ulpn.ui.admin

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import app.ulpn.ApiManager

class AdminViewModelFactory(private val apiManager: ApiManager) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(AdminViewModel::class.java)) {
            @Suppress("UNCHECKED_CAST")
            return AdminViewModel(apiManager) as T
        }
        throw IllegalArgumentException("Unknown ViewModel class")
    }
}
