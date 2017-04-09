var googleUser = {};
var initGoogleAuth = function(appId) {
    gapi.load('auth2', function(){
        // Retrieve the singleton for the GoogleAuth library and set up the client.
        auth2 = gapi.auth2.init({
            client_id: appId,
            cookiepolicy: 'single_host_origin',
            // Request scopes in addition to 'profile' and 'email'
            //scope: 'additional_scope'
        });
        googleAttachSignin(document.getElementById('google-signin-button'));
    });
};

function googleAttachSignin(element) {
    console.log(element.id);
    auth2.attachClickHandler(element, {},
        function(googleUser) {
            onSignIn(googleUser);
        });
}

function onSignIn(googleUser) {
  var profile = googleUser.getBasicProfile();
  var id_token = googleUser.getAuthResponse().id_token;
  console.log('ID: ' + profile.getId()); // Do not send to your backend! Use an ID token instead.
  console.log('ID Token: ' + id_token);
  console.log('Name: ' + profile.getName());

  window.location.href = "http://localhost:3010/google-auth?username=" + profile.getName() + '&auth_token=' + id_token + '&id=' + profile.getId();
}

function googleSignOut() {
    var auth2 = gapi.auth2.getAuthInstance();
    auth2.signOut().then(function () {
    console.log('User signed out.');
});
}
