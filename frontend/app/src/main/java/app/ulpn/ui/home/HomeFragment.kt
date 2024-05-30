package app.ulpn.ui.home

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import app.ulpn.databinding.FragmentHomeBinding
import io.noties.markwon.Markwon

class HomeFragment : Fragment() {

    private var _binding: FragmentHomeBinding? = null
    private lateinit var markwon: Markwon

    // This property is only valid between onCreateView and
    // onDestroyView.
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

        val textView = binding.textHome
        markwon = Markwon.create(requireContext())
        homeViewModel.text.observe(viewLifecycleOwner) { markdownText ->
            markwon.setMarkdown(textView, markdownText)
        }

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
