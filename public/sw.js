// Minimal service worker for PWA functionality
self.addEventListener("install", (event) => {
  // Skip waiting to activate immediately
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  // Activate the new service worker
  return self.clients.claim();
});
