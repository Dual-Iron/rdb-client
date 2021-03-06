# rdb-client
This is a client used to interact with [rdb](http://rdb.dual-iron.xyz/), the **R**ealm **d**ata**b**ase of mods.

## Submitting a mod
You should have a few things ready:
- The URL to your mod icon. must be a 128x128 PNG or JPG
- The URL to your mod binary. must be a GitHub release asset, Google Drive file, or Discord attachment
- Somewhere to write a password down for later

Once you've got that, just [download](https://github.com/Dual-Iron/rdb-client/releases/latest) and run the program.

<details open>
<summary>What to put for each field</summary>

FIELD|DESCRIPTION
--|--
secret|Essentially a password. Write this down somewhere and keep it *secret*.*
name|The mod's name.
owner|The person or team of people that made the mod.
version|The mod's current [version](https://semver.org/) that increases after each update.
description|[Optional] A brief description of the mod.
icon|URL to a 128x128 PNG or JPG file.
binary|URL to a GitHub release asset, Google Drive file, or Discord attachment. Should be a ZIP or DLL file.
homepage|[Optional] A URL users can visit to learn more about the mod.

\* When submitting a new mod, the `secret` field can be anything you want. When updating the mod later, the `secret` field must match, or you won't be able to update your mod. If you lose your secret, contact Dual (Discord ID [303617148411183105](https://discord.id)).

</details>

## Integrating with GitHub
Submitting mods over and over can be tedious. To fix this, you can add a webhook to your GitHub repository. Every time you publish a release, it will be reflected on rdb.

<details open>
<summary>Video example of setup</summary>  

https://user-images.githubusercontent.com/31146412/163689916-df787775-ce33-478e-b7a1-0edac45585dd.mp4

</details>

<details open>
<summary>How the webhook works</summary>

FIELD|WHERE IT COMES FROM
--|--
secret|Given in the URL query parameter.
name|The repository's name.
owner|The repository's owner.
version|The release's tag name excluding any "v" prefix.
description|The repository description at the time of publishing.
icon|The file "icon.png" in the repository's root.
binary|The release's last asset when sorted alphabetically.
homepage|The repository's homepage or readme URL.

</details>
