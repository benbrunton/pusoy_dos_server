'use strict';

self.addEventListener('push', function(event) {
    console.log('[Service Worker] Push received');
    console.log('[Service Worker] Push data: ', event.data.text());

    let data = event.data.json();

    const title = data.title || 'Pusoy Dos';
    const options = {
        body: data.body || 'Missing message!',
        icon: data.icon || '/public/img/push_icon.png',
    };

    event.waitUntil(self.registration.showNotification(title, options));
});

self.addEventListener('notificationclick', function(event) {
    console.log('[Service Worker] Notification Clicked');
    //TODO got to relevant game page
    event.notification.close();
});
