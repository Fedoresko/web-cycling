// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import { default as init, WebClient } from "./pkg/web_cycling.js";
const rust = import('./pkg');

init("./pkg/web_cycling_bg.wasm").then( ctx => {
  const webClient = new WebClient()
  webClient.start()
} );

