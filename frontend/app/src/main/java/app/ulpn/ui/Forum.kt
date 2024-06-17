package app.ulpn.ui

data class Forum(
    val id: Int,
    val title: String,
    val description: String,
    val isLocked: Boolean,
    val accessLevel: String,
    val ownerId: Int?
)