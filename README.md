# Introduction

`hatecli` is an incredibly simplistic command-line tool enabling you to upload text to [Hatebin](https://hatebin.com) via the command-line. It is easy to use and is swift and slick.

# Installation

To install, it is necessary you have the rust compiler installed. Once done, you can compile the crate. That's all!

# Usage

To upload basic text specified in the CLI, do:
`hatecli -t "I love HateCLI!"`

To upload text in a file, specify the file path like:
`hatecli -f A/File/Path.txt`

To upload a body found at another URL, specify the URL and `hatecli` fetches the content for you:
`hatecli -u "https://example.com`

Seeing as Hatebin has a maximum character limit of 50,000 characters, `hatecli` will not allow you to upload text that exceeds that limit by default. If you are okay with the text being truncated, add the force flag (`-F`)!
`hatecli -F -f A/Extremely/Large/File/Path.txt`