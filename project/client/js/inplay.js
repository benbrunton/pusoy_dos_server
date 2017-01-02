console.log('Pusoy Dos:: in play');

Vue.component('player', {
    props: ['player'],
    template: '<li><span :class="player.loggedIn ? \'logged-in-player\' : \'\'" class="name">{{ player.name }}</span><span v-if="player.next">*</span></li>'
});

Vue.component('move-card', {
    props: ['card'],
    template: '<span class="card" :class="card.suit.toLowerCase() + \' \' + card.rank.toLowerCase()">{{card.rank + card.suitDisplay}}</span>'
});

var app = new Vue({
    el: "#inplay",
    data: {
        playerList: [],
        lastMove:[]
    }
}); 


grab('/api/v1/players/' + pd.gameId, 'playerList');
grab('/api/v1/last-move/' + pd.gameId,  'lastMove');

function grab(url, prop){
    var creds = {credentials: 'same-origin'};
    fetch(url,  creds)
        .then(function(response){
            return response.json();
        }).then(function(result){
            app[prop] = result;
        });
}
