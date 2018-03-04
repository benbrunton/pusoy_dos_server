const express = require('express')
const app = express()

const AWS = require('aws-sdk');
const bodyParser = require('body-parser'); 


app.use(bodyParser.json())

app.post('/relay/:id', async (req, res) => {

    console.log(req.body);

    var sns = new AWS.SNS({region:'eu-west-1'});
    var message = { id: req.body.id, result: req.body.players };

    var params = {
      Message: JSON.stringify(message),
      TopicArn: process.env.TOPIC_ARN
    };
    sns.publish(params, function(err, data) {
        if (err){
            console.log(err, err.stack); // an error occurred
            res.send({success: false});
        } else {
            console.log(data);           // successful response
            res.send({success: true})
        }
    });
})

app.listen(3080, () => console.log('Example app listening on port 3080!'))
