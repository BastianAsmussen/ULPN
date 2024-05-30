package app.ulpn.ui.home

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class HomeViewModel : ViewModel() {

    private val _text = MutableLiveData<String>()
    val text: LiveData<String> = _text

    init {
        // Example Markdown text
        val markdownText = """
            # Hello Markdown
            
            This is a **bold** text and *italic* text.
            
            ## List
            
            - Item 1
            - Item 2
            - Item 3
            
            ## Link
            
            [OpenAI](https://www.openai.com/)
            
            ## Code
            
            ```
            fun main() {
                println("Hello, Markdown!")
            }
            ```
        """.trimIndent()

        _text.value = markdownText
    }
}
