# rdb-client
This is a client used to interact with [rdb](http://rdb.dual-iron.xyz/), the **R**ealm **d**ata**b**ase of mods.

## Submitting a mod
You should have a few things ready:
- The URL to your mod icon. must be a 128x128 PNG or JPG
- The URL to your mod binary. must be a GitHub release asset, Google Drive file, or Discord attachment
- Somewhere to write a password down for later

Once you've got that, just [download](https://github.com/Dual-Iron/rdb-client/releases/latest) and run the program.

## Extra help
FIELD|DESCRIPTION
--|--
secret|Essentially a password. Write this down somewhere and keep it *secret*.*
name|The mod's name.
owner|The person or team of people that made the mod.
version|The mod's current [version](https://semver.org/) that increases after each update.
description|[Optional] A brief description of the mod.
icon|A URL to a 128x128 PNG or JPG file.
binary|A URL to a GitHub release asset, Google Drive file, or Discord attachment. Should be a ZIP or DLL file.
homepage|[Optional] A URL users can visit to learn more about the mod.

\* When submitting a new mod, the `secret` field can be anything you want. When updating the mod later, the `secret` field must match, or you won't be able to update your mod. If you lose your secret, contact Dual (Discord ID [303617148411183105](https://discord.id)).
