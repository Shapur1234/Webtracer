# [Webtracer](https://shapur1234.github.io/Webtracer/ "Link to live demo (mobile controls not supported)")
 * A pathtracer written in rust - runs in the web and includes an editor 
 * Rendering is parallelized and utilizes all cpu cores
 * You can *easily* edit scenes and move the camera (see [how to use](https://github.com/Shapur1234/Webtracer/edit/main/README.md#how-to-use))
 * Based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) tutorial series - supports spheres, bricks, lights, image-based textures... 
     * *note: images, due to DOM weirdness, have to be baked into the binary, so if you want to add your custom images, you'll have to rebuild the project (feel free to message me how to do so)*
 * Try out the live demo: [Live Demo](https://shapur1234.github.io/Webtracer/ "Link to live demo (mobile controls not supported)")
## Screenshots
![Screenshot of the UI](/example_images/screenshot.png?raw=true "Screenshot of the UI")
![Example render 1](/example_images/1.png?raw=true "Example render 1")
![Example render 2](/example_images/2.png?raw=true "Example render 2")
More images can be found [here](/example_images/ "Screenshot folder")
## How to use
* Upon loading the webpage, you will be in "normal mode", click onto the rendered image to enter "pointerlock mode" and move around using the mouse and keyboard
* Press [F] or [R] to render - the screen will go white, that is normal. Check progress in the console ([F12]). Download the output using [U]. Then press [Y] to re-unlock the screen
* Press [T] to toggle the settings panel
* You can also edit objects ([E], [C], [X], in "pointerlock mode") and easily mess around with different parameters
* To save and share the scenes you created, download and upload their the .xml representation (Buttons under settings)
* #### Controls: 
```
F           - Full render
R           - Preview render
Y           - Unlock screen after render
T           - Toggle settings
```
* #### Controls - "pointerlock mode": 
```
W, S, A, D  - Move
Arrow keys  - Move
J, K        - Move down, up
Mouse move  - Rotate camera
Q           - Reset camera to starting position
E           - Edit the object in the center of the screen
X           - Delete the object in the center of the screen
C           - Create a new object in the center of the screen
```

## How to build
Run the build [script](/build.sh) - rust and wasm-pack are requiered
Use a static file server like [sfz](https://github.com/weihanglo/sfz) to host.
