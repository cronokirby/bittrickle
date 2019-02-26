# Bittrickle

A UDP bittorrent tracker in Rust, compliant with http://bittorrent.org/beps/bep_0015.html;

## Features

The tracker is mostly just an MVP at this point. It supports the standard announce and scrape
requests, but doesn't handle things like forgetting connection IDs after a certain time.

The implementation should be relatively fast, given how lightweight the code is.