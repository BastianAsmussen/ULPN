// ApiManager.kt
package app.ulpn

import android.content.Context
import android.util.Log
import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.toolbox.JsonArrayRequest
import com.android.volley.toolbox.Volley
import app.ulpn.ui.Forum

data class ApiManager(private val context: Context) {
    private val apiSample = "http://172.17.0.1:3000/forum" // TODO: Change to server IP!

    fun fetchForums(callback: (List<Forum>) -> Unit) {
        val reqQueue: RequestQueue = Volley.newRequestQueue(context)
        val request = JsonArrayRequest(Request.Method.GET, apiSample, null, { result ->
            val forums = arrayListOf<Forum>()
            for (i in 0 until result.length()) {
                val jsonObj = result.getJSONObject(i)
                val forum = Forum(
                    jsonObj.getInt("id"),
                    jsonObj.getString("title"),
                    jsonObj.getString("description"),
                    jsonObj.getBoolean("is_locked")
                )
                forums.add(forum)
            }
            Log.d("ULPN API", forums.toString())
            callback(forums)
        }, { err ->
            Log.e("ULPN API", err.message.toString())
        })

        reqQueue.add(request)
    }
}
