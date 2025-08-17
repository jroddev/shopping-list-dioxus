// Service Worker for Shopping List PWA
const CACHE_NAME = "shopping-list-v1";
const STATIC_CACHE_NAME = "shopping-list-static-v1";
const DYNAMIC_CACHE_NAME = "shopping-list-dynamic-v1";

// Files to cache for offline functionality
const STATIC_FILES = [
  "/",
  "/assets/styling/main.css",
  "/assets/favicon.ico",
  "/assets/pwa/manifest.json",
  "/assets/pwa/icon-32.png",
  "/assets/pwa/icon-192.png",
  "/assets/pwa/icon-410.png",
];

// Install event - cache static files
self.addEventListener("install", (event) => {
  console.log("Service Worker installing...");
  event.waitUntil(
    caches
      .open(STATIC_CACHE_NAME)
      .then((cache) => {
        console.log("Caching static files");
        return cache.addAll(STATIC_FILES);
      })
      .then(() => {
        console.log("Static files cached successfully");
        return self.skipWaiting();
      })
      .catch((err) => {
        console.error("Failed to cache static files:", err);
      }),
  );
});

// Activate event - clean up old caches
self.addEventListener("activate", (event) => {
  console.log("Service Worker activating...");
  event.waitUntil(
    caches
      .keys()
      .then((cacheNames) => {
        return Promise.all(
          cacheNames.map((cacheName) => {
            if (
              cacheName !== STATIC_CACHE_NAME &&
              cacheName !== DYNAMIC_CACHE_NAME
            ) {
              console.log("Deleting old cache:", cacheName);
              return caches.delete(cacheName);
            }
          }),
        );
      })
      .then(() => {
        console.log("Service Worker activated");
        return self.clients.claim();
      }),
  );
});

// Fetch event - serve from cache, fallback to network
self.addEventListener("fetch", (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== "GET") {
    return;
  }

  // Handle API requests differently
  if (url.pathname.startsWith("/api/")) {
    event.respondWith(
      fetch(request)
        .then((response) => {
          // Clone the response before caching
          const responseClone = response.clone();

          // Cache successful API responses
          if (response.ok) {
            caches.open(DYNAMIC_CACHE_NAME).then((cache) => {
              cache.put(request, responseClone);
            });
          }

          return response;
        })
        .catch(() => {
          // Return cached version if network fails
          return caches.match(request);
        }),
    );
    return;
  }

  // Handle static files and pages
  event.respondWith(
    caches.match(request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      // If not in cache, fetch from network
      return fetch(request)
        .then((response) => {
          // Don't cache non-successful responses
          if (!response.ok) {
            return response;
          }

          // Clone the response
          const responseClone = response.clone();

          // Cache the response
          caches.open(DYNAMIC_CACHE_NAME).then((cache) => {
            cache.put(request, responseClone);
          });

          return response;
        })
        .catch(() => {
          // If network fails and we don't have a cached version,
          // return a fallback page for navigation requests
          if (request.destination === "document") {
            return caches.match("/");
          }
        });
    }),
  );
});

// Handle background sync for offline actions
self.addEventListener("sync", (event) => {
  console.log("Background sync event:", event.tag);

  if (event.tag === "background-sync") {
    event.waitUntil(
      // Here you could implement offline queue processing
      Promise.resolve(),
    );
  }
});

// Handle push notifications (for future enhancement)
self.addEventListener("push", (event) => {
  console.log("Push notification received");

  const options = {
    body: event.data ? event.data.text() : "New update available",
    icon: "/assets/pwa/icon-192.png",
    badge: "/assets/pwa/icon-32.png",
    tag: "shopping-list-notification",
    renotify: true,
  };

  event.waitUntil(self.registration.showNotification("Shopping List", options));
});

// Handle notification click
self.addEventListener("notificationclick", (event) => {
  console.log("Notification clicked");

  event.notification.close();

  event.waitUntil(clients.openWindow("/"));
});

// Log service worker messages
self.addEventListener("message", (event) => {
  console.log("Service Worker received message:", event.data);

  if (event.data && event.data.type === "SKIP_WAITING") {
    self.skipWaiting();
  }
});
