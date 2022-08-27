export function draw(canvas_name, width, data) {
    let canvas = document.getElementById(canvas_name);
    let canvas_ctx = canvas.getContext("2d");
    let bitmap = new ImageData(new Uint8ClampedArray(data), width);

    canvas_ctx.putImageData(bitmap, 0, 0);
}

function scaleImageData(originalImageData, targetWidth, targetHeight) {
    const targetImageData = new ImageData(targetWidth, targetHeight);
    const h1 = originalImageData.height;
    const w1 = originalImageData.width;
    const h2 = targetImageData.height;
    const w2 = targetImageData.width;
    const kh = h1 / h2;
    const kw = w1 / w2;
    const cur_img1pixel_sum = new Int32Array(4);
    for (let i2 = 0; i2 < h2; i2 += 1) {
        for (let j2 = 0; j2 < w2; j2 += 1) {
            for (let i in cur_img1pixel_sum) cur_img1pixel_sum[i] = 0;
            let cur_img1pixel_n = 0;
            for (let i1 = Math.ceil(i2 * kh); i1 < (i2 + 1) * kh; i1 += 1) {
                for (let j1 = Math.ceil(j2 * kw); j1 < (j2 + 1) * kw; j1 += 1) {
                    const cur_p1 = (i1 * w1 + j1) * 4;
                    for (let k = 0; k < 4; k += 1) {
                        cur_img1pixel_sum[k] += originalImageData.data[cur_p1 + k];
                    };
                    cur_img1pixel_n += 1;
                };
            };
            const cur_p2 = (i2 * w2 + j2) * 4;
            for (let k = 0; k < 4; k += 1) {
                targetImageData.data[cur_p2 + k] = cur_img1pixel_sum[k] / cur_img1pixel_n;
            };
        };
    };
    return targetImageData;
};

export function cpu_cores() {
    return navigator.hardwareConcurrency;
}

export function download_blob(blob, fileName) {
    var a = document.createElement("a");
    document.body.appendChild(a);
    a.style = "display: none";

    var url = window.URL.createObjectURL(blob);
    a.href = url;
    a.download = fileName;
    a.click();
    window.URL.revokeObjectURL(url);
}

export function blob_from_str(data) {
    return new Blob([data], {
        type: "text/plain",
    });
}

export function event_to_file(event) {
    return event.target.files[0];
}
