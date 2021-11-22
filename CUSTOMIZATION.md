# Customization

Findex can be customized by applying properties to certain css classes. Below is a table containing class names and what they correspond to:

| Class                  | Widget                                                            |
|------------------------|-------------------------------------------------------------------|
| findex-window          | Top level window                                                  |
| findex-container       | Top level container of all widgets                                |
| findex-query           | Text input box where user gives query                             |
| findex-results-scroll  | Scrollable container containing ListBox that shows search results |
| findex-results         | Listbox containing search results                                 |
| findex-result-row      | ListBoxRow containing single search result                        |
| findex-result-icon     | App icon of result row                                            |
| findex-result-app-name | App name of result row                                            |
| findex-result-command  | The command that'll be used to launch the app                     |

To customize Findex, edit the style.css file in `~/.config/findex`. If there is no such file, run Findex to generate it.

Behaviour can be changed by modifying `~/.config/findex/settings.toml`. If there is no such file, run Findex to generate it.

| Name                          | Description                                              | Type    |
|-------------------------------|----------------------------------------------------------|---------|
| default_window_width          | Set default width of the window                          | Integer |
| min_content_height            | Minimum content height of result                         | Integer |
| max_content_height            | Maximum content height of result                         | Integer |
| max_name_fuzz_result_score    | Maximum score for fuzzing result of application names    | Float   |
| max_command_fuzz_result_score | Maximum score for fuzzing result of application commands | Float   |
| max_fuzz_distance             | Maximum fuzzing distance                                 | Integer |
| decorate_window               | Show toolbar of window                                   | Boolean |
| query_placeholder             | Placeholder text to show in query input box              | String  |
