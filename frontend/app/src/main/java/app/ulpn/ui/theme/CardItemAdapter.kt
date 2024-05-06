package app.ulpn.ui.theme

import android.app.Activity
import android.content.Intent
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.TextView
import androidx.annotation.LayoutRes
import app.ulpn.R
import app.ulpn.ui.theme.CardDetailsActivity

class CardItemAdapter(private val activity: Activity, private val cardItemList: List<CardItem>) :
    ArrayAdapter<CardItem>(activity, 0, cardItemList) {

    override fun getView(position: Int, convertView: View?, parent: ViewGroup): View {
        var listItemView = convertView
        if (listItemView == null) {
            listItemView = LayoutInflater.from(activity).inflate(
                R.layout.list_item_card,
                parent,
                false
            )
        }

        val currentItem = getItem(position)

        val descriptionTextView = listItemView?.findViewById<TextView>(R.id.descriptionTextView)
        val cardButton = listItemView?.findViewById<Button>(R.id.cardButton)

        currentItem?.let { item ->
            descriptionTextView?.text = item.description
            cardButton?.text = item.title

            // Set click listener on the button
            cardButton?.setOnClickListener {
                val intent = Intent(activity, CardDetailsActivity::class.java).apply {
                    putExtra("title", item.title)
                    putExtra("description", item.description)
                }
                activity.startActivity(intent)
                activity.finish()
            }
        }

        return listItemView!!
    }
}

