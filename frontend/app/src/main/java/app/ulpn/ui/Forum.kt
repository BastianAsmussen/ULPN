package app.ulpn.ui

data class Forum(
    val id: Int,
    val title: String,
    val description: String,
    val is_locked: Boolean,
    val access_Level: String,
    val owner_id: Int?
)