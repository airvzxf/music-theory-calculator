package net.rovisoft.tonicmusic

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowDropDown
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Checkbox
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.ScrollableTabRow
import androidx.compose.material3.Tab
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen
import net.rovisoft.tonicmusic.ui.theme.TonicMusicTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        // Install Splash Screen before super.onCreate()
        installSplashScreen()
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            TonicMusicTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    TonicMusicApp(modifier = Modifier.padding(innerPadding))
                }
            }
        }
    }
}

enum class Accidental(val symbol: String) {
    NATURAL("♮"), SHARP("♯"), FLAT("♭")
}

enum class AppMode(val label: String) {
    SCALE("Scale"), CHORD("Chord"), HARMONIZE("Harmonize"), PROGRESSION("Progression")
}

// Helper to format FfiNote enum names to readable strings with symbols
fun FfiNote.toDisplayString(): String {
    return this.name.replace("_SHARP", "♯").replace("SHARP", "♯")
}

// Helper to format ChordType enum names
fun FfiChordType.toDisplayString(): String {
    return when (this) {
        FfiChordType.MAJOR -> "Maj"
        FfiChordType.MINOR -> "min"
        FfiChordType.DIMINISHED -> "dim"
        FfiChordType.AUGMENTED -> "aug"
        FfiChordType.MAJOR7 -> "Maj7"
        FfiChordType.MINOR7 -> "min7"
        FfiChordType.DOMINANT7 -> "7"
        FfiChordType.MINOR7B5 -> "m7b5"
        FfiChordType.DIMINISHED7 -> "dim7"
        FfiChordType.MINOR_MAJOR7 -> "mM7"
        FfiChordType.AUGMENTED_MAJOR7 -> "augM7"
    }
}

fun FfiScaleType.toDisplayString(): String {
    return this.name.lowercase().replace("_", " ")
        .replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
}

fun FfiHarmonicFormula.toDisplayString(): String {
    return this.name.lowercase().replace("_", " ")
        .replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
}

@Composable
fun TonicMusicApp(modifier: Modifier = Modifier) {
    var selectedBaseNote by remember { mutableStateOf("C") }
    var selectedAccidental by remember { mutableStateOf(Accidental.NATURAL) }
    var selectedMode by remember { mutableStateOf(AppMode.SCALE) }
    var resultText by remember { mutableStateOf("") }
    var errorText by remember { mutableStateOf<String?>(null) }

    // Mode-specific states
    var selectedScaleType by remember { mutableStateOf(FfiScaleType.MAJOR) }
    var selectedChordType by remember { mutableStateOf(FfiChordType.MAJOR) }
    var showInversions by remember { mutableStateOf(false) } // State for Inversions
    var harmonizeSevenths by remember { mutableStateOf(false) }

    // Progression states
    var isCustomProgression by remember { mutableStateOf(false) }
    var progressionFormula by remember { mutableStateOf(FfiHarmonicFormula.BLOCK) }
    var customProgressionText by remember { mutableStateOf("") }

    val keyboardController = LocalSoftwareKeyboardController.current
    val baseNotes = listOf("C", "D", "E", "F", "G", "A", "B")

    fun getFfiNote(): FfiNote {
        // C=0, C#=1, D=2, D#=3, E=4, F=5, F#=6, G=7, G#=8, A=9, A#=10, B=11
        val baseValue = when (selectedBaseNote) {
            "C" -> 0
            "D" -> 2
            "E" -> 4
            "F" -> 5
            "G" -> 7
            "A" -> 9
            "B" -> 11
            else -> 0
        }

        val modifier = when (selectedAccidental) {
            Accidental.NATURAL -> 0
            Accidental.SHARP -> 1
            Accidental.FLAT -> -1
        }

        // Calculate generic integer value (mod 12)
        var finalValue = (baseValue + modifier) % 12
        if (finalValue < 0) finalValue += 12

        return when (finalValue) {
            0 -> FfiNote.C
            1 -> FfiNote.C_SHARP
            2 -> FfiNote.D
            3 -> FfiNote.D_SHARP
            4 -> FfiNote.E
            5 -> FfiNote.F
            6 -> FfiNote.F_SHARP
            7 -> FfiNote.G
            8 -> FfiNote.G_SHARP
            9 -> FfiNote.A
            10 -> FfiNote.A_SHARP
            11 -> FfiNote.B
            else -> FfiNote.C
        }
    }

    fun executeAction() {
        errorText = null
        try {
            val root = getFfiNote()
            val result = when (selectedMode) {
                AppMode.SCALE -> {
                    val notes = getScaleNotes(root, selectedScaleType)
                    "Scale: ${selectedScaleType.toDisplayString()}\nNotes: " + notes.joinToString(", ") { it.toDisplayString() }
                }

                AppMode.CHORD -> {
                    val notes = getChordNotes(root, selectedChordType)
                    val chordName =
                        "${root.toDisplayString()} ${selectedChordType.toDisplayString()}"

                    var output = "$chordName: " + notes.joinToString("-") { it.toDisplayString() }

                    if (showInversions) {
                        // Polyfill for Inversions since FFI doesn't support it yet
                        val inversions = mutableListOf<List<FfiNote>>()
                        val currentRotation = notes.toMutableList()
                        // Rotate N-1 times
                        (1 until notes.size).forEach { _ ->
                            // Rotate left
                            val first = currentRotation.removeAt(0)
                            currentRotation.add(first)
                            inversions.add(currentRotation.toList())
                        }

                        inversions.forEachIndexed { index, invNotes ->
                            output += "\nInv ${index + 1}: " + invNotes.joinToString("-") { it.toDisplayString() }
                        }
                    }
                    output
                }

                AppMode.HARMONIZE -> {
                    val degrees = getHarmonization(root, selectedScaleType, harmonizeSevenths)
                    degrees.joinToString("\n") { degree ->
                        val roman = when (degree.degree.toInt()) {
                            1 -> "I"
                            2 -> "II"
                            3 -> "III"
                            4 -> "IV"
                            5 -> "V"
                            6 -> "VI"
                            7 -> "VII"
                            else -> degree.degree.toString()
                        }
                        "$roman: ${degree.rootNote.toDisplayString()} ${degree.chordType.toDisplayString()} (${
                            degree.notes.joinToString(
                                "-"
                            ) { it.toDisplayString() }
                        })"
                    }
                }

                AppMode.PROGRESSION -> {
                    if (isCustomProgression) {
                        val chords = getCustomProgression(root, customProgressionText)
                        chords.joinToString("\n") { chord ->
                            "${chord.degree}: ${chord.rootNote.toDisplayString()} ${chord.chordType.toDisplayString()} (${
                                chord.notes.joinToString(
                                    "-"
                                ) { it.toDisplayString() }
                            })"
                        }
                    } else {
                        val chords = getProgression(root, progressionFormula)
                        chords.joinToString("\n") { chord ->
                            "${chord.degree}: ${chord.rootNote.toDisplayString()} ${chord.chordType.toDisplayString()} (${
                                chord.notes.joinToString(
                                    "-"
                                ) { it.toDisplayString() }
                            })"
                        }
                    }
                }
            }
            resultText = result
        } catch (e: Exception) {
            errorText = "Error: ${e.message}"
            e.printStackTrace()
        }
    }

    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp)
            .verticalScroll(rememberScrollState()),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text("Tonic Music", style = MaterialTheme.typography.displaySmall)

        // --- Root Note Selection ---
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceVariant)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text("Root Note", style = MaterialTheme.typography.labelLarge)
                Spacer(modifier = Modifier.height(8.dp))
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceEvenly
                ) {
                    baseNotes.forEach { note ->
                        FilterChip(
                            selected = selectedBaseNote == note,
                            onClick = { selectedBaseNote = note },
                            label = { Text(note) })
                    }
                }
                Spacer(modifier = Modifier.height(8.dp))
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceEvenly
                ) {
                    Accidental.entries.forEach { acc ->
                        FilterChip(
                            selected = selectedAccidental == acc,
                            onClick = { selectedAccidental = acc },
                            label = { Text(acc.symbol) } // Only symbol for space
                        )
                    }
                }
            }
        }

        // --- Mode Tabs ---
        androidx.compose.material3.PrimaryScrollableTabRow(selectedTabIndex = selectedMode.ordinal, edgePadding = 0.dp) {
            AppMode.entries.forEach { mode ->
                Tab(
                    selected = selectedMode == mode,
                    onClick = { selectedMode = mode },
                    text = { Text(mode.label) })
            }
        }

        // --- Mode Specific Inputs ---
        Card(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 8.dp)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                when (selectedMode) {
                    AppMode.SCALE -> {
                        Text("Select Scale Type:")
                        SimpleDropdown(
                            items = FfiScaleType.entries,
                            selectedItem = selectedScaleType,
                            onItemSelected = { selectedScaleType = it },
                            itemLabel = { it.toDisplayString() })
                    }

                    AppMode.CHORD -> {
                        Text("Select Chord Type:")
                        SimpleDropdown(
                            items = FfiChordType.entries,
                            selectedItem = selectedChordType,
                            onItemSelected = { selectedChordType = it },
                            itemLabel = { it.toDisplayString() })
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable { showInversions = !showInversions }
                        ) {
                            Checkbox(
                                checked = showInversions,
                                onCheckedChange = { showInversions = it }
                            )
                            Text("Show Inversions")
                        }
                    }

                    AppMode.HARMONIZE -> {
                        Text("Harmonize Scale:")
                        // Filter out Pentatonic scales for Harmonize
                        val validScales = FfiScaleType.entries.filter {
                            !it.name.contains("PENTATONIC")
                        }

                        // Ensure selected is valid
                        LaunchedEffect(validScales) {
                            if (!validScales.contains(selectedScaleType)) {
                                selectedScaleType = validScales.first()
                            }
                        }

                        SimpleDropdown(
                            items = validScales,
                            selectedItem = selectedScaleType,
                            onItemSelected = { selectedScaleType = it },
                            itemLabel = { it.toDisplayString() })
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable { harmonizeSevenths = !harmonizeSevenths }
                        ) {
                            Checkbox(
                                checked = harmonizeSevenths,
                                onCheckedChange = { harmonizeSevenths = it })
                            Text("Use 7th Chords")
                        }
                    }

                    AppMode.PROGRESSION -> {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            RadioButton(
                                selected = !isCustomProgression,
                                onClick = { isCustomProgression = false })
                            Text("Standard")
                            Spacer(Modifier.width(16.dp))
                            RadioButton(
                                selected = isCustomProgression,
                                onClick = { isCustomProgression = true })
                            Text("Custom")
                        }

                        if (isCustomProgression) {
                            OutlinedTextField(
                                value = customProgressionText,
                                onValueChange = { customProgressionText = it },
                                label = { Text("Formula (e.g., I IV V)") },
                                modifier = Modifier.fillMaxWidth(),
                                singleLine = true,
                                keyboardOptions = KeyboardOptions(imeAction = ImeAction.Done),
                                keyboardActions = KeyboardActions(
                                    onDone = {
                                        keyboardController?.hide()
                                        executeAction()
                                    }
                                )
                            )
                        } else {
                            SimpleDropdown(
                                items = FfiHarmonicFormula.entries,
                                selectedItem = progressionFormula,
                                onItemSelected = { progressionFormula = it },
                                itemLabel = { it.toDisplayString() })
                        }
                    }
                }
            }
        }

        // --- Generate Button ---
        Button(
            onClick = { executeAction() }, modifier = Modifier.fillMaxWidth()
        ) {
            Text("GENERATE")
        }

        // --- Output ---
        if (errorText != null) {
            Card(colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.errorContainer)) {
                Text(
                    text = errorText!!,
                    color = MaterialTheme.colorScheme.onErrorContainer,
                    modifier = Modifier.padding(16.dp)
                )
            }
        } else if (resultText.isNotEmpty()) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.secondaryContainer)
            ) {
                Text(
                    text = resultText,
                    modifier = Modifier.padding(16.dp),
                    style = MaterialTheme.typography.bodyLarge
                )
            }
        }
    }
}

@Composable
fun <T> SimpleDropdown(
    items: List<T>, selectedItem: T, onItemSelected: (T) -> Unit, itemLabel: (T) -> String
) {
    var expanded by remember { mutableStateOf(false) }

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp)
            .clip(RoundedCornerShape(4.dp))
            .background(MaterialTheme.colorScheme.surfaceVariant)
            .clickable { expanded = true }
            .padding(16.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(text = itemLabel(selectedItem))
            Icon(Icons.Default.ArrowDropDown, contentDescription = "Dropdown")
        }

        DropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false },
            modifier = Modifier.fillMaxWidth(0.9f)
        ) {
            items.forEach { item ->
                DropdownMenuItem(text = { Text(itemLabel(item)) }, onClick = {
                    onItemSelected(item)
                    expanded = false
                })
            }
        }
    }
}
