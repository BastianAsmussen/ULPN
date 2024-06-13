package app.ulpn

import android.content.Context
import android.util.Log
import app.ulpn.ui.Forum
import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.StringRequest
import com.android.volley.toolbox.Volley
import org.json.JSONException
import org.json.JSONObject
import java.nio.charset.Charset

data class ApiManager(private val context: Context?) {
    private val serverIp = "http://51.68.175.190:3000"

    fun fetchForumsApi(callback: (List<Forum>) -> Unit) {
        val activity = context as MainActivity
        val credentials = activity.getCredentials()

        val reqQueue: RequestQueue = Volley.newRequestQueue(context)
        val apiUrl = "$serverIp/forum"


        val request = object : StringRequest(
            Method.GET,
            apiUrl,
            { response ->
                try {
                    // Parse the JSON manually
                    val jsonResponse = JSONObject(response)
                    val forumsArray = jsonResponse.getJSONArray("forums")
                    val forums = mutableListOf<Forum>()

                    for (i in 0 until forumsArray.length()) {
                        val jsonObj = forumsArray.getJSONObject(i)
                        val owner = if (jsonObj.has("ownerId")) jsonObj.getInt("ownerId") else null
                        val forum = Forum(
                            jsonObj.getInt("id"),
                            jsonObj.getString("title"),
                            jsonObj.getString("description"),
                            jsonObj.getBoolean("is_locked"),
                            jsonObj.getString("access_level"),
                            owner
                        )
                        forums.add(forum)
                    }

                    callback(forums)
                } catch (e: JSONException) {
                    Log.e("ULPN API", "Error parsing JSON: ${e.message}")
                }
            },
            { error ->
                Log.e("ULPN API", "Error: ${error.message}")
                error.networkResponse?.let { response ->
                    Log.e("ULPN API", "Error Response Code: ${response.statusCode}")
                    response.data?.let { data ->
                        Log.e("ULPN API", "Error Data: ${String(data)}")
                    }
                }
            }
        ) {

        }

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
            Log.e("ULPN API", "Error: ${err.message}")
            err.networkResponse?.let { response ->
                Log.e("ULPN API", "Error Response Code: ${response.statusCode}")
                response.data?.let { data ->
                    Log.e("ULPN API", "Error Data: ${String(data)}")
                }
            }
        })

        reqQueue.add(request)
    }
}
