package app.ulpn

import android.content.Context
import android.nfc.Tag
import android.util.Log
import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.toolbox.JsonArrayRequest
import com.android.volley.toolbox.Volley
import app.ulpn.ui.Forum
import app.ulpn.ui.User
import com.android.volley.toolbox.JsonObjectRequest
import org.json.JSONArray
import org.json.JSONObject
import java.lang.reflect.Method

data class ApiManager(private val context: Context?) {
    private val serverIp = "http://51.68.175.190:3000" // TODO: Change to server IP!

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

    fun sendUserInfoToApi(user: User, callback: (String?) -> Unit) {
        val reqQueue: RequestQueue = Volley.newRequestQueue(context)
        val apiUrl = "$serverIp/sessions/oauth/google"

        val requestBody = JSONObject()
        requestBody.put("redirect_uri", "")
        requestBody.put("client_id", "")

        val scopes = JSONArray()
        scopes.put("https://www.googleapis.com/auth/userinfo.profile")
        scopes.put("https://www.googleapis.com/auth/userinfo.email")
        requestBody.put("scope", scopes)

        val request = JsonObjectRequest(
            Request.Method.POST, apiUrl, requestBody,
            { response: JSONObject ->
                val token = response.optString("token")
                callback(token)
            },
            { error ->
                Log.e("ULPN API", error.message ?: "Unknown error")
                callback(null)
            }
        )
        reqQueue.add(request)
    }



    fun fetchForumsJwt(jwtToken: String, callback: (List<Forum>) -> Unit) {
        val reqQueue: RequestQueue = Volley.newRequestQueue(context)
        val apiUrl = "$serverIp/forum"

        val request = object : JsonArrayRequest(
            Method.GET, apiUrl, null,
            { response: JSONArray ->
                val forums = arrayListOf<Forum>()
                for (i in 0 until response.length()) {
                    val jsonObj: JSONObject = response.getJSONObject(i)
                    val forum = Forum(
                        jsonObj.getInt("id"),
                        jsonObj.getString("title"),
                        jsonObj.getString("description"),
                        jsonObj.getBoolean("is_locked"),
                        jsonObj.getInt("owner")
                    )
                    forums.add(forum)
                }
                Log.d("ULPN API", forums.toString())
                callback(forums)
            },
            { error ->
                Log.e("ULPN API", error.message.toString())
            }
        ) {
            override fun getHeaders(): Map<String, String> {
                val headers = HashMap<String, String>()
                headers["Authorization"] = "Bearer $jwtToken"
                return headers
            }
        }

        Log.d("API CALL", request.toString())
        Log.d("API CALL", request.headers.toString())
        reqQueue.add(request)
    }

}

