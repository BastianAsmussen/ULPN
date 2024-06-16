package app.ulpn.ui.home

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
import com.auth0.android.authentication.AuthenticationException
import com.auth0.android.callback.Callback
import com.auth0.android.provider.WebAuthProvider
import com.auth0.android.result.Credentials
import com.google.android.gms.auth.api.signin.GoogleSignIn
import com.google.android.gms.auth.api.signin.GoogleSignInClient
import com.google.android.gms.auth.api.signin.GoogleSignInOptions
import com.google.android.material.button.MaterialButton
import org.json.JSONObject

class HomeFragment : Fragment() {

    private var _binding: FragmentHomeBinding? = null
    private val binding get() = requireNotNull(_binding)

    private lateinit var viewModel: HomeViewModel
    private lateinit var mGoogleSignInClient: GoogleSignInClient
    private lateinit var signInButton: MaterialButton

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentHomeBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        // Initialize ViewModel using ViewModelProvider with HomeViewModelFactory
        val apiManager = ApiManager(requireContext())
        val factory = HomeViewModelFactory(apiManager)
        viewModel = ViewModelProvider(this, factory).get(HomeViewModel::class.java)

        // Observe changes to title and description
        viewModel.title.observe(viewLifecycleOwner) { title ->
            binding.title.text = title ?: ""
        }

        viewModel.description.observe(viewLifecycleOwner) { description ->
            binding.description.text = description ?: ""
        }

        // Initialize Google Sign-In options
        val gso = GoogleSignInOptions.Builder(GoogleSignInOptions.DEFAULT_SIGN_IN)
            .requestEmail()
            .build()

        // Initialize Google Sign-In Client
        mGoogleSignInClient = GoogleSignIn.getClient(requireActivity(), gso)

        // Set up Google Sign-In Button
        signInButton = binding.loginWithGoogleButton
        updateButtonState(signInButton)
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }

    // Google Sign-In method
    private fun signIn() {
        val activity = requireActivity() as MainActivity
        val account = activity.account

        WebAuthProvider.login(account)
            .withScheme("demo")
            .withScope("openid profile email")
            .start(requireActivity(), object : Callback<Credentials, AuthenticationException> {
                override fun onFailure(exception: AuthenticationException) {
                    Toast.makeText(requireContext(), "Failed to log in!", Toast.LENGTH_SHORT).show()
                }

                override fun onSuccess(credentials: Credentials) {
                    val json = JSONObject()
                    json.put("userId", credentials.user.getId().toString())
                    json.put("accessToken", credentials.accessToken)

                    activity.userData = json
                    updateButtonState(signInButton)
                    activity.fetchForums(activity.apiManager)

                    // Fetch settings after successful login
                    activity.fetchSettings(activity.apiManager)
                }
            })
    }

    // Google Sign-Out method
    private fun signOut() {
        val activity = requireActivity() as MainActivity
        val account = activity.account

        WebAuthProvider.logout(account)
            .withScheme("demo")
            .start(requireActivity(), object : Callback<Void?, AuthenticationException> {
                override fun onFailure(error: AuthenticationException) {
                    Log.e(TAG, "Failed to log out!", error)
                }

                override fun onSuccess(payload: Void?) {
                    // Handle logout success
                    activity.userData = null
                    updateButtonState(signInButton)
                }
            })
    }

    // Update button state based on sign-in status
    private fun updateButtonState(button: MaterialButton) {
        val activity = requireActivity() as MainActivity
        if (activity.userData != null) {
            button.text = getString(R.string.logout)
            button.setOnClickListener { signOut() }
        } else {
            button.text = getString(R.string.login_with_google)
            button.setOnClickListener { signIn() }
        }
    }

    companion object {
        private const val TAG = "HomeFragment"
    }
}
