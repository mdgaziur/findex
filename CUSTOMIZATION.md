# Customization

Findex can be customized by applying properties to certain css classes. Below is a table containing class names and what they correspond to:

| Class                          | Widget                                                            |
|--------------------------------|-------------------------------------------------------------------|
| findex-window                  | Top level window                                                  |
| findex-container               | Top level container of all widgets                                |
| findex-query                   | Text input box where user gives query                             |
| findex-results-scroll          | Scrollable container containing ListBox that shows search results |
| findex-results                 | Listbox containing search results                                 |
| findex-result-row              | ListBoxRow containing single search result                        |
| findex-result-icon             | App icon of result row                                            |
| findex-result-app-name         | App name of result row                                            |
| findex-result-app-description  | Description of the app                                            |
| findex-result-app-command      | Command of the app                                                |
| findex-result-trigger-shortcut | Shortcut for running the command in the result                    |
| findex-result-icon-container   | Container of the icon of each result                              |
| findex-result-info-container   | Container of the info of each result                              |
| findex-query-container         | Container of the search box                                       |

To customize Findex, edit the style.css file in `~/.config/findex`. If there is no such file, run Findex to generate it.

Behaviour can be changed by modifying `~/.config/findex/settings.toml`. If there is no such file, run Findex to generate it.

| Name                         | Description                                                                                                                     | Type    |
|------------------------------|---------------------------------------------------------------------------------------------------------------------------------|---------|
| default_window_width         | Set default width of the window                                                                                                 | Integer |
| min_content_height           | Minimum content height of result                                                                                                | Integer |
| max_content_height           | Maximum content height of result                                                                                                | Integer |
| name_match_highlight_color   | Color of matches highlighted in app name                                                                                        | String  |
| min_score                    | Minimum Score of app name match                                                                                                 | Integer |
| result_size                  | Maximum amount of apps to show as result                                                                                        | Integer |
| toggle_key                   | Key to toggle Findex(eg. `<Alt>space`). This doesn't work in Wayland. Check [INSTALLATION.md](./INSTALLATION.md) for more info. | String  |
| decorate_window              | Show toolbar of window                                                                                                          | Boolean |
| query_placeholder            | Placeholder text to show in query input box                                                                                     | String  |
| close_window_on_losing_focus | Close window when it loses focus                                                                                                | Boolean |
| icon_size                    | Icon width and height will be set from this value                                                                               | Integer |
