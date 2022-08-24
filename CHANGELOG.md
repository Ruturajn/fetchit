# Changelog:

### 24th August, 2022
- Fixed Unknown OS issue, the file `/etc/os-release` not being read.
- Added support for some more WMs by checking `_NET_WM_NAME`, if the `XDG` environment variables are not set.
- Refactored `match` and `if - else` statements for color argument handling &#8594; [#5](https://github.com/Ruturajn/fetchit/pull/5).
- Handling of the packages count improved &#8594; [#3](https://github.com/Ruturajn/fetchit/pull/3).
- Added Hostname detection &#8594; [#6](https://github.com/Ruturajn/fetchit/pull/6).
- Reduced binary size from `9.5 MB` to `770 KB`.
- `fetchit` now available in the `AUR` &#8594; [#4](https://github.com/Ruturajn/fetchit/pull/4).
