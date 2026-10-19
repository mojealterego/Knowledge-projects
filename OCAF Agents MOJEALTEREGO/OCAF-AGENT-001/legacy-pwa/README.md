# WDA Photo Agent — mobile PWA

A mobile-first virtual art director for professional photo workflows.

## What it does
- Upload a photo from camera/gallery.
- Send the photo to OpenAI for visual analysis and a structured retouching plan.
- Chat with the art director about preserving identity, changing backgrounds, lighting, color, composition, etc.
- Generate Firefly prompts directly from the chat.
- Call Adobe Firefly Services server-side to generate images.
- Keep API secrets off the mobile/browser client.
- Install as a PWA on Android/iOS-compatible browsers.

## API architecture
OpenAI is accessed through `/v1/responses` with `input_image` for image analysis. Adobe Firefly Services is accessed through its documented Generate Image API. Adobe documents Firefly generation at `https://firefly-api.adobe.io/v3/images/generate` and image upload at `/v2/storage/image`.

For production, add Adobe Photoshop API v2 and Firefly Fill/Remove Background as additional server routes. Photoshop API v2 supports unified editing operations and is the current production path; v1 reached end-of-life on 2026-07-31.

## Run locally

1. Copy `.env.example` to `.env` and fill in credentials.
2. Export those variables in the shell.
3. Run `node server.mjs`.
4. Open `http://localhost:8787` on the phone or desktop.

Example:

```bash
export OPENAI_API_KEY='...'
export OPENAI_MODEL='gpt-5.6-luna'
export FIREFLY_SERVICES_CLIENT_ID='...'
export FIREFLY_SERVICES_CLIENT_SECRET='...'
node server.mjs
```

## Production next steps
- Add user authentication and per-user credential vaulting.
- Add image/object masks and Firefly Fill.
- Add Adobe Photoshop API v2 workflows for Lightroom/Photoshop-style adjustments and PSD operations.
- Add original/edited version history and compare slider.
- Add background removal and object-aware retouching.
- Add export presets: Instagram, portfolio, editorial, print.
- Add durable job polling for async Firefly operations.
