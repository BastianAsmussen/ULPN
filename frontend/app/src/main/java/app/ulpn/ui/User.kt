package app.ulpn.ui

import java.security.MessageDigest

class User private constructor(private val username: String, private val email: String, private var accessLevel: Int = 0) {

    companion object {
        private var instance: User? = null

        fun getInstance(username: String, email: String): User {
            if (instance == null) {
                instance = User(username, email)
            }
            return instance!!
        }
    }

    // Custom setter for accessLevel
    fun setAccessLevel(level: Int) {
        accessLevel = level
    }

    // Getter methods for properties
    fun getUsername(): String {
        return username
    }

    fun getEmail(): String {
        return email
    }

    fun getAccessLevel(): Int {
        return accessLevel
    }

}
