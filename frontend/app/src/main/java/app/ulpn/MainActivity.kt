package app.ulpn

import android.os.Bundle
import android.util.Log
import android.view.Menu
import com.google.android.material.snackbar.Snackbar
import com.google.android.material.navigation.NavigationView
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import androidx.drawerlayout.widget.DrawerLayout
import androidx.appcompat.app.AppCompatActivity
import androidx.core.os.bundleOf
import androidx.navigation.NavArgument
import androidx.navigation.fragment.FragmentNavigator
import androidx.navigation.get
import app.ulpn.databinding.ActivityMainBinding
import app.ulpn.R

data class Forum(val id: Int, val title: String)

class MainActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding

    // List of forums
    private val forums = listOf(
        Forum(1, "Forum 1"),
        Forum(2, "Forum 2"),
        Forum(3, "Forum 3")
    )

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.appBarMain.toolbar)

        binding.appBarMain.fab.setOnClickListener { view ->
            Snackbar.make(view, "Replace with your own action", Snackbar.LENGTH_LONG)
                .setAction("Action", null)
                .setAnchorView(R.id.fab).show()
        }
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
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.main, menu)

        // Get the group to which new items will be added
        val group = menu.findItem(R.id.forum_group)?.subMenu

        // Get the NavController
        val navController = findNavController(R.id.nav_host_fragment_content_main)

        // Iterate over the list of forums and add menu items for each forum
        forums.forEach { forum ->
            Log.d("ForumDebug", "Adding forum with ID: ${forum.id}")
            val menuItem = group?.add(Menu.NONE, forum.id, Menu.NONE, forum.title)
                ?.setIcon(R.drawable.ic_menu_camera)

            menuItem?.setOnMenuItemClickListener {
                // Create a Bundle for arguments
                val args = bundleOf("forumId" to forum.id)

                // Navigate to the new destination
                navController.navigate(forum.id, args)

                true
            }

            // Add the destination to the navigation graph if it doesn't already exist
            if (navController.graph.findNode(forum.id) == null) {
                val destination = FragmentNavigator.Destination(
                    navController.navigatorProvider.getNavigator(FragmentNavigator::class.java)
                ).apply {
                    id = forum.id
                    addArgument("forumId", NavArgument.Builder().setDefaultValue(forum.id).build())
                }
                navController.graph.addDestination(destination)
            }
        }

        return true
    }


    override fun onSupportNavigateUp(): Boolean {
        val navController = findNavController(R.id.nav_host_fragment_content_main)
        return navController.navigateUp(appBarConfiguration) || super.onSupportNavigateUp()
    }
}
