var googleUser = {};
var initGoogleAuth = function() {
    gapi.load('auth2', function(){
        // Retrieve the singleton for the GoogleAuth library and set up the client.
        auth2 = gapi.auth2.init({
            client_id: '251660764349-1ud95gk891fbl37jco4p4d4de4tpl84n.apps.googleusercontent.com',
            cookiepolicy: 'single_host_origin',
        });

        var signInButton = document.getElementById('google-signin-button');
        if (signInButton) {
            googleAttachSignin(document.getElementById('google-signin-button'));
        }

        var logoutButton = document.getElementById('logout-button');
        if (logoutButton) {
            console.log('logout button click handler bound');
            logoutButton.addEventListener('click', googleSignOut);
        }
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

  window.location.href = "/google-auth?username=" + profile.getName() + '&auth_token=' + id_token + '&id=' + profile.getId();
}

function googleSignOut() {
    var auth2 = gapi.auth2.getAuthInstance();
    auth2.signOut().then(function () {
});
}
