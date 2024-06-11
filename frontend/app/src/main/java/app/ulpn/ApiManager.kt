package app.ulpn

import android.content.Context
import android.util.Log
import app.ulpn.ui.Forum
import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.toolbox.JsonArrayRequest
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.Volley

data class ApiManager(private val context: Context?) {
    private val serverIp = "http://51.68.175.190:3000"

    fun fetchForums(callback: (List<Forum>) -> Unit) {
        val reqQueue: RequestQueue = Volley.newRequestQueue(context)
        val apiUrl = "$serverIp/forum"
        val request = JsonArrayRequest(Request.Method.GET, apiUrl, null, { result ->
            val forums = arrayListOf<Forum>()
            for (i in 0 until result.length()) {
                val jsonObj = result.getJSONObject(i)
                val owner = if (jsonObj.has("owner")) jsonObj.getInt("owner") else 0
                val forum = Forum(
                    jsonObj.getInt("id"),
                    jsonObj.getString("title"),
                    jsonObj.getString("description"),
                    jsonObj.getBoolean("is_locked"),
                    owner
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

    fun fetchSettings(callback: (Map<String, String>) -> Unit) {
        val reqQueue: RequestQueue = Volley.newRequestQueue(context)
        val apiUrl = "$serverIp/settings"
        val request = JsonObjectRequest(Request.Method.GET, apiUrl, null, { result ->
            val settings = mutableMapOf<String, String>()
            val keys = result.keys()
            while (keys.hasNext()) {
                val key = keys.next()
                settings[key] = result.getString(key)
            }
            Log.d("ULPN API", settings.toString())
            callback(settings)
        }, { err ->
            Log.e("ULPN API", err.message.toString())
        })

        reqQueue.add(request)
    }
}
