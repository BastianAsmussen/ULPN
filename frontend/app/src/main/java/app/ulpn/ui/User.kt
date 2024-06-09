package app.ulpn.ui

import java.security.MessageDigest

class User(val username: String, val email: String, var accessLevel: Int) {

    fun hashUser(): String {
        val userData = "$username$email$accessLevel"
        val bytes = userData.toByteArray()
        val md = MessageDigest.getInstance("SHA-256")
        val digest = md.digest(bytes)
        return digest.fold("", { str, it -> str + "%02x".format(it) })
    }
}
