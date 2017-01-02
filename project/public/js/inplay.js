console.log('Pusoy Dos:: in play');

Vue.component('player', {
    props: ['player'],
    template: '<li><span :class="player.loggedIn ? \'logged-in-player\' : \'\'" class="name">{{ player.name }}</span><span v-if="player.next">*</span></li>'
});

var app = new Vue({
    el: "#inplay",
    data: {
        playerList: [
/*            { name: "Ben Brunton", next: true, loggedIn: true },
            { name: "Testy McTestface", next: false, loggedIn:false } */
        ]
    }
}); 


fetch('/api/v1/players/' + pd.gameId, {credentials: 'same-origin'}).then(function(result){
    return result.json();
}).then(function(players){
    app.playerList = players;
});


