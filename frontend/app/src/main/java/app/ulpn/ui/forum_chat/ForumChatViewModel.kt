package app.ulpn.ui.forum_chat

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class ForumChatViewModel : ViewModel() {

    private val _text = MutableLiveData<String>().apply {
        value = "This is forum Fragment"
    }

    fun updateText(newText: String) {
        _text.value = newText
    }

    val text: LiveData<String> = _text
}