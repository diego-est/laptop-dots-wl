<!-- LTeX: language=en,es -->

# dots

![The Rice](https://media.discordapp.net/attachments/635625917623828520/1085283400610488442/rice_comp.png)

## Colors
I used `pywal` to generate this colorscheme, feel free to do the same.
* [pywal GitHub](https://github.com/dylanaraps/pywal)

```
Black:   #222F30
Grey:    #A59D9C
Red:     #86999B
Green:   #86A78F
Yellow:  #8592A5
Blue:    #87B0AF
Magenta: #B0A7A9
Cyan:    #DCBCB6
White:   #EDE1E0
```

## fetch program
Provided is a rust crate called `fetch` with a basic template to make a hard-coded fetch program.
(That's the one I used for the screenshots.)
The program displays:
```

  UserName@HostName
  Distro  ~ DistroName
  Shell   ~ ShellName
  Term    ~ Terminal
  WM      ~ WindowManager
  ▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅

```

So simply modify the variables in the source file to fit your setup and `cargo build --release` to get the binary.
(Tip: I copied the binary over to `./local/bin/`.)

## Assets

### Firefox startpage
eject by Spogelsesmaskinen.
* [eject](https://twitter.com/spogelsemaskine/status/1607752472331689991) 
* [Spogelsesmankinen Twitter profile](https://twitter.com/spogelsemaskine)

### Wallpaper
Could not find the original artist for the background but I found this wallhaven post with a pretty high res version saved.
* https://whvn.cc/yxqj6l

## firefox startpage
### Startpage
***WARNING:*** _**do not** apply this without considering the vulnerabilities it opens up._
* [See this CVE](https://www.mozilla.org/en-US/security/advisories/mfsa2019-21/#CVE-2019-11730)

I copied most of what's in this:
(I added the lil' gif at the side and changed some of the website links.)
* https://github.com/okitavera/vimstart

In order to have this work you will have to let firefox to be able to view `file://` from within a locally hosted html file.
I'm guessing you could get around having to disable these security policies by locally hosting the html but I'm not quite there yet...

Provided is a user.js inside `.mozilla/firefox/default-release` with the necessary changes to make.
More specifically, these three lines (as documented in mozillazine):
```js
user_pref("capability.policy.policynames", "localfilelinks");
user_pref("capability.policy.localfilelinks.sites", "file://");
user_pref("capability.policy.localfilelinks.checkloaduri.enabled", "allAccess");
```
* [mozillazine](https://kb.mozillazine.org/Links_to_local_pages_do_not_work)

Additionally, if you are running the latest version of firefox (hopefully):
Toggle `security.fileuri.strict_origin_policy` to `false` in your `about:config` page.

### Other Firefox css tweaks
I copied `color_variable_template` and `autohide_tabstoolbar` templates from this incredible firefox css website:
* https://mrotherguy.github.io/firefox-csshacks/

The `userChrome.css` provided under `.mozilla/firefox/default-release/chrome` already has these css patches applied and should be placed inside the `chrome` directory that *you* have to make inside your firefox local directory.
Your firefox local directory can be found in `about:profiles`.

## Vim
I did not theme much of what's on my vim other than enabling a couple options.
Most of it (but basically all of it) comes from the amazing team over at AstroNvim .
Additionally, my nvim version is `0.8.0` and was installed through bob-nvim.
* [AstroNvim GitHub](https://github.com/AstroNvim/AstroNvim)
* [bob-nvim](https://crates.io/crates/bob-nvim)

## shell
As seen in my screenshot I am using nushell.
A new type of shell that deals with data instead of simple text streams.
Very interesting and fast out of the box (even on my 2008 hardware).
**NOTE** I have a bunch of aliases at the end of my nushell config file that replace most of the gnu coreutils with rust implementations of each.
* [nushell Website](https://www.nushell.sh/)
* [UUtils GitHub](https://github.com/uutils/coreutils)

## Sway
### I am not using the upstream sway.
I am using swayfx.
A fork of sway that adds corners, shadows and active/inactive dim.
That means that the options under the swayfx area in my sway config will most definitely give you errors when launching the upstream sway istead of swayfx sway.
* [swayfx GitHub](https://github.com/WillPower3309/swayfx)

### dvorak
***IMPORTANT***
My keybinds and input layout in sway are set to a dvorak keyboard.
So if you are using a normal qwerty keyboard remember to change the layout back to normal in the sway config and possibly change the sway keybinds around in the keymap config.
