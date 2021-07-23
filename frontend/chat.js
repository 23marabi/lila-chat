// VARIABLES
let date = '2021-07-22'
let messageCount = 0;
let username = localStorage.getItem('username');
const form = document.querySelector('form');

// SEND A MESSAGE

// GRABS MESSAGE FROM FORM & SENDS
form.addEventListener("submit", async function (event) {
    event.preventDefault();
    const formData = new FormData(form);

    formMessage = formData.get('message');

    sendMessage()

})

//SEND MESSAGE FETCH FUNCTION

async function sendMessage() {
    sendMessageInfo = { "name": username, "body": formMessage, "date": date }
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

let messageUpdate = window.setInterval(fetchMessages, 500);

async function fetchMessages() {
    const response = await fetch('/api/message/messages.json');
    const recievedMessages = await response.json();
    document.getElementById("innerchatbox").innerHTML = ""

    for (const message of recievedMessages) {
      printText(message.user.bold() + ": " + message.body);
    }

        
    if (recievedMessages.length != messageCount) {
        let scroll = document.getElementById("innerchatbox");
        scroll.scrollTop = scroll.scrollHeight;
        }

    messageCount = recievedMessages.length;
}


// FUNCTION TO PRINT MESSAGES IN THE CHAT BOX

function printText(text) {
    let p = document.createElement("p");
    const div = document.getElementById("innerchatbox");
    div.appendChild(p)
    p.innerHTML = text
}


//LOGGED IN STUFF
//TODO ADD CHECK TO SEE IF USERNAME AND TOKEN MATCHES
if (username === null) {
    document.querySelector("#loggeduser").innerHTML = 'You are not logged in'
    username = ''
} else {
    document.querySelector("#loggeduser").innerHTML = `You are logged in as ${username}`
}