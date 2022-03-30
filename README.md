# rdb-client
This is a client used to interact with rdb, the __R__ealm __d__ata__b__ase of mods.

Currently, this client just submits mods to the database. To submit a mod, run the program and fill in fields as prompted.

FIELD|DESCRIPTION
--|--
secret|Essentially a password. Write this down somewhere and keep it *secret*.*
name|The mod's name.
owner|The person(s) who made the mod.
version|The mod's current [version](https://semver.org/) that increases each update.
description|An optional brief description of the mod.
icon|A URL to a 128x128 PNG or JPG file.
binary|A URL to a GitHub release asset, Google Drive file, or Discord attachment. Should be a ZIP or DLL file.
homepage|An optional URL users can visit to learn more about the mod.

\* When submitting a new mod, the `secret` field can be anything you want. When updating the mod later, the `secret` field must match, or you won't be able to update your mod. If you lose your secret, contact Dual (Discord ID 303617148411183105).
