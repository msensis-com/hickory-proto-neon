// This module loads the platform-specific build of the addon on
// the current system. The supported platforms are registered in
// the `platforms` object below, whose entries can be managed by
// by the Neon CLI:
//
//   https://www.npmjs.com/package/@neon-rs/cli

module.exports = require("@neon-rs/load").proxy({
  platforms: {
    "win32-x64-msvc": () => require("@msensis/hickory-proto-neon-win32-x64-msvc"),
    "linux-x64-gnu": () => require("@msensis/hickory-proto-neon-linux-x64-gnu"),
  },
  debug: () => require("../index.node"),
});
