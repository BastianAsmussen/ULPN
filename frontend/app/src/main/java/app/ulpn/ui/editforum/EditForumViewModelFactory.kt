package app.ulpn.ui.editforum

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import app.ulpn.ApiManager

class EditForumViewModelFactory(private val apiManager: ApiManager) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(EditForumViewModel::class.java)) {
            @Suppress("UNCHECKED_CAST")
            return EditForumViewModel(apiManager) as T
        }
        throw IllegalArgumentException("Unknown ViewModel class")
    }
}
