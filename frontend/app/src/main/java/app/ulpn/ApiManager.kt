package app.ulpn

import android.content.Context
import android.util.Log
import app.ulpn.ui.Forum
import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.toolbox.JsonArrayRequest
import com.android.volley.toolbox.StringRequest
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
                        val owner = if (jsonObj.has("owner_id")) jsonObj.getIntOrNull("owner_id") else null
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

    // Helper function to handle nullability of getInt
    private fun JSONObject.getIntOrNull(key: String): Int? {
        return if (has(key) && !isNull(key)) getInt(key) else null
    }


    fun saveForum(id: Int, newTitle: String, newDescription: String, isLocked: Boolean, accessLevel: String, callback: (Boolean) -> Unit) {
        val activity = context as MainActivity
        val credentials = activity.getCredentials()
        val apiUrl = "$serverIp/forum/$id"

        val request = object : StringRequest(Method.PUT, apiUrl,
            Response.Listener { response ->
                Log.d("Save Forum Response:", response)
                callback(true)
            },
            Response.ErrorListener { error ->
                Log.e("ULPN API", "Error: ${error.message}")
                callback(false)
            }) {
            override fun getHeaders(): MutableMap<String, String> {
                val headers = mutableMapOf<String, String>()
                headers["Authorization"] = "${credentials["userId"]}"
                headers["Content-Type"] = "application/json"
                return headers
            }

            override fun getBody(): ByteArray {
                val jsonBody = JSONObject()
                jsonBody.put("title", newTitle)
                jsonBody.put("description", newDescription)
                jsonBody.put("isLocked", isLocked)
                jsonBody.put("accessLevel", accessLevel)
                return jsonBody.toString().toByteArray(Charsets.UTF_8)
            }
        }

        reqQueue.add(request)
    }


    fun deleteForum(id: Int, callback: (Boolean) -> Unit) {
        val activity = context as MainActivity
        val credentials = activity.getCredentials()
        val apiUrl = "$serverIp/forum/$id"

        val request = object : StringRequest(Method.DELETE, apiUrl,
            Response.Listener { response ->
                Log.d("Delete Forum Response:", response)
                callback(true)
            },
            Response.ErrorListener { error ->
                Log.e("ULPN API", "Error: ${error.message}")
                callback(false)
            }) {
            override fun getHeaders(): MutableMap<String, String> {
                val headers = mutableMapOf<String, String>()
                headers["Authorization"] = "${credentials["userId"]}"
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

    fun saveSetting(key: String, newValue: String, callback: (Boolean) -> Unit) {
        Log.d("Confirmation", "Save Setting is being called...")
        val activity = context as MainActivity
        val credentials = activity.getCredentials()
        val apiUrl = "$serverIp/settings/$key"

        val request = object : StringRequest(Method.PUT, apiUrl, Response.Listener { response ->
            Log.d("Save Setting Response:", response)
            callback(true)
        }, Response.ErrorListener { err ->
            Log.e("ULPN API", "Error: ${err.message}")
            err.networkResponse?.let { response ->
                Log.e("ULPN API", "Error Response Code: ${response.statusCode}")
                response.data?.let { data ->
                    Log.e("ULPN API", "Error Data: ${String(data)}")
                }
            }
            callback(false)
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
