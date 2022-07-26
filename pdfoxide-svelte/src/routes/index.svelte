<script lang="ts">
    import * as pdfium from "pdfoxide-core";
    import init, {Document, PdfOxide} from "pdfoxide-core"
    import {onMount} from "svelte";

    let canvas;
    //let out_buffer = new Uint8Array(1000 * 1000 * 500);
    let pdfoxide: PdfOxide;

    async function initPdfOxide() {
        let initOut = await init();
        pdfium.initialize_pdfium_render(window.PDFIUM_MODULE, initOut, false);
        pdfoxide = new pdfium.PdfOxide();
    }

    async function loadDocument() {
        let response = await fetch('https://raw.githubusercontent.com/mozilla/pdf.js/ba2edeae/examples/learning/helloworld.pdf');
        let data = await response.arrayBuffer();
        let document = pdfoxide.load_document(new Uint8Array(data));
        return document;
    }

    function renderPage(document: Document, callback) {
        console.time("TOTAL");
        document.render_page(0, 2000, (width, height, buffer) => {
            callback(width, height, buffer)
            console.error(width, height);
            console.timeEnd("TOTAL");
        });
    }

    function createImageData(width: number, height: number, buffer: Uint8ClampedArray) {
        console.time("creating image data");
        canvas.width = width;
        canvas.height = height;
        let imgData = new ImageData(buffer, width, height);
        console.timeEnd("creating image data");
        return imgData;
    }

    function drawToCanvas(imgData: ImageData) {
        console.time("drawing to canvas");
        canvas.getContext("2d").putImageData(imgData, 0, 0);
        console.timeEnd("drawing to canvas");
    }

    onMount(async () => {
        await initPdfOxide();
        let document = await loadDocument();

        renderPage(document, (width, height, buffer: Uint8ClampedArray) => {
            console.warn(buffer);
            let imageData = createImageData(width, height, buffer)
            drawToCanvas(imageData);
        });
    });

</script>

<canvas bind:this={canvas}></canvas>