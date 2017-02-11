(function(){
    'use strict';

    if (!('serviceWorker' in navigator && 'PushManager' in window)) {
        console.warn('Push messaging is not supported');
        return;
    }

    console.log('Service Worker and Push is supported');

    // TODO - check whether permissions have been granted or dismissed before
    Promise.all([ 
        navigator.permissions.query({name:'push', userVisibleOnly:true}),
        navigator.serviceWorker.register('/public/js/sw.js')])
        .then(init)
        .catch(function(error) {
            console.error('Service Worker Error', error);
        });

    function init(result){

        let pushPermission = result[0];
        let swReg = result[1];

        if(pushPermission.state === 'granted'){
            console.log('permission is already granted');
            return;
        }

        addNotice()
            .addEventListener('click', function() {
                this.disabled = true;
                subscribeUser(swReg);
            });

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
        swRegistration.pushManager.subscribe({userVisibleOnly: true})
            .then(function(subscription) {
                console.log('User is subscribed.');
                updateSubscriptionOnServer(subscription);
          })
          .catch(function(err) {
            // TODO: could do with doing a bit more than this...
            console.log('Failed to subscribe the user: ', err);
          });
    }

    function updateSubscriptionOnServer(subscription) {
      // TODO: Send subscription to application server


      if (subscription) {
        console.log(subscription);
    //    console.log(JSON.stringify(subscription));
      }

    }
    
}());

