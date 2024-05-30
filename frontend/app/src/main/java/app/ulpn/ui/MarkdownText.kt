package app.ulpn.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.google.accompanist.markdown.Markdown
import com.google.accompanist.markdown.MarkdownTheme

@Composable
fun MarkdownText(markdown: String) {
    Markdown(
        content = markdown,
        modifier = Modifier.padding(16.dp),
        theme = MarkdownTheme(
            heading1 = TextStyle(fontSize = 24.sp),
            heading2 = TextStyle(fontSize = 20.sp),
            body1 = TextStyle(fontSize = 16.sp)
        )
    )
}
