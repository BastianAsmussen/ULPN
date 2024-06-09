// MainActivity.kt
package app.ulpn

import android.os.Bundle
import android.view.Menu
import com.google.android.material.navigation.NavigationView
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import androidx.drawerlayout.widget.DrawerLayout
import androidx.appcompat.app.AppCompatActivity
import androidx.core.os.bundleOf
import androidx.core.view.GravityCompat
import androidx.navigation.NavController
import app.ulpn.databinding.ActivityMainBinding
import app.ulpn.ui.Forum

class MainActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding
    private lateinit var apiManager: ApiManager
    private lateinit var navController: NavController
    private lateinit var drawerLayout: DrawerLayout
    private lateinit var navView: NavigationView
    private val addedForumMenuIds = mutableListOf<Int>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.appBarMain.toolbar)

        drawerLayout = binding.drawerLayout
        navView = binding.navView
        navController = findNavController(R.id.nav_host_fragment_content_main)

        // Passing each menu ID as a set of Ids because each
        // menu should be considered as top level destinations.
        appBarConfiguration = AppBarConfiguration(
            setOf(
                R.id.nav_home
            ), drawerLayout
        )
        setupActionBarWithNavController(navController, appBarConfiguration)
        navView.setupWithNavController(navController)

        // Initialize ApiManager
        apiManager = ApiManager(this)

        // Fetch forums from the API
        apiManager.fetchForums { forums ->
            runOnUiThread {
                addForumNavViews(forums)
            }
        }

    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        menuInflater.inflate(R.menu.main, menu)
        return true
    }

    override fun onSupportNavigateUp(): Boolean {
        return navController.navigateUp(appBarConfiguration) || super.onSupportNavigateUp()
    }

    fun addForumNavViews(forums: List<Forum>) {
        val menu = navView.menu
        val forumMap = forums.associateBy { it.id }
        val ownerToForumMap = forums.filter { it.ownerId != null }.associateBy({ it.ownerId }, { it })
        val addedForumIds = mutableSetOf<Int>() // Track added forum IDs

        forums.forEach { forum ->
            if (forum.id !in addedForumIds) {
                val menuItem = menu.add(R.id.forum_group, forum.id, Menu.NONE, forum.title)
                menuItem.setIcon(if (forum.is_locked) R.drawable.forum else R.drawable.chat)
                menuItem.setOnMenuItemClickListener { menuItem ->
                    val bundle = bundleOf("forumId" to forum.id, "forumTitle" to forum.title, "description" to forum.description)

                    val destinationId = if (forum.is_locked) R.id.nav_dynamic_Read else R.id.nav_dynamic_Write

                    val navInflater = navController.navInflater
                    val navGraph = navInflater.inflate(R.navigation.mobile_navigation)
                    navGraph.findNode(destinationId)?.label = forum.title
                    navController.graph = navGraph

                    navController.navigate(destinationId, bundle)
                    drawerLayout.closeDrawer(GravityCompat.START)
                    true
                }

                addedForumIds.add(forum.id) // Track added forum IDs

                // If the forum has an owner, find its owner forum and add it directly underneath
                forum.ownerId?.let { ownerId ->
                    ownerToForumMap[ownerId]?.let { ownerForum ->
                        if (ownerForum.id !in addedForumIds) {
                            val ownerMenuItem = menu.add(R.id.forum_group, ownerForum.id, Menu.NONE, ownerForum.title)
                            ownerMenuItem.setIcon(if (ownerForum.is_locked) R.drawable.forum else R.drawable.chat)
                            ownerMenuItem.setOnMenuItemClickListener { menuItem ->
                                // Handle click event for owner forum
                                // Navigation
                                true
                            }
                            addedForumIds.add(ownerForum.id) // Track added owner forum IDs
                        }
                    }
                }
            }
        }
    }




    fun removeForumNavViews() {
        val menu = navView.menu
        addedForumMenuIds.forEach { menuId ->
            menu.removeItem(menuId)
        }
        addedForumMenuIds.clear()
    }
}