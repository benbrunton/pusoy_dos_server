console.log('Pusoy Dos:: in play');

Vue.component('player', {
    props: ['player'],
    template: '<li><span :class="player.loggedIn ? \'logged-in-player\' : \'\'" class="name">{{ player.name }}</span><span v-if="player.next">*</span></li>'
});

Vue.component('table-card', {
    props: ['card'],
    template: '<span class="card" :class="card.suit.toLowerCase() + \' \' + card.rank.toLowerCase()">{{card.rank + card.suitDisplay}}</span>'
});


Vue.component('move-card', {
    props: ['card'],
    template: '<span v-on:click="deselect" class="card" :class="card.suit.toLowerCase() + \' \' + card.rank.toLowerCase()">{{card.rank + card.suitDisplay}}</span>',
    methods: {
        deselect: function(){
            var card = this.card;
            app.myCards.push(card);
            app.selectedCards = app.selectedCards.filter(function(c){ return c !== card; });
        }
    }
});

Vue.component('player-card', {
    props: ['card'],
    template: '<span class="card-container" v-on:click="select" :class="card.suit.toLowerCase() + \' \' + card.rank.toLowerCase()"><span class="card"><p>{{card.rank}}</p><p>{{card.suitDisplay}}</p></span></span>',
    methods: {
        select: function(){
            var card = this.card;
            app.selectedCards.push(card);
            app.myCards = app.myCards.filter(function(c){ return c !== card; });
        }
    }
});

var app = new Vue({
    el: "#inplay",
    data: {
        playerList: [],
        lastMove:[],
        myCards:[],
        selectedCards:[]
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

