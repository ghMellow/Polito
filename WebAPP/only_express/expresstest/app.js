const express = require('express');

const app = express();

app.get('/', (req, res) => {
    res.send("Hellow World");
});

app.listen(4000, () => {
    console.log("listening to port 4000");
})