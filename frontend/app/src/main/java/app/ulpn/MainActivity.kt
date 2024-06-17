package app.ulpn

import android.os.Bundle
import android.util.Log
import android.view.Menu
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.core.os.bundleOf
import androidx.core.view.GravityCompat
import androidx.drawerlayout.widget.DrawerLayout
import androidx.navigation.NavController
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import app.ulpn.databinding.ActivityMainBinding
import app.ulpn.ui.Forum
import app.ulpn.ui.home.HomeViewModel
import app.ulpn.ui.home.HomeViewModelFactory
import com.auth0.android.Auth0
import com.google.android.material.navigation.NavigationView
import org.json.JSONObject

class MainActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding
    lateinit var apiManager: ApiManager
    private lateinit var navController: NavController
    private lateinit var drawerLayout: DrawerLayout
    private lateinit var navView: NavigationView
    private val addedForumMenuIds = mutableListOf<Int>()

    lateinit var account: Auth0

    var userData: JSONObject? = null

    // ViewModel initialization
    private val homeViewModel: HomeViewModel by viewModels {
        HomeViewModelFactory(ApiManager(this))
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        account = Auth0("8KKHPtb3Q4PFQRHuJJezbjqsHlkGp3JA", "asmussen.eu.auth0.com")

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

        // Fetch forums and settings from the API
        fetchSettings(apiManager)
        fetchForums(apiManager)
    }

    fun fetchSettings(apiManager: ApiManager) {
        apiManager.fetchSettings { settingsArray ->
            val settingsMap = mutableMapOf<String, String>()
            for (i in 0 until settingsArray.length()) {
                val settingObject = settingsArray.getJSONObject(i)
                val key = settingObject.getString("key")
                val value = settingObject.getString("value")
                settingsMap[key] = value
            }
            runOnUiThread {
                homeViewModel.updateSettings(settingsMap)
            }
        }
    }

    fun fetchForums(apiManager: ApiManager) {
        apiManager.fetchForumsApi { forums ->
            runOnUiThread {
                Log.d("Found Forums: ", forums.toString())
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
        val ownerForumMap = forums.filter { it.ownerId != null }.groupBy { it.ownerId }
        val addedForumIds = mutableSetOf<Int>() // Track added forum IDs

        Log.d("MainActivity", "Total forums count: ${forums.size}")

        forums.filter { it.ownerId == null }.forEach { forum ->
            addForumAndItsOwned(menu, forum, ownerForumMap, addedForumIds)
        }
    }

    fun addForumAndItsOwned(menu: Menu, forum: Forum, ownerForumMap: Map<Int?, List<Forum>>, addedForumIds: MutableSet<Int>) {
        if (forum.id in addedForumIds) return

        // Add the main forum
        addForumToMenu(menu, forum, addedForumIds, forum.ownerId == null)

        // Add forums owned by this forum
        ownerForumMap[forum.id]?.forEach { ownedForum ->
            addForumAndItsOwned(menu, ownedForum, ownerForumMap, addedForumIds)
        }
    }

    fun addForumToMenu(menu: Menu, forum: Forum, addedForumIds: MutableSet<Int>, withIcon: Boolean) {
        val menuItem = menu.add(R.id.forum_group, forum.id, Menu.NONE, forum.title)
        if (withIcon) {
            menuItem.setIcon(if (forum.isLocked) R.drawable.forum else R.drawable.chat)
        }
        menuItem.setOnMenuItemClickListener {
            val bundle = bundleOf("forumId" to forum.id, "forumTitle" to forum.title, "description" to forum.description)

            val destinationId = if (forum.isLocked) R.id.nav_dynamic_Read else R.id.nav_dynamic_Write

            val navInflater = navController.navInflater
            val navGraph = navInflater.inflate(R.navigation.mobile_navigation)
            navGraph.findNode(destinationId)?.label = forum.title
            navController.graph = navGraph

            navController.navigate(destinationId, bundle)
            drawerLayout.closeDrawer(GravityCompat.START)
            true
        }

        Log.d("MainActivity", "Added forum: ${forum.id}")
        addedForumIds.add(forum.id) // Track added forum IDs
    }

    fun removeForumNavViews() {
        val menu = navView.menu
        addedForumMenuIds.forEach { menuId ->
            menu.removeItem(menuId)
        }
        addedForumMenuIds.clear()
    }

    fun getCredentials(): JSONObject {
        return userData ?: JSONObject().apply {
            put("userId", "dummyUserId")
            put("accessToken", "dummyAccessToken")
        }
    }
}
