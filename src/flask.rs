use crate::os::mkfile;
use indoc::indoc;
use std::io::Write;

use super::os;

pub fn create_flask_env() {
    os::mkdir(String::from("templates"));
    os::mkdir(String::from("static"));

    // content for base.html
    let mut content = indoc! {"
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\">
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
                {% block head %}
                {% endblock %}
            </head>
            <body>
            {block body %}
            {% endblock %}
            </body>
            <html>
            "
    };

    os::cwd(String::from("templates"));

    let mut html_file = mkfile(String::from("base.html"));
    html_file
        .write_all(content.as_bytes())
        .expect("Error. File not wriiten");

    content = indoc! {"
        {% extends 'base.html' %}

        {% block head %}
        <title>My Flask app</title>
        {% endblock %}


        {% block body %}
    

        {% endblock %}
        "
    };

    html_file = mkfile(String::from("index.html"));
    html_file
        .write_all(content.as_bytes())
        .expect("Error. File not written");
}
