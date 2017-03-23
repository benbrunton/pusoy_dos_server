(function(){
    'use strict';

    const applicationServerPublicKey = 'BFFMF4R7hS3cjTlKIoZ82Tk4mJrCi9IIS990_ypcaz79LRJxKowFI7T-T7oxcJfsBABAEqq2lnXAo_UMGheAmpk';

    function urlB64ToUint8Array(base64String) {
        const padding = '='.repeat((4 - base64String.length % 4) % 4);
        const base64 = (base64String + padding)
            .replace(/\-/g, '+')
            .replace(/_/g, '/');

        const rawData = window.atob(base64);
        const outputArray = new Uint8Array(rawData.length);

        for (let i = 0; i < rawData.length; ++i) {
            outputArray[i] = rawData.charCodeAt(i);
        }
        return outputArray;
    }

    if (!('serviceWorker' in navigator && 'PushManager' in window)) {
        console.warn('Push messaging is not supported');
        return;
    }

    console.log('Service Worker and Push is supported');

    navigator.serviceWorker.register('sw.js')
        .then(function(swRegistration) {
            console.log('Service Worker registered', swRegistration);

            init(swRegistration);
        })
        .catch(function(error) {
            console.log('Service Worker error', error);
        });

    function init(swRegistration) {

        swRegistration.pushManager.getSubscription()
            .then(function(subscription) {
                let isSubscribed = !(subscription === null);

                if (!isSubscribed) {
                    console.log('user not subscribed');
                    addNotice()
                        .addEventListener('click', function() {
                            this.disabled = true;
                            subscribeUser(swRegistration);
                        });
                } else {
                    console.log('user subscribed');
                    updateSubscriptionOnServer(subscription);
                }
            })
    }

    function addNotice(){
        let enableHeader = document.createElement('div');
        enableHeader.innerHTML = '<button class="enable-notifications pure-button action-button">Allow Notifications</button>';

        let container = document.querySelector('.container');
        let pushButton = enableHeader.querySelector('.enable-notifications');

        container.insertBefore(enableHeader, container.firstChild);

        return pushButton;
    }


    function subscribeUser(swRegistration) {
        const applicationServerKey = urlB64ToUint8Array(applicationServerPublicKey);
        swRegistration.pushManager.subscribe({
            userVisibleOnly: true,
            applicationServerKey: applicationServerKey,
        })
            .then(function(subscription) {
                console.log('User is subscribed');

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
                console.log('User is unsubscribed');
            });
    }

    function updateSubscriptionOnServer(subscription) {
      // TODO: Send subscription to application server
        fetch('http://localhost:8080', {
            method: 'post',
            headers: { 'Content-type': 'application/json' },
            body: JSON.stringify({ subscription: subscription })
        })
            .then(function(res) {
                console.log(res);
            })
            .catch(function(error) {
                console.log(error);
            });

      if (subscription) {
        console.log(subscription);
    //    console.log(JSON.stringify(subscription));
      }

    }
}());

