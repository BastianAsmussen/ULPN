package app.ulpn.ui.theme
import android.os.Bundle
import android.widget.TextView
import androidx.activity.ComponentActivity
import app.ulpn.R

class CardDetailsActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_card_details)

        val title = intent.getStringExtra("title")
        val description = intent.getStringExtra("description")

        val titleTextView = findViewById<TextView>(R.id.titleTextView)
        val descriptionTextView = findViewById<TextView>(R.id.descriptionTextView)

        titleTextView.text = title
        descriptionTextView.text = description
    }
}
