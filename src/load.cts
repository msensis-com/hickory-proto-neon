// This module loads the platform-specific build of the addon on
// the current system. The supported platforms are registered in
// the `platforms` object below, whose entries can be managed by
// by the Neon CLI:
//
//   https://www.npmjs.com/package/@neon-rs/cli

module.exports = require("@neon-rs/load").proxy({
  platforms: {
    "windows-x64-gnu": () => require("@msensis/hickory-proto-neon-windows-x64-gnu"),
    "linux-x64-gnu": () => require("@msensis/hickory-proto-neon-linux-x64-gnu"),
  },
  debug: () => require("../index.node"),
});
