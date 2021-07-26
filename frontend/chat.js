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
    return formMessage;
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
    modCommand()
}


// RECIEVE MESSAGES

let messageUpdate = window.setInterval(fetchMessages, 1000);

async function fetchMessages() {
    const response = await fetch('/api/message/messages.json');
    const recievedMessages = await response.json();
    document.getElementById("chatbox").innerHTML = ""

    for (const message of recievedMessages) {

        let leftBracket = '('
        let rightBracket = ')'
        let space = ' '

        if (message.pronouns === '' || message.pronouns === 'none' || message.pronouns === null) {
            leftBracket = ''
            rightBracket = ''
            space = ''
        }
        printText(message.user.bold().toString() + space + leftBracket.small() + message.pronouns.small().toString() + rightBracket.small() + ": " + message.body.toString());
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

//MODERATION

async function modCommand() {
let action = ''
let target = ''

if (formMessage.startsWith('/ban')) {
    action = "Ban"
    target = formMessage.replace('/ban ', '')
    sendCommand()
} else if (formMessage.startsWith('/kick')) {
    action = "Kick"
    target = formMessage.replace('/kick ', '')
    sendCommand()
} else if (formMessage.startsWith('/promote')) {
    action = "Promote"
    target = formMessage.replace('/promote ', '')
    sendCommand()
} else if (formMessage.startsWith('/demote')) {
    action = "Demote"
    target = formMessage.replace('/demote ', '')
    sendCommand()
} else {
    return;
}

async function sendCommand() {
    let sendCommand = { "name": username, "action": action, "target": target }
    const response = await fetch('/api/mod/', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(sendCommand),
    });
    if (response.status === 'ok') {
        return;
    } else {
        printText('Error Issuing Command. Are you an Admin or Mod?')
    }
  }
}



