package app.ulpn.ui.forum_chat

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import app.ulpn.databinding.FragmentForumChatBinding

class ForumChatFragment : Fragment() {

    private var _binding: FragmentForumChatBinding? = null
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val forumChatViewModel =
            ViewModelProvider(this)[ForumChatViewModel::class.java]

        _binding = FragmentForumChatBinding.inflate(inflater, container, false)
        val root: View = binding.root


        // Accessing arguments and updating text
        arguments?.getString("description")?.let {
            forumChatViewModel.updateText(it)
        }

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
