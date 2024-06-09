package app.ulpn.ui.forum

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class ForumViewModel : ViewModel() {

    private val _text = MutableLiveData<String>()
    val text: LiveData<String> = _text

    init {
        val markdownText = """
        """.trimIndent()

        _text.value = markdownText
    }

    fun updateText(newText: String) {
        _text.value = newText
    }
}
