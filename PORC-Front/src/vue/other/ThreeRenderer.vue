<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import * as THREE from 'three';
import { STLLoader } from 'three/addons/loaders/STLLoader.js';
import { mergeVertices } from 'three/addons/utils/BufferGeometryUtils.js';
import { HDRLoader } from 'three/examples/jsm/Addons.js';

const renderCanvas = ref<HTMLCanvasElement | null>(null);

function createGradientMap(): THREE.DataTexture {
    const colors = [
        new THREE.Color('#6B1E00'),
        new THREE.Color('#8C2D00'),
        new THREE.Color('#B84400'),
        new THREE.Color('#D96800'),
        new THREE.Color('#EFA31C'),
        new THREE.Color('#F8CD55'),
        new THREE.Color('#FFE99A'),
        new THREE.Color('#FFF5C7'),
    ];

    const width = 256;
    const data = new Uint8Array(width * 3);

    for (let x = 0; x < width; x++) {
        const t = x / (width - 1);

        const scaled = t * (colors.length - 1);
        const i = Math.floor(scaled);
        const f = scaled - i;

        const a = colors[Math.min(i, colors.length - 1)];
        const b = colors[Math.min(i + 1, colors.length - 1)];

        const color = a.clone().lerp(b, f);

        data[x * 3 + 0] = color.r * 255;
        data[x * 3 + 1] = color.g * 255;
        data[x * 3 + 2] = color.b * 255;
    }

    const texture = new THREE.DataTexture(
        data,
        width,
        1,
        THREE.RGBFormat
    );

    texture.needsUpdate = true;

    texture.minFilter = THREE.LinearFilter;
    texture.magFilter = THREE.LinearFilter;

    texture.wrapS = THREE.ClampToEdgeWrapping;
    texture.wrapT = THREE.ClampToEdgeWrapping;

    return texture;
}

onMounted(() => {
    const canvas = renderCanvas.value!;
    const pivot = new THREE.Group();
    const rotationAxis = new THREE.Vector3(0, 5, 1).normalize();

    // Renderer
    const renderer = new THREE.WebGLRenderer({
        canvas,
        antialias: true,
        alpha: false,
    });

    renderer.setSize(
        canvas.clientWidth,
        canvas.clientHeight,
        false
    );

    renderer.outputColorSpace = THREE.SRGBColorSpace;
    renderer.toneMapping = THREE.NoToneMapping;
    renderer.toneMappingExposure = 1.2;

    // Camera
    const camera = new THREE.PerspectiveCamera(
        75,
        canvas.clientWidth / canvas.clientHeight,
        0.1,
        100
    );

    camera.position.z = 50;

    // Scene
    const scene = new THREE.Scene();

    // --------------------
    // Lighting
    // --------------------

    const ambientLight = new THREE.AmbientLight(
        0xffea00,
        7.5
    );

    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(
        0xffffb0,
        10
    );

    directionalLight.position.set(10, 0, 4);

    scene.add(directionalLight);

    // const geometry = new THREE.BoxGeometry(1, 1, 1);

    // const material = new THREE.MeshBasicMaterial({
    //     color: 0x44aa88,
    // });

    // const cube = new THREE.Mesh(geometry, material);

    // scene.add(cube);

    // renderer.render(scene, camera);





    const pmremGenerator = new THREE.PMREMGenerator(renderer);

    new HDRLoader()
        .load('/hdr/studio.hdr', (hdr) => {
            const envMap = pmremGenerator.fromEquirectangular(hdr).texture;

            scene.environment = envMap;

            hdr.dispose();
            pmremGenerator.dispose();
        });






    // --------------------
    // Model
    // --------------------

    const loader = new STLLoader();

    loader.load('/models/shiftstones/Charge_Stone.stl', (geometry) => {
        // Center the geometry around the origin
        // geometry = new THREE.TorusKnotGeometry();
        geometry.center();

        // STL geometry is commonly non-indexed.
        // Remove the normals supplied by the STL first.
        geometry.deleteAttribute('normal');

        // Merge vertices that occupy the same position.
        geometry = mergeVertices(geometry, 1e-4);

        // Now that vertices are shared, calculate smooth normals.
        geometry.computeVertexNormals();

        console.log('indexed:', geometry.index !== null);
        console.log('vertices:', geometry.attributes.position.count);
        
        
        // --------------------
        // Cell Shading
        // --------------------


        let fiveTone = new THREE.TextureLoader().load('/models/gradientMaps/fiveTone.jpg');
        fiveTone.minFilter = THREE.NearestFilter
        fiveTone.magFilter = THREE.NearestFilter

        let material = new THREE.MeshToonMaterial({
            color: 0xfff457,
            gradientMap: fiveTone,
        });

        // material = new THREE.MeshNormalMaterial();

        material = new THREE.MeshPhysicalMaterial({
            color: 0xffffbf,
            roughness: 0,
            metalness: 0.7,
            flatShading: true,

            clearcoat: 0.5,
            clearcoatRoughness: 0.15,
        })

        const gradientMap = createGradientMap();
        gradientMap.minFilter = THREE.NearestFilter;
        gradientMap.magFilter = THREE.NearestFilter;

        // material = new THREE.MeshToonMaterial({
        //     color: 0xffff00,
        //     gradientMap: gradientMap,
        // });

        const outlineMaterial = new THREE.MeshBasicMaterial({
            color: 0x000000,
            side: THREE.BackSide,
        });

        const outline = new THREE.Mesh(
            geometry,
            outlineMaterial
        );

        outline.scale.multiplyScalar(1.03);

        const model = new THREE.Mesh(
            geometry,
            material
        );

        model.add(outline);

        pivot.add(model)

        scene.add(pivot);

        // --------------------
        // Animation
        // --------------------

        function animate() {
            requestAnimationFrame(animate);

            // pivot.rotation.x += 0.01
            pivot.rotateOnAxis(rotationAxis, 0.01);

            renderer.render(scene, camera);
        }

        animate();
    });
});
</script>

<template>
    <canvas ref="renderCanvas"></canvas>
</template>

<style scoped lang="scss">
canvas {
    width: 500px;
    height: 500px;

    color: #ffea00;
}
</style>
