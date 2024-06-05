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
            Hjælp dit barn eller dine børn med at beskytte sig på de forskellige apps og medier, 
            så de bliver bedst udrustettil den digitale verden. Hvorpå den svære samtale gøres nemmere, 
            når man skal italesætte problematikker derkan opstå når man begår sig online.

            
  
        """.trimIndent()

        _text.value = markdownText
    }
}
