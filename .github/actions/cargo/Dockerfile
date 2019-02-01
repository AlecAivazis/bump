FROM rust:latest

LABEL "com.github.actions.name"="Cargo"
LABEL "com.github.actions.description"="Execute cargo things"
LABEL "com.github.actions.icon"="mic"
LABEL "com.github.actions.color"="Red"

LABEL "maintainer"="AlecAivazis <alec@aivazis.com>"

ADD entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
