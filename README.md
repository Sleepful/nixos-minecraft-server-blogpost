# NixOS Minecraft Server

This project is useful to set up a NixOS server instance with Minecraft.

In the `main` branch we have the basic server, it monitors active connections to the Minecraft port and shuts down the server if it does not see active players for 20 minutes.

In the `discord-channel-logging` branch we have a Discord webhook to send Minecraft logs to a Discord channel.

Find more about this project in the [blog post](https://bloggeroo.dev/articles/202402292320)!
