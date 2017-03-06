console.log('Pusoy Dos:: in play');

var playerTemplate = `<div class="user" :class="player.next ? 'next' : ''">
                        <span v-if="player.next" class="next-icon">
                            <i class="fa fa-arrow-circle-right"></i>
                        </span>
                        <span v-else-if="player.loggedIn">
                            <i class="fa fa-user-circle-o"></i>
                        </span>
                        <span v-else><i class="fa fa-user"></i></span>
                        <span>&nbsp;</span>
                        <strike 
                            :class="player.loggedIn ? 
                                'logged-in-player' : ''" 
                            v-if="!player.stillIn" class="name">{{player.name}}</strike>
                        <span v-else :class="player.loggedIn ? 'logged-in-player' : ''" 
                                class="name">{{ player.name }} 
                                </span>
                        <small>( {{player.cardCount }} )</small>
                        <span class="icons">
                            <span v-if="player.winner">
                                <i class="win-trophy fa fa-trophy"></i>
                            </span>
                        </span>
                      </div>`;

Vue.component('player', {
    props: ['player'],
    template: playerTemplate
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
            if(!app.myGo){
                return;
            }

            var card = this.card;

            if(card.joker){
                app.jokerModal = true;
            }else{
                app.selectedCards.push(card);
            }
            app.myCards = app.myCards.filter(function(c){ return c !== card; });
        }
    }
});

Vue.component('status', {
    props: ['players'],
    template:'<div class="text-center"><div v-if="hasWon">You Won!</div><span v-if="userTurn">your turn</span><span v-else>Waiting for <strong>{{nextUser}}</strong></span>',
    computed: {
        hasWon: function(){
            var hasWon = false;
            this.players.forEach(function(player){
                if(player.loggedIn && player.winner){
                    hasWon = true;
                }
            });
            return hasWon;
        },
        userTurn: function(){
            var userTurn = false;
            this.players.forEach(function(player){
                if(player.loggedIn && player.next){
                    userTurn = true;
                }
            });

            return userTurn;
        },
        nextUser: function(){
            var nextUser = 'player';
            this.players.forEach(function(player){
               if(player.next){
                    nextUser = player.name;
                } 
            });

            return nextUser;
        }
    }
});

Vue.component('submit-move', {
    props:['selectedCards'],
    template: `<div class="action">
                <span v-if="submitted">
                    <i class="fa fa-spin fa-circle-o-notch"></i>
                </span>
                <button v-else-if="selectedCards.length > 0" 
                    v-on:click="submit"
                    class="pure-button action-btn play-btn">
                        play card<span v-if="selectedCards.length > 1">s</span>
                </button>
                <input v-else 
                        v-on:click="submit" class="pure-button action-btn pass-btn" type="submit" value="pass"/>
            </div>`,
    data:function(){
        return {
            submitted: false
        };
    },
    computed: {
        haveMove: function(){
            return this.selectedCards.length > 0;
        }
    },
    methods: {
        submit: function(){
            this.submitted = true;

            try{
                ga('send', 'event', 'Game', 'take-move');
            } catch (e) { }

            var o = this;
            post('/api/v1/submit-move/' + pd.gameId, app.selectedCards,
                function(result){
                    o.submitted = false; 
                    if(result.success){
                        app.selectedCards = [];
                        reloadData();
                        swal({
                            type:'success', 
                            title:'nice move!', 
                            timer: 1500, 
                            showConfirmButton:false
                        });
                    } else {
                        console.log(result);
                        swal({
                            type:'error', 
                            title:'that move didn\'t work!',
                            showConfirmButton: false,
                            timer:1500
                        });
                    }
                }); 


        }
    }
});

var app = new Vue({
    el: "#inplay",
    data: {
        myGo: false,
        playerList: [],
        lastMove:[],
        myCards:[],
        selectedCards:[],
        reversed: false,
        jokerModal: false,
        jokerRank: null,
        jokerSuit: null 
    },
    methods:{
        setJoker: function(){
            app.selectedCards.push({
                rank:app.jokerRank,
                suit:app.jokerSuit.className,
                joker: true,
                reversed: app.reversed,
                suitDisplay:app.jokerSuit.display
            });
            app.jokerModal = false;
        }
    }
}); 

var updatePoll = 0;

function setup(){
    document.querySelector('.inplay').classList.remove('not-loaded');
    reloadData();
}

function reloadData(){
    grab('/api/v1/players/' + pd.gameId, 'playerList');
    grab('/api/v1/last-move/' + pd.gameId,  'lastMove');
    grab('/api/v1/my-cards/' + pd.gameId, 'myCards');
}

function grab(url, prop){
    var creds = {credentials: 'same-origin'};
    fetch(url,  creds)
        .then(function(response){
            return response.json();
        }).then(function(result){
            app[prop] = result;
                        
            // hack to globally set when logged in player turn
            app.myGo  = false;
            app.selectedCards = [];
            app.playerList.forEach(function(player){
                if(player.loggedIn && player.next){
                    app.myGo = true;
                }

                // hack to display when order is reversed
                app.reversed = player.reversed;

            });

            clearTimeout(updatePoll);
            if(!app.myGo){
                updatePoll = setTimeout(reloadData, 5000);
            }

        });
}

function post(url, data, callback){
    var body = JSON.stringify(data);
    var myHeaders = new Headers({
        "Content-Type": "application/json",
    });

    var opts = {
        method: "POST",
        headers: myHeaders,
        body: body,
        credentials: 'same-origin'
    };

    console.log('sending..');
    console.log(data);

    fetch(url, opts)
        .then(function(response){
            return response.json();
        })
        .then(callback);
}

setup();
