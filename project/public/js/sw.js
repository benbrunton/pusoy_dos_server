'use strict';

//page should be able to clear notifications by sending message to sw
self.addEventListener('message', function(event){
    return self.registration.getNotifications()
        .then(function(notifications) {
            for (var i = 0; i < notifications.length; i++) {
                notifications[i].close();
            }
        });
});

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
                        var ddata = data.data;
                        if (typeof ddata === 'string') {
                            ddata = JSON.parse(ddata);
                        }

                        var title = data.title || 'Pusoy Dos';
                        var options = {
                            body: data.body || 'Missing message!',
                            icon: data.icon || '/public/img/push_icon.png',
                            tag: data.tag || null,
                            actions: data.actions || [],
                            data: ddata || null,
                            renotify: true,
                        };

                        if (notifications.length) {
                            var ndata = notifications[0].data;
                            if (typeof ndata === 'string') {
                                ndata = JSON.parse(notifications[0].data);
                            }

                            if (ndata.game !== ddata.game) {
                                if (!ndata.moves) {
                                    ndata.moves = 2;
                                } else {
                                    ndata.moves++;
                                }
                                title = 'You have ' + ndata.moves + ' moves to make';
                                options = {
                                    body: 'Click here to see your games',
                                    icon: data.icon || '/public/img/push_icon.png',
                                    tag: 'moves',
                                    data: ndata,
                                    renotify: true,
                                };
                            }
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
        .then(function(client) {
            for (var i = 0; i < client.length; i++) {
                if ('focus' in client[i] && 'navigate' in client[i]) {
                    client[i].navigate(url)
                    return client[i].focus();
                }
            }

            return clients.openWindow(url).then(function(client) {
                client.navigate(url);
                client.focus();
            });
        }));
});
