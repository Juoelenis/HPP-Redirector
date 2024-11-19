class CSSParser:
    def __init__(self, css_text):
        self.css_text = css_text
        self.parsed_data = self.parse(css_text)

    def parse(self, css_text):
        """
        Parse the CSS text into a dictionary structure.
        """
        css_rules = {}
        
        # Remove comments
        css_text = self.remove_comments(css_text)

        # Split by closing braces
        blocks = css_text.split('}')

        for block in blocks:
            if '{' in block:
                selector, properties = block.split('{', 1)
                selector = selector.strip()
                properties = self.parse_properties(properties)

                if selector:
                    css_rules[selector] = properties

        return css_rules

    def parse_properties(self, properties_text):
        """
        Parse the properties within a CSS block.
        """
        properties = {}

        for line in properties_text.split(';'):
            if ':' in line:
                key, value = line.split(':', 1)
                properties[key.strip()] = value.strip()

        return properties

    def remove_comments(self, css_text):
        """
        Remove CSS comments (/* ... */).
        """
        import re
        return re.sub(r'/\*.*?\*/', '', css_text, flags=re.DOTALL)

    def get_rules(self):
        """
        Return the parsed CSS rules.
        """
        return self.parsed_data

# Example usage
if __name__ == "__main__":
    css_code = """
    /* Example CSS */
    body {
        background-color: white;
        color: black;
    }
    h1 {
        font-size: 24px;
        text-align: center;
    }
    """

    parser = CSSParser(css_code)
    parsed_css = parser.get_rules()

    for selector, properties in parsed_css.items():
        print(f"Selector: {selector}")
        for property_name, value in properties.items():
            print(f"  {property_name}: {value}")
        print()
