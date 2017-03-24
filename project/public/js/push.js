(function(){
    'use strict';

    var applicationServerPublicKey = 'BKriLwzXwDTwU6erJI69Bo-tH-3G2Ia1hFkN_rA5P6m05ZofcPPR6v1RcXcy7_FZNEgQK1uv91YB6APex3Fduik';

    function urlB64ToUint8Array(base64String) {
        var padding = '='.repeat((4 - base64String.length % 4) % 4);
        var base64 = (base64String + padding)
            .replace(/\-/g, '+')
            .replace(/_/g, '/');

        var rawData = window.atob(base64);
        var outputArray = new Uint8Array(rawData.length);

        for (var i = 0; i < rawData.length; ++i) {
            outputArray[i] = rawData.charCodeAt(i);
        }
        return outputArray;
    }

    if (!('serviceWorker' in navigator && 'PushManager' in window)) {
        console.warn('Push messaging is not supported');
        return;
    }

    //console.log('Service Worker and Push is supported');

    navigator.serviceWorker.register('/sw.js')
        .then(function(swRegistration) {
            //console.log('Service Worker registered', swRegistration);

            init(swRegistration);
        })
        .catch(function(error) {
            //console.log('Service Worker error', error);
        });

    function init(swRegistration) {

        if (Notification.permission === 'denied') {
            //console.log('Push Blocked');
            return;
        }

        swRegistration.pushManager.getSubscription()
            .then(function(subscription) {
                var isSubscribed = !(subscription === null);

                if (!isSubscribed) {
                    //console.log('user not subscribed');
                    addNotice()
                        .addEventListener('click', function() {
                            this.disabled = true;
                            subscribeUser(swRegistration);
                        });
                } else {
                    //console.log('user subscribed');
                    setTimeout(updateSubscriptionOnServer.bind(this, subscription), 2000);
                }
            })
    }

    function addNotice(){
        var enableHeader = document.createElement('div');
        enableHeader.innerHTML = '<button class="enable-notifications pure-button action-button">Allow Notifications</button>';

        var container = document.querySelector('.container');
        var pushButton = enableHeader.querySelector('.enable-notifications');

        container.insertBefore(enableHeader, container.firstChild);

        return pushButton;
    }


    function subscribeUser(swRegistration) {
        var applicationServerKey = urlB64ToUint8Array(applicationServerPublicKey);
        swRegistration.pushManager.subscribe({
            userVisibleOnly: true,
            applicationServerKey: applicationServerKey,
        })
            .then(function(subscription) {
                //console.log('User is subscribed');

                updateSubscriptionOnServer(subscription);
            })
            .catch(function(error) {
                console.log('Subscription Failed: ', error);
            });
    }

    function unsubscribeUser(swRegistration) {
        swRegistration.pushManager.getSubscription()
            .then(function(subscription) {
                if (subscription) {
                    return subscription.unsubscribe();
                }
            })
            .catch(function(error) {
                console.log('Error unsubscribing');
            })
            .then(function() {
                updateSubscriptionOnServer(null);
                //console.log('User is unsubscribed');
            });
    }

    function updateSubscriptionOnServer(subscription) {
        var game = Math.floor(Math.random() * 100);
        var body = {
            subscription: subscription,
            title: 'Your Move in game #' + game,
            body: 'Dean McGinty played Pair 2s',
            data: { game: game },
            actions: [
                { action: 'pass', title: 'Pass' }
            ],
            tag: 'moves',
        }
        fetch('http://localhost:8080', {
            method: 'post',
            headers: { 'Content-type': 'application/json' },
            body: JSON.stringify(body)
        })
            .then(function(res) {
                //console.log(res);
            })
            .catch(function(error) {
                //console.log(error);
            });
    }
}());

