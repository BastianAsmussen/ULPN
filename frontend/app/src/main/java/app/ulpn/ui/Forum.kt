package app.ulpn.ui

data class Forum(val id: Int, val title: String, val description: String)

val forums = listOf(
    Forum(
        id = 1,
        title = "General Discussion",
        description = "Discuss anything and everything here."
    ),
    Forum(
        id = 2,
        title = "Technical Support",
        description = "Get help with technical issues and troubleshooting."
    ),
    Forum(
        id = 3,
        title = "Programming",
        description = "Discuss programming languages, techniques, and problems."
    ),
)
