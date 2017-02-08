
let swRegistration;
let isSubscribed = false;

//if not registered
let enableHeader = document.createElement('div');

enableHeader.innerHTML = '<button class="enable-notifications pure-button action-button">Allow Notifications</button>';
let container = document.querySelector('.container');
container.insertBefore(enableHeader, container.firstChild);
let pushButton = document.querySelector('.enable-notifications');

if ('serviceWorker' in navigator && 'PushManager' in window) {
  console.log('Service Worker and Push is supported');

  navigator.serviceWorker.register('/public/js/sw.js')
  .then(function(swReg) {
    console.log('Service Worker is registered', swReg);

    swRegistration = swReg;
    initialiseUI();
  })
  .catch(function(error) {
    console.error('Service Worker Error', error);
  });
} else {
  console.warn('Push messaging is not supported');
  pushButton.textContent = 'Push Not Supported';
}

//Notification.requestPermission();
function initialiseUI() {
  pushButton.addEventListener('click', function() {
    pushButton.disabled = true;
    if (isSubscribed) {
      // TODO: Unsubscribe user
    } else {
      subscribeUser();
    }
  });

  // Set the initial subscription value
  swRegistration.pushManager.getSubscription()
  .then(function(subscription) {
    isSubscribed = !(subscription === null);

    updateSubscriptionOnServer(subscription);

    if (isSubscribed) {
      console.log('User IS subscribed.');
    } else {
      console.log('User is NOT subscribed.');
    }

  });
}

function subscribeUser() {
  swRegistration.pushManager.subscribe({userVisibleOnly: true})
  .then(function(subscription) {
    console.log('User is subscribed.');

    updateSubscriptionOnServer(subscription);

    isSubscribed = true;
  })
  .catch(function(err) {
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
