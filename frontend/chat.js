// VARIABLES
let messageCount = 0;
let username = localStorage.getItem('username');
const form = document.querySelector('form');

// SEND A MESSAGE

// GRABS MESSAGE FROM FORM & SENDS
form.addEventListener("submit", async function (event) {
    event.preventDefault();
    const formData = new FormData(form);

    formMessage = formData.get('message').toString();

    //KINDA UNNECESSARY
    //CHECKS TO SEE IF THE PERSON IS LOGGED IN IN ORDER TO SEND A MESSAGE.
    const response = await fetch(`api/token/${username}/`);
    const matches = await response.json();

    //YES THIS IS CONFUSING I KNOW.
    if (matches.status === "ok") {
        sendMessage()
    } else {
        const mismatch = 'Username and token mismatch. Try logging in again.'
        printText(mismatch.bold())
        logout()
        localStorage.removeItem('username')
        form.reset()
    }

})

//SEND MESSAGE FETCH FUNCTION

async function sendMessage() {
    sendMessageInfo = { "name": username, "body": formMessage }
    fetch('/api/message/send', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(sendMessageInfo),
    })
    form.reset()
}


// RECIEVE MESSAGES

let messageUpdate = window.setInterval(fetchMessages, 1000);

async function fetchMessages() {
    const response = await fetch('/api/message/messages.json');
    const recievedMessages = await response.json();
    document.getElementById("chatbox").innerHTML = ""

    for (const message of recievedMessages) {
        printText(message.user.bold().toString() + ": " + message.body.toString());
    }


    if (recievedMessages.length != messageCount) {
        let scroll = document.getElementById("chatbox");
        scroll.scrollTop = scroll.scrollHeight;
    }

    messageCount = recievedMessages.length;
}


// FUNCTION TO PRINT MESSAGES IN THE CHAT BOX

function printText(text) {
    let p = document.createElement("p");
    const div = document.getElementById("chatbox");
    div.appendChild(p)
    p.innerHTML = text
}


//LOGGED IN STUFF
//TODO ADD CHECK TO SEE IF USERNAME AND TOKEN MATCHES
function loggedIn() {
    if (username === '' || username === null) {
        document.querySelector("#loggeduser").innerHTML = 'You are not logged in'
    } else {
        document.querySelector("#loggeduser").innerHTML = `You are logged in as "${username}"`
    }
}

loggedIn()


//REVIECE USERS PRONOUNS

async function getPronouns() {
const response = await fetch(`api/users/${username}/`);
const data = await response.json();
pronouns = data.pronouns
return pronouns;
}