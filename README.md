# README

Welcome to [RedwoodJS](https://redwoodjs.com)!
Powered through [Tauri 1.1](https://tauri.app/)!

This was quite a lazy integration BUUUUTTT it works great with a minor example on the home page.

There's an input field. If you enter a, say, comma separated list of numbers, it'll sum them if you click "calculate" button.
If you click "greet" button, you'll just be greeted with the message "Hello, ..." then actual string you typed in the input field.

This is here just to show how everything is connected and how you can run Rust (system) commands through RedwoodJS frontend.

Aye, let's get into it!

> **Prerequisites**
>
> - Redwood requires [Node.js](https://nodejs.org/en/) (>=14.19.x <=16.x) and [Yarn](https://yarnpkg.com/) (>=1.15)
>
> System dependencies:
> ```
> sudo apt update
> sudo apt install libwebkit2gtk-4.0-dev \
>     build-essential \
>     curl \
>     wget \
>     libssl-dev \
>     libgtk-3-dev \
>     libayatana-appindicator3-dev \
>     librsvg2-dev
> ```
>
> Install Rust:
> ```
> curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
> ```
> At this point, it's a good call to restart your terminal. :)

Start by installing dependencies:

```
yarn install
```

Then start the development server:

```
yarn redwood dev
```

Your browser should automatically open to http://localhost:8910 where you'll see the Welcome Page, which links out to a ton of great resources.

HOWEVER, we also use Tauri, which means Rust + Cargo:


To run Tauri and RW, run:
```
yarn tauri dev
```

RedwoodJS will be opened in the browser, however, feel free to close it at this moment.
I'll update at some point to not open the browser by default, felt kinda lazy at the moment.

## Next Steps

The best way to learn Redwood is by going through the comprehensive [tutorial](https://redwoodjs.com/docs/tutorial/foreword) and joining the community (via the [Discourse forum](https://community.redwoodjs.com) or the [Discord server](https://discord.gg/redwoodjs)).

The best way to learn Tauri: [Official Tauri docs](https://tauri.app/)

The best way to get around Rust: Visit some folks on YouTube, Google around, have fun. :)
