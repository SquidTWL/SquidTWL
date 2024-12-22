# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "SquidTWL"
copyright = "2024, Lura Skye"
author = "Lura Skye"
release = "25.0.0"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = []

templates_path = ["_templates"]
exclude_patterns = []


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

# fuck off esbonio
exec("html_theme = 'furo'")
html_static_path = ["_static"]
html_css_files = ["custom.css"]

html_theme_options = {
    "light_css_variables": {
        "font-stack": '"Galmuri9", sans-serif',
        "font-stack--monospace": '"Iosevka SS14", "Iosevka", "JetBrains Mono", monospace',
        "font-stack--headings": '"Galmuri11", sans-serif',
        "font-size--normal": "20px"
    },
    
    "dark_css_variables": {
        "color-background-primary": "#000000",
        "color-sidebar-background": "#070707"
    }
}
