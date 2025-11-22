# Dioxus version

This is a Dioxus demo for a HY course, managing to get the same codebase to be served over the browser, desktop and Android.

## Styles via Tailwind CSS

Run the below commands:

```bash
npm install
npm run dev:css
```

## Running on Desktop (Arch Linux)

Followed [this guide](https://dioxuslabs.com/learn/0.7/getting_started/#linux), installed deps, everything worked out of the box.

## Running on Android

Followed [this guide](https://dioxuslabs.com/learn/0.7/guides/platforms/mobile) with the below deviations:

- had **`jdk24-openjdk`, which was too new for gradle 8.9** (used internally by `dx serve`)
  - installed `jdk21-openjdk`
- also installed `android-ndk`
- and set env vars as below:

```sh
export JAVA_HOME="/usr/lib/jvm/java-21-openjdk"
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="/opt/android-ndk"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

**SERVE YOUR APP NOW** (next section)

Once the server is up and running, locate the generated `.apk` using a separate terminal window:

```sh
find target -name '*.apk'
```

Transfer (via USB) and install it manually to your phone.

### Serving Your App

Run one of the following commands in the root of the project to start developing for the web:

```sh
dx serve
```

... for the desktop:

```sh
dx serve --platform desktop
```

... for Android (exposing server over WiFi)

```sh
SERVER_URL=http://DEV_MACHINE_IP_ON_WIFI:8080 dx serve --platform android --addr 0.0.0.0
```
