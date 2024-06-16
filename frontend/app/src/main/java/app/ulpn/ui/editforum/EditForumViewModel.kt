package app.ulpn.ui.editforum

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import app.ulpn.ApiManager
import app.ulpn.ui.Forum

class EditForumViewModel(private val apiManager: ApiManager) : ViewModel() {

    private val _forums = MutableLiveData<List<Forum>>()
    val forums: LiveData<List<Forum>> = _forums

    init {
        fetchForums()
    }

    fun fetchForums() {
        apiManager.fetchForumsApi { response ->
            _forums.postValue(response)
        }
    }

    fun saveForum(id: Int, newTitle: String, newDescription: String, isLocked: Boolean, accessLevel: String, callback: (Boolean) -> Unit) {
        apiManager.saveForum(id, newTitle, newDescription, isLocked, accessLevel) { success ->
            callback(success)
        }
    }

    fun deleteForum(id: Int, callback: (Boolean) -> Unit) {
        apiManager.deleteForum(id) { success ->
            if (success) {
                // Update the list of forums after deletion
                fetchForums()
            }
            callback(success)
        }
    }

    fun addForum(title: String, description: String, isLocked: Boolean, accessLevel: String, ownerId: Int?, callback: (Boolean) -> Unit) {
        apiManager.addForum(title, description, isLocked, accessLevel, ownerId) { success ->
            if (success) {
                // Update the list of forums after addition
                fetchForums()
            }
            callback(success)
        }
    }
}
