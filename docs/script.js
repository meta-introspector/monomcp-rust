document.addEventListener('DOMContentLoaded', () => {
    const imagePrompt = document.getElementById('image-prompt');
    const imageModel = document.getElementById('image-model');
    const generateImageBtn = document.getElementById('generate-image');
    const imageOutputUrl = document.getElementById('image-output-url');

    const videoPrompt = document.getElementById('video-prompt');
    const videoModel = document.getElementById('video-model');
    const generateVideoBtn = document.getElementById('generate-video');
    const videoOutputUrl = document.getElementById('video-output-url');

    generateImageBtn.addEventListener('click', () => {
        const prompt = encodeURIComponent(imagePrompt.value);
        const model = imageModel.value;
        const url = `https://example.com/image-generator?model=${model}&prompt=${prompt}`;
        imageOutputUrl.textContent = url;
    });

    generateVideoBtn.addEventListener('click', () => {
        const prompt = encodeURIComponent(videoPrompt.value);
        const model = videoModel.value;
        const url = `https://example.com/video-generator?model=${model}&prompt=${prompt}`;
        videoOutputUrl.textContent = url;
    });
});
