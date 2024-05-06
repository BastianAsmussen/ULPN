package app.ulpn

import android.content.Intent
import android.os.Bundle
import android.widget.Button
import android.widget.ListView
import androidx.activity.ComponentActivity
import app.ulpn.ui.theme.CardItem
import app.ulpn.ui.theme.CardItemAdapter

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val listView = findViewById<ListView>(R.id.listView)

        val cardList = mutableListOf(
            CardItem("Grooming", "Card 1 "),
            CardItem("Online Gaming", "Card 2 Description")
        )

        val adapter = CardItemAdapter(this, cardList)
        listView.adapter = adapter

        val loginButton = findViewById<Button>(R.id.login)
        loginButton.setOnClickListener {
            val intent = Intent(this, HomeActivity::class.java)
            startActivity(intent)
            finish()
        }
    }
}

