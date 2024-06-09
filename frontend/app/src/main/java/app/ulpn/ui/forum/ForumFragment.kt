package app.ulpn.ui.forum

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import app.ulpn.databinding.FragmentForumBinding
import app.ulpn.ui.forum.ForumViewModel
import io.noties.markwon.Markwon

class ForumFragment : Fragment() {

    private lateinit var markwon: Markwon
    private var _binding: FragmentForumBinding? = null
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val forumViewModel =
            ViewModelProvider(this)[ForumViewModel::class.java]

        _binding = FragmentForumBinding.inflate(inflater, container, false)
        val root: View = binding.root

        val textView = binding.textForum
        markwon = Markwon.create(requireContext())
        forumViewModel.text.observe(viewLifecycleOwner) { markdownText ->
            markwon.setMarkdown(textView, markdownText)
        }

        // Accessing arguments and updating text
        arguments?.getString("description")?.let {
            forumViewModel.updateText(it)
        }

        return root
    }


    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
