
const redis = require('redis')
const express = require('express')

const app = express()
const http = require('http').Server(app)
const io = require('socket.io')(http, {path: '/ws/socket.io'})

const moment = require('moment')

const redisOptions = { host: process.env.REDIS_HOST || 'redis' }
const client = redis.createClient(redisOptions)
const subscriberClient = redis.createClient(redisOptions)

client.on("error", function (err) {
    console.log("Error " + err);
})

subscriberClient.subscribe('room_update', (err) => {
    if(err){
        console.log(err)
        return
    }

    console.log('subscribed to room_update')
})

subscriberClient.on('message', updateChannel)

async function updateChannel (channel, message) {
    console.log('room update from redis')

    let msg = JSON.parse(message)
    let room = msg.key
    let key = `game.${room}`

    let messages = await asyncGet(key)
    io.to(room).emit('room-update', messages)
}

app.post('/ws/create-room', createRoomPost)
app.get('/ws/room/:id', showRoom)

io.on('connection', (socket) => {
    let room = socket.handshake.query.room
    let user = socket.handshake.query.user

    // todo - validate that user can join room with token
    console.log(`socket connected to ${room}`)
    socket.join(room)
    socket.on('disconnect', (reason) => {
        console.log(`user [${user}] disconnected from ${room} 
                    on environment ${process.env.APP_NAME}
                    with reason ${reason}`)
    })


    socket.on('new_message', (data) => {
        console.log(data)
        sendText(data, room, user)
    })

    updateChannel(null, JSON.stringify({ key: room }));

    socket.emit('confirmation', process.env.APP_NAME)
})

http.listen(8888, () => console.log('Example app listening on port 8888!'))

function createRoomPost(req, res){
    client.set(req.query.name, req.query.token)
    res.send({ ok: true })
}

async function showRoom(req, res){
    let key = `game.${req.params.id}`
    // todo - use token to validate
    let token = req.query.token;
    let feed = await asyncGet(key) || []

    feed = feed.map((message) => {
        try{
            return JSON.parse(message)
        } catch(e){
            return false
        }
    }).filter((message) => {
        return message
    })

    res.send({ feed });
}

async function sendText(text, roomId, user){

    let key = `game.${roomId}`

    let msg = {
        user: user,
        time: moment(Date.now()).format('MMM Do HH:mm'),
        type: 'message',
        body: text,
        key: roomId
    }

    let serialisedMessage = JSON.stringify(msg)
    await asyncPush(key, serialisedMessage)
    await publishMessage(roomId, msg)

}

async function asyncGet(key) {

    return new Promise((result, reject) => {
        client.lrange(key, 0, 100, (err, reply) =>{
            result(reply)
        })
    })
}

async function asyncPush(key, val){

    console.log(`pushing to ${key} : ${val}`);

    return new Promise((result, reject) => {
        client.lpush(key, val, () => {
            result()
        })
    })
}

async function publishMessage(key, msg){
    console.log('publishing room update...')
    let message = JSON.stringify({key, msg})
    return new Promise((result, reject) => {
        client.publish('room_update', message, (err) => {
            if(err){
                console.log(err)
                return reject(err)
            }

            console.log('publish successful')
            result()
        })
    })
}
