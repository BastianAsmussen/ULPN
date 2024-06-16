package app.ulpn.ui.editforum

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import app.ulpn.ApiManager
import app.ulpn.ui.Forum

class EditForumViewModel(private val apiManager: ApiManager) : ViewModel() {

    // Private mutable LiveData to hold the list of forums
    private val _forums = MutableLiveData<List<Forum>>()
    val forums: LiveData<List<Forum>> = _forums

    init {
        // Fetch forums when ViewModel is initialized
        fetchForums()
    }

    fun fetchForums() {
        apiManager.fetchForumsApi { response ->
            _forums.postValue(response)
        }
    }

    fun saveForum(id: Int, newTitle: String, newDescription: String) {

    }
}
