// MainActivity.kt
package app.ulpn

import android.os.Bundle
import android.util.Log
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
import app.ulpn.databinding.ActivityMainBinding
import app.ulpn.ui.Forum

class MainActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding
    private lateinit var apiManager: ApiManager

    private var forums = arrayListOf<Forum>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.appBarMain.toolbar)

        val drawerLayout: DrawerLayout = binding.drawerLayout
        val navView: NavigationView = binding.navView
        val navController = findNavController(R.id.nav_host_fragment_content_main)

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
        // Inside fetchForums callback
        apiManager.fetchForums { forums ->
            val menu = navView.menu
            forums.forEach { forum ->
                val menuItem = menu.add(R.id.forum_group, forum.id, Menu.NONE, forum.title)
                menuItem.setIcon(if (forum.is_locked) R.drawable.chat else R.drawable.forum)
                menuItem.setOnMenuItemClickListener {
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
            }
        }

    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        menuInflater.inflate(R.menu.main, menu)
        return true
    }

    override fun onSupportNavigateUp(): Boolean {
        val navController = findNavController(R.id.nav_host_fragment_content_main)
        return navController.navigateUp(appBarConfiguration) || super.onSupportNavigateUp()
    }
}
