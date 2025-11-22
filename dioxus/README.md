# Dioxus version

This is a Dioxus demo for a HY course, managing to get the same codebase to be served over the browser, desktop and Android.

## Styles via Tailwind CSS

Run the below commands:

```bash
npm install
npm run dev:css
```

## Running on Desktop (Arch Linux)

Followed [this guide](https://dioxuslabs.com/learn/0.6/getting_started/#linux), installed deps, everything worked out of the box.

## Running on Android

Followed [this guide](https://dioxuslabs.com/learn/0.6/guides/mobile/#android) with the below deviations:

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

- built the binary with the SERVER_URL set to my laptop's IP

  ```sh
   SERVER_URL=http://DEV_MACHINE_IP_ON_WIFI:8080 dx build --platform android
  ```

- locate the generated `.apk` using a separate terminal window

  ```sh
  find -name '*.apk'
  ```

- transferred (via USB) and installed it manually to my phone

### Serving Your App

Run one of the following commands in the root of the project to start developing for

- the web:

  ```sh
  dx serve
  ```

- the desktop:

  ```sh
  dx serve --platform desktop
  ```

- Android emulators:

  ```sh
  dx serve --platform android
  ```
