package app.ulpn.ui

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import app.ulpn.databinding.FragmentForumBinding

class ForumFragment : Fragment() {

    private var _binding: FragmentForumBinding? = null
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentForumBinding.inflate(inflater, container, false)
        val root: View = binding.root

        val forumId = arguments?.getInt("forumId") ?: 0
        // Use the forumId to fetch or display the forum details

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
