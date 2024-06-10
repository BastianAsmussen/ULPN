package app.ulpn.ui.admin

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import app.ulpn.databinding.FragmentAdminBinding
import app.ulpn.ui.admin.AdminViewModel
import io.noties.markwon.Markwon

class AdminFragment : Fragment() {

    private lateinit var markwon: Markwon
    private var _binding: FragmentAdminBinding? = null
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val adminViewModel =
            ViewModelProvider(this)[AdminViewModel::class.java]

        _binding = FragmentAdminBinding.inflate(inflater, container, false)
        val root: View = binding.root



        // Accessing arguments and updating text
        arguments?.getString("description")?.let {
            adminViewModel.updateText(it)
        }

        return root
    }


    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
