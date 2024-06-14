package app.ulpn

import android.content.Context
import android.util.Log
import app.ulpn.ui.Forum
import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.toolbox.JsonArrayRequest
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.Volley
import org.json.JSONArray
import org.json.JSONException
import org.json.JSONObject


data class ApiManager(private val context: Context?) {
    private val serverIp = "http://51.68.175.190:3000"
    var reqQueue: RequestQueue = Volley.newRequestQueue(context)
    fun fetchForumsApi(callback: (List<Forum>) -> Unit) {
        val activity = context as MainActivity
        val credentials = activity.getCredentials()
        val apiUrl = "$serverIp/forum"

        val request = object : JsonArrayRequest(
            Request.Method.GET,
            apiUrl,
            null,
            Response.Listener { response ->
                try {
                    val forums = mutableListOf<Forum>()

                    for (i in 0 until response.length()) {
                        val jsonObj = response.getJSONObject(i)
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
            Response.ErrorListener { error ->
                Log.e("ULPN API", "Error: ${error.message}")
                error.networkResponse?.let { response ->
                    Log.e("ULPN API", "Error Response Code: ${response.statusCode}")
                    response.data?.let { data ->
                        Log.e("ULPN API", "Error Data: ${String(data)}")
                    }
                }
            }
        ) {
            override fun getHeaders(): MutableMap<String, String> {
                val headers = mutableMapOf<String, String>()
                headers["Authorization"] = "${credentials["userId"]}"
                Log.d("Credentials: ", headers.toString())
                return headers
            }
        }
        reqQueue.add(request)
    }



    fun fetchSettings(callback: (JSONArray) -> Unit) {
        Log.d("Confirmation", "Fetch Settings is being called...")
        val activity = context as MainActivity
        val credentials = activity.getCredentials()
        val apiUrl = "$serverIp/settings"

        val request = object : JsonArrayRequest(Method.GET, apiUrl, null, Response.Listener { result ->
            Log.d("Settings:", result.toString())
            callback(result)
        }, Response.ErrorListener { err ->
            Log.e("ULPN API", "Error: ${err.message}")
            err.networkResponse?.let { response ->
                Log.e("ULPN API", "Error Response Code: ${response.statusCode}")
                response.data?.let { data ->
                    Log.e("ULPN API", "Error Data: ${String(data)}")
                }
            }
        }) {
            override fun getHeaders(): MutableMap<String, String> {
                val headers = mutableMapOf<String, String>()
                headers["Authorization"] = "${credentials["userId"]}"
                Log.d("Credentials: ", headers.toString())
                return headers
            }
        }

        reqQueue.add(request)
    }



}
