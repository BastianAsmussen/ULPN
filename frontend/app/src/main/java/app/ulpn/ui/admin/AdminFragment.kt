package app.ulpn.ui.admin

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.TableLayout
import android.widget.TableRow
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import app.ulpn.ApiManager
import app.ulpn.R
import app.ulpn.databinding.FragmentAdminBinding

class AdminFragment : Fragment() {

    private var _binding: FragmentAdminBinding? = null
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentAdminBinding.inflate(inflater, container, false)
        val root: View = binding.root

        val context = requireContext()
        val apiManager = ApiManager(context)
        val adminViewModelFactory = AdminViewModelFactory(apiManager)
        val adminViewModel = ViewModelProvider(this, adminViewModelFactory)[AdminViewModel::class.java]

        // Observe the settings list and update the UI
        adminViewModel.settings.observe(viewLifecycleOwner) { settings ->
            updateSettingsUI(settings, adminViewModel)
        }

        return root
    }

    private fun updateSettingsUI(settings: List<Pair<String, String>>, viewModel: AdminViewModel) {
        val tableLayout = binding.tableLayout // Using view binding
        settings.forEach { setting ->
            val tableRow = TableRow(context)
            val linearLayout = LinearLayout(context).apply {
                orientation = LinearLayout.VERTICAL
            }

            val textView = TextView(context).apply {
                text = setting.first
                layoutParams = LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.WRAP_CONTENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                )
            }

            val editText = EditText(context).apply {
                setText(setting.second)
                layoutParams = LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.WRAP_CONTENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                )
            }

            val saveButton = Button(context).apply {
                text = "Save"
                layoutParams = LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.WRAP_CONTENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                )
                setOnClickListener {
                    val newValue = editText.text.toString()
                    viewModel.saveSetting(setting.first, newValue)
                }
            }

            linearLayout.addView(textView)
            linearLayout.addView(editText)
            linearLayout.addView(saveButton)
            tableRow.addView(linearLayout)
            tableLayout.addView(tableRow)
        }
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
