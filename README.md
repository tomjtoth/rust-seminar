# Rust Seminar Demo

This is a demo for a HY course.

## Styles via Tailwind CSS

Run the below commands:

```bash
npm install
npm run dev:css
```

## Running on Desktop

Followed [this guide](https://dioxuslabs.com/learn/0.6/getting_started/#linux), installed deps, everything worked out of the box.

## Running on Android

Followed [this guide](https://dioxuslabs.com/learn/0.6/guides/mobile/#getting-set-up) with the below deviations:

- had **`jdk24-openjdk`, which was too new for gradle 8.9** (used internally by `dx serve`)
  - installed `jdk21-openjdk`
- also installed `android-ndk`
- and set env vars on Arch:

```sh
export JAVA_HOME="/usr/lib/jvm/java-21-openjdk"
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="/opt/android-ndk"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

**SERVE YOUR APP NOW**

After running the command from the next section, I located and copied over the `.apk` to my home folder:

```sh
find target -name '*.apk' -exec cp {} "$HOME/" \; -quit
```

From where I transferred (via USB) and installed it manually to my phone.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform (web):

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. e.g.

```bash
# for the desktop
dx serve --platform desktop

# for Android, exposing server over WiFi
SERVER_URL=http://DEV_MACHINE_IP_ON_WIFI:8080 dx serve --platform android --addr 0.0.0.0
```
