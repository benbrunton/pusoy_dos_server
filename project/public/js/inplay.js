console.log('Pusoy Dos:: in play');

Vue.component('player', {
    props: ['player'],
    template: '<li><span :class="player.loggedIn ? \'logged-in-player\' : \'\'" class="name">{{ player.name }}</span><span v-if="player.next">*</span></li>'
});

Vue.component('move-card', {
    props: ['card'],
    template: '<span class="card" :class="card.suit.toLowerCase() + \' \' + card.rank.toLowerCase()">{{card.rank + card.suitDisplay}}</span>'
});

Vue.component('player-card', {
    props: {
        card: {
            type: Object
        },
        selected: {
            type: Boolean,
            default: false
        }
    },
    template: '<span class="card-container" v-on:click="select" :class="card.suit.toLowerCase() + \' \' + card.rank.toLowerCase() + \' \' + (selected ? \'picked\' : \'\')"><span class="card"><p>{{card.rank}}</p><p>{{card.suitDisplay}}</p></span></span>',
    methods: {
        select: function(){
            deselectAllCards();
            this.selected = !this.selected; 
        }
    }
});

var app = new Vue({
    el: "#inplay",
    data: {
        playerList: [],
        lastMove:[],
        myCards:[]
    }
}); 


grab('/api/v1/players/' + pd.gameId, 'playerList');
grab('/api/v1/last-move/' + pd.gameId,  'lastMove');
grab('/api/v1/my-cards/' + pd.gameId, 'myCards');

function grab(url, prop){
    var creds = {credentials: 'same-origin'};
    fetch(url,  creds)
        .then(function(response){
            return response.json();
        }).then(function(result){
            app[prop] = result;
        });
}

function deselectAllCards(){
    app.myCards.forEach(function(card){
        card.selected = false;
    });
}
