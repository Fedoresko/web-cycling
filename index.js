// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import { default as init, WebClient } from "./pkg/webgl_water_tutorial.js";
const rust = import('./pkg');

init("./pkg/webgl_water_tutorial_bg.wasm").then( ctx => {
  const webClient = new WebClient()
  webClient.start()
} );

