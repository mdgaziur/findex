# Customization

Findex can be customized by applying properties to certain css classes. Below is a table containing
the class name and what they correspond to:

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

To customize Findex, edit the style.css file in `~/.config/findex`. If you can't find the file, run Findex to generate that. After that, you
can modify the appearance of Findex. If Findex isn't opening after editing stylesheet, run `findex` in terminal to see why the stylesheet is invalid.
