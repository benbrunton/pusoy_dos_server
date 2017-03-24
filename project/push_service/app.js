const express = require('express');
const bodyParser = require('body-parser');
const webpush = require('web-push');

const publicKey = 'BKriLwzXwDTwU6erJI69Bo-tH-3G2Ia1hFkN_rA5P6m05ZofcPPR6v1RcXcy7_FZNEgQK1uv91YB6APex3Fduik';
const privateKey = 'yyxbh8LGWWUoWGa8hgR-lhnZMAHtyGsx9AFLmIql5As';

const options = {
  TTL: 86400
};

webpush.setVapidDetails(
  'mailto:notnurbyor@gmail.com',
  publicKey,
  privateKey
);

var app = express();

app.use(function(req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
  next();
});

app.use(bodyParser.urlencoded({ extended: false }));
app.use(bodyParser.json());

app.get('/', function (req, res) {
  res.send(latestSub);
});

app.post('/', function (req, res) {
  const subscription = req.body.subscription;
  const payload = {
    title: req.body.title || 'Pusoy Dos',
    body: req.body.body || 'Missing message!',
    icon: req.body.icon || '/public/img/spade_icon_144.png',
    actions: req.body.actions || [],
    tag: req.body.tag || 0,
    data: req.body.data ||null,
  };

  webpush.sendNotification(subscription, JSON.stringify(payload), options)
    .then((request) => {
      //console.log(request);
      res.status(request.statusCode).send(request.body);
    })
    .catch((error) => {
      res.status(500).send(error.toString());
    });
});

app.listen(8080, function() {
  console.log('listening...');
});
