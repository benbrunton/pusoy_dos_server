'use strict';

self.addEventListener('push', function(event) {
    //console.log('[Service Worker] Push received');
    //console.log('[Service Worker] Push data: ', event.data.text());

    var promiseChain = clients.matchAll()
        .then(function(clients) {
            var showNotification = true;
            var client;
            if (clients.length) {
                for (var i = 0; i < clients.length; i++) {
                    if (clients[i].visibilityState === 'visible') {
                        client = clients[i];
                        showNotification = false;
                        break;
                    }
                }
            }

            if (showNotification) {
                return self.registration.getNotifications({ tag: 'moves' })
                    .then(function(notifications) {

                        var data = event.data.json();

                        var title = data.title || 'Pusoy Dos';
                        var options = {
                            body: data.body || 'Missing message!',
                            icon: data.icon || '/public/img/push_icon.png',
                            tag: data.tag || null,
                            actions: data.actions || [],
                            data: data.data || null,
                            renotify: true,
                        };

                        if (notifications.length) {
                            if (!notifications[0].data.moves) {
                                notifications[0].data.moves = 2;
                            } else {
                                notifications[0].data.moves++;
                            }
                            title = 'You have ' + notifications[0].data.moves + ' moves to make';
                            options = {
                                body: 'Click here to see your games',
                                icon: data.icon || '/public/img/push_icon.png',
                                tag: 'moves',
                                data: notifications[0].data,
                                renotify: true,
                            };
                        }

                        return self.registration.showNotification(title, options);
                    })
            } else {
                //if page is open on games list page, just refresh page
                if (client.url.endsWith('games')) {
                    client.navigate('/games');
                }
            }
        });
    event.waitUntil(promiseChain);
});

self.addEventListener('notificationclick', function(event) {
    //console.log('[Service Worker] Notification Clicked', event);
    event.notification.close();

    //change action based on event.action

    var url = (event.notification.data.moves) ? '/games' : '/game/' + event.notification.data.game;

    event.waitUntil(clients.matchAll({
        type: 'window'
    })
        .then(function(clients) {
            for (var i = 0; i < clients.length; i++) {
                if ('focus' in clients[i] && 'navigate' in clients[i]) {
                    clients[i].navigate(url)
                    return clients[i].focus();
                }
            }

            if (clients.openWindow) {
                clients.openWindow(url);
            }
        }));
});
