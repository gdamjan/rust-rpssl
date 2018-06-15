# rock-paper-scissors-spock-lizard in rust

```
GET / -> main page
POST / -d '' -> Redirect to /<uuid>/

GET /<uuid>/ -> game page
POST /<uuid>/attack -d attack=scissors|rock|paper|spock|lizard ->
    long poll,
    sends attack to game actor
    game actor only returns when it has 2 hands played
    returns result of the play

```

written in Rust, using Actix and Actix-Web

see also:
https://github.com/gdamjan/erlang-rpssl-comet
