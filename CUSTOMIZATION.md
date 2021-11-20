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

To customize Findex, edit the style.css file in `~/.config/findex`. If there is no such file, run Findex to generate it.<br>
If Findex isn't being executed after modifying `~/.config/findex`, run `findex` in terminal to see what went wrong in the stylesheet.
