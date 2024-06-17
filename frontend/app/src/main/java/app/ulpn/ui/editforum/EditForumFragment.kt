package app.ulpn.ui.editforum

import android.graphics.Typeface
import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.TextView
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.lifecycle.Observer
import androidx.lifecycle.ViewModelProvider
import app.ulpn.ApiManager
import app.ulpn.R
import app.ulpn.databinding.FragmentEditforumBinding
import app.ulpn.ui.Forum

class EditForumFragment : Fragment() {

    private var _binding: FragmentEditforumBinding? = null
    private val binding get() = _binding!!

    private lateinit var editForumViewModel: EditForumViewModel

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentEditforumBinding.inflate(inflater, container, false)
        val root: View = binding.root

        // Initialize ViewModel and observe forums
        val context = requireContext()
        val apiManager = ApiManager(context)
        val editForumViewModelFactory = EditForumViewModelFactory(apiManager)
        editForumViewModel = ViewModelProvider(this, editForumViewModelFactory).get(EditForumViewModel::class.java)

        // Observe the forums list and update the UI
        editForumViewModel.forums.observe(viewLifecycleOwner, Observer { forums ->
            updateForumsUI(forums)
        })

        // Handle add forum button click
        binding.addForumButton.setOnClickListener {
            val newTitle = binding.newForumTitleEditText.text.toString()
            val newDescription = binding.newForumDescriptionEditText.text.toString()
            val isLocked = binding.newForumLockedCheckBox.isChecked
            val accessLevel = binding.newForumAccessLevelEditText.text.toString()
            val ownerId = binding.newForumOwnerIdEditText.text.toString().toIntOrNull()


            // Call ViewModel to add the new forum
            editForumViewModel.addForum(newTitle, newDescription, isLocked, accessLevel, ownerId) { success ->
                if (success) {
                    Toast.makeText(context, "Forum added successfully", Toast.LENGTH_SHORT).show()
                    // Clear input fields after successful addition if needed
                    binding.newForumTitleEditText.text.clear()
                    binding.newForumDescriptionEditText.text.clear()
                    binding.newForumLockedCheckBox.isChecked = false
                    binding.newForumAccessLevelEditText.text.clear()
                    binding.newForumOwnerIdEditText.text.clear()
                } else {
                    Toast.makeText(context, "Failed to add forum", Toast.LENGTH_SHORT).show()
                }
            }
        }

        return root
    }

    private fun updateForumsUI(forums: List<Forum>) {
        val containerLayout = binding.containerLayout
        containerLayout.removeAllViews()

        // Sort forums by id first
        val sortedForums = forums.sortedBy { it.id }

        // Create a map to group forums by ownerId
        val forumsByOwnerId = sortedForums.groupBy { it.ownerId }

        // Iterate through sorted forums and add views
        sortedForums.forEach { forum ->
            val forumView = LayoutInflater.from(requireContext()).inflate(R.layout.item_forum, containerLayout, false)

            val titleTextView = forumView.findViewById<TextView>(R.id.titleTextView)
            val titleEditText = forumView.findViewById<EditText>(R.id.titleEditText)
            val descriptionEditText = forumView.findViewById<EditText>(R.id.descriptionEditText)
            val idTextView = forumView.findViewById<TextView>(R.id.idTextView)
            val ownerIdTextView = forumView.findViewById<TextView>(R.id.ownerIdTextView)
            val saveButton = forumView.findViewById<Button>(R.id.saveButton)
            val deleteButton = forumView.findViewById<Button>(R.id.deleteButton)

            titleTextView.text = forum.title
            titleEditText.setText(forum.title)
            descriptionEditText.setText(forum.description)
            idTextView.text = forum.id.toString()
            ownerIdTextView.text = forum.ownerId.toString()

            saveButton.setOnClickListener {
                val newTitle = titleEditText.text.toString()
                val newDescription = descriptionEditText.text.toString()
                val isLocked = forum.isLocked
                val accessLevel = forum.accessLevel

                editForumViewModel.saveForum(forum.id, newTitle, newDescription, isLocked, accessLevel) { success ->
                    if (success) {
                        Toast.makeText(context, "Forum saved successfully", Toast.LENGTH_SHORT).show()
                    } else {
                        Toast.makeText(context, "Failed to save forum", Toast.LENGTH_SHORT).show()
                    }
                }
            }

            deleteButton.setOnClickListener {
                editForumViewModel.deleteForum(forum.id) { success ->
                    if (success) {
                        Toast.makeText(context, "Forum deleted successfully", Toast.LENGTH_SHORT).show()
                    } else {
                        Toast.makeText(context, "Failed to delete forum", Toast.LENGTH_SHORT).show()
                    }
                }
            }

            containerLayout.addView(forumView)
        }
    }




    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }

    companion object {
        private const val TAG = "EditForumFragment"
    }
}
