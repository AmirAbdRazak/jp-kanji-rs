let express = require("express");
let app = express();

app.get("/", (req, res) => {
    res.sendFile("jp.html", { root: __dirname });
});

app.listen(3000, (err) => {
    if (err) {
        console.error(err);
    }
    console.log("Listening on port 3000");
});
