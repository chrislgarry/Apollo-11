// Get the container element
const sceneContainer = document.getElementById('scene-container');

// Create a scene
const scene = new THREE.Scene();

// Create a camera
const camera = new THREE.PerspectiveCamera(75, sceneContainer.clientWidth / sceneContainer.clientHeight, 0.1, 1000);
camera.position.z = 5;

// Create a renderer
const renderer = new THREE.WebGLRenderer();
renderer.setSize(sceneContainer.clientWidth, sceneContainer.clientHeight);
sceneContainer.appendChild(renderer.domElement);

// Add lighting
const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
scene.add(ambientLight);

const pointLight = new THREE.PointLight(0xffffff, 0.5);
pointLight.position.set(2, 3, 4);
scene.add(pointLight);

// Create a loader
const loader = new THREE.GLTFLoader();

// Placeholder URLs for the models
const rocketUrl = 'https://raw.githubusercontent.com/USER/REPO/main/rocket.glb';
const capsuleUrl = 'https://raw.githubusercontent.com/USER/REPO/main/capsule.glb';
const landerUrl = 'https://raw.githubusercontent.com/USER/REPO/main/lander.glb';

let rocket, capsule, lander;

function loadModel(url, onLoad) {
    loader.load(url, (gltf) => {
        const model = gltf.scene;
        scene.add(model);
        onLoad(model);
    }, undefined, (error) => {
        console.error(`An error happened loading ${url}: ${error}`);
    });
}

loadModel(rocketUrl, (model) => {
    rocket = model;
    rocket.position.y = -2;
});

loadModel(capsuleUrl, (model) => {
    capsule = model;
    capsule.position.y = 0;
});

loadModel(landerUrl, (model) => {
    lander = model;
    lander.position.y = 2;
});


// Instantiate the AGC simulator
const agc = new AGCSimulator();

// Animation loop
function animate() {
    requestAnimationFrame(animate);

    // Update lander position based on AGC memory
    const resultAddress = agc.labels ? agc.labels['RESULT'] : undefined;
    if (resultAddress !== undefined) {
        const resultValue = agc.rom[resultAddress];
        if (lander) {
            lander.position.y = 2 + resultValue * 0.1;
        }
    }


    // Rotate the objects for some visual interest
    if (rocket) rocket.rotation.y += 0.01;
    if (capsule) capsule.rotation.y += 0.01;
    if (lander) lander.rotation.y += 0.01;

    renderer.render(scene, camera);
}

animate();

// UI event listeners
document.getElementById('load-program').addEventListener('click', () => {
    fetch('program.agc')
        .then(response => response.text())
        .then(program => {
            agc.loadProgram(program);
            console.log("Program loaded.");
        });
});

document.getElementById('step').addEventListener('click', () => {
    agc.step();
    logRegisters();
});

let runInterval = null;
const runButton = document.getElementById('run');

runButton.addEventListener('click', () => {
    if (runInterval) {
        clearInterval(runInterval);
        runInterval = null;
        runButton.textContent = 'Run';
        runButton.classList.remove('running');
    } else {
        runInterval = setInterval(() => {
            agc.step();
            logRegisters();
        }, 100);
        runButton.textContent = 'Stop';
        runButton.classList.add('running');
    }
});

function logRegisters() {
    console.log(`A: ${agc.A}, L: ${agc.L}, Q: ${agc.Q}, Z: ${agc.Z}`);
    const resultAddress = agc.labels['RESULT'];
    if (resultAddress !== undefined) {
        console.log(`RESULT: ${agc.rom[resultAddress]}`);
    }
}
