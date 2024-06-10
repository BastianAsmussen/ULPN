package app.ulpn.ui.home

import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import app.ulpn.ApiManager
import app.ulpn.MainActivity
import app.ulpn.R
import app.ulpn.databinding.FragmentHomeBinding
import app.ulpn.ui.User
import com.google.android.gms.auth.api.signin.GoogleSignIn
import com.google.android.gms.auth.api.signin.GoogleSignInClient
import com.google.android.gms.auth.api.signin.GoogleSignInOptions
import com.google.android.gms.common.api.ApiException
import com.google.android.material.button.MaterialButton
import io.noties.markwon.Markwon

class HomeFragment : Fragment() {

    private var _binding: FragmentHomeBinding? = null
    private lateinit var markwon: Markwon
    private lateinit var mGoogleSignInClient: GoogleSignInClient
    private val apiManager = ApiManager(requireContext())
    private val RC_SIGN_IN = 123 // Request code for Google Sign-In

    // This property is only valid between onCreateView and onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val homeViewModel =
            ViewModelProvider(this)[HomeViewModel::class.java]

        _binding = FragmentHomeBinding.inflate(inflater, container, false)
        val root: View = binding.root

        val textView = binding.description
        markwon = Markwon.create(requireContext())
        homeViewModel.text.observe(viewLifecycleOwner) { markdownText ->
            markwon.setMarkdown(textView, markdownText)
        }

        // Initialize Google Sign-In options
        val gso = GoogleSignInOptions.Builder(GoogleSignInOptions.DEFAULT_SIGN_IN)
            .requestEmail()
            .build()

        // Initialize Google Sign-In Client
        mGoogleSignInClient = GoogleSignIn.getClient(requireActivity(), gso)

        // Set up Google Sign-In Button
        val signInButton = binding.loginWithGoogleButton
        updateButtonState(signInButton)

        return root
    }

    override fun onDestroy() {
        super.onDestroy()
        signOut()
    }

    // Google Sign-In method
    private fun signIn() {
        val signInIntent = mGoogleSignInClient.signInIntent
        startActivityForResult(signInIntent, RC_SIGN_IN)
    }

    // Google Sign-Out method
    private fun signOut() {
        mGoogleSignInClient.signOut().addOnCompleteListener(requireActivity()) {
            updateButtonState(binding.loginWithGoogleButton)
        }
    }

    // Update button state based on sign-in status
    fun updateButtonState(button: MaterialButton) {
        val account = GoogleSignIn.getLastSignedInAccount(requireActivity())
        if (account != null) {
            button.text = getString(R.string.logout)
            button.setOnClickListener { signOut() }
        } else {
            button.text = getString(R.string.login_with_google)
            button.setOnClickListener { signIn() }
        }
    }

    // Handle Google Sign-In result
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)

        if (requestCode == RC_SIGN_IN) {
            val task = GoogleSignIn.getSignedInAccountFromIntent(data)
            try {
                // Google Sign-In was successful
                val account = task.getResult(ApiException::class.java)
                Log.d(TAG, "signInWithGoogle:" + account.id)
                val user = User.getInstance(account.displayName.toString(), account.email.toString())
                user.setAccessLevel(1)
                Log.d(TAG, "User logged in: ${user}")

                // Call the API to fetch JWT with the user's information
                apiManager.sendUserInfoToApi(user) { jwt ->
                    Log.d(TAG, "JWT fetched: $jwt")
                    // You can now use the JWT to fetch forums or any other protected resources

                    //TODO: Fetch Forums
                }

                // Update button state
                updateButtonState(binding.loginWithGoogleButton)

            } catch (e: ApiException) {
                // Google Sign-In failed
                Log.w(TAG, "Google sign in failed", e)
                // Display error message to user
                Toast.makeText(
                    requireContext(),
                    getString(R.string.google_sign_in_failed),
                    Toast.LENGTH_SHORT
                ).show()
            }
        }
    }



    companion object {
        private const val TAG = "HomeFragment"
    }
}
