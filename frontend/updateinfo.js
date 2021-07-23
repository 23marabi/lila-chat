
//SETTING VARIABLES. GRABBING ELEMENT VALUES FROM FORM. 

let uname = document.querySelector('#uname').value;
let pin = document.querySelector('#pin').value;
let newUname = document.querySelector('#newuname').value;
let newPin = document.querySelector('#newpin').value;
const form = document.querySelector('form');
let selected = document.querySelector('#selected').value;
let custom = document.querySelector('#custom').value;
let responseText;
let updateEvent = ''
let newEvent = ''
let newPronouns = ''

//FORM SUMBIT FUNCTION & GET THE USERS USERNAME AND SEE IF IT IS CORRECT.


form.addEventListener("submit", async function (event) {
  event.preventDefault();
  const formData = new FormData(form);

  uname = formData.get('uname');
  pin = formData.get('pin');
  newUname = formData.get('newuname');
  newPin = formData.get('newpin');
  selected = formData.get('selected');
  custom = formData.get('custom')

  if (custom === '' && selected === 'none') {
    newPronouns = ''
  } else if (custom !== '') {
    newPronouns = custom
  } else {
    newPronouns = selected
  }

//CHECKS IF THE USER IS CHANGING MORE THAN ONE TEXT FIELD AT A TIME
  let onlyChangeOne = document.querySelector("#errormessage").innerHTML = 'You can only change one at a time!'
  if (newUname !== '' && newPin !== '') {
    onlyChangeOne
  } else if (newUname !== '' && newPronouns !== '') {
    onlyChangeOne
  } else if (newPin !== '' && newPronouns !== '') {
    onlyChangeOne
  } else {
    checkLoginInfo()
  }


// ASSIGNS VARIABLES TO BE SENT TO API
  if (newUname === '' && newPin === '' && newPronouns !== '') {
    newEvent = newPronouns
    updateEvent = 'pronouns'
  } else if (newUname === '' && newPronouns === '' && newPin !== '') {
    newEvent = newPin
    updateEvent = 'pin'
  } else if (newPin === '' && newPronouns === '' && newUname !== '') {
    newEvent = newUname
    updateEvent = 'name'
  } else if (newPin === '' && newUname === '' && newPronouns === '') {
    document.querySelector("#errormessage").innerHTML = 'Please enter a new name, pin, or pronouns!'
  } else {
    checkLoginInfo()
  }

  //CHECKS IF USERNAME IS TAKEN
    const isTaken = await getUname();

    if (isTaken.status === 'ok') {
      document.querySelector("#errormessage").innerHTML = `username ${newUname} is already taken! `
    }
})


// FETCH FUNTIONS. FETCHING USERNAME TO SEE IF ITS TAKEN.

async function getUname() {
  let response = await fetch(`/api/users/${newUname}`);
  responseJson = await response.json();
  return responseJson;
}


//FETCH FUNCTION TO UPDATE USER INFO

  //TODO ADD CHECKING THE TOKEN with LOGIN IN IF STATEMENT
//CHECKING IF THE USER CAN LOGIN WITH GIVEN CURRENT USERNAME AND PIN
async function checkLoginInfo() {
  const response = await fetch(`/api/users/${uname}/${pin}`);
  const loginInfo = await response.json();

  if (loginInfo.status === "ok") {
    updateInfo()
  } else {
    incorrectLogin()
  }
}

async function updateInfo() {
  let sendUpdateInfo = { "name": uname, "pin": pin, "changed_event": updateEvent, "new_event": newEvent }
  fetch('/api/users/change', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(sendUpdateInfo),
  });
  //document.querySelector("#errormessage").innerHTML = 'Login Changed!'
  //window.location.replace("/login.html")
}

function incorrectLogin() {
  document.querySelector("#errormessage").innerHTML = 'Username and pin combination do not match! Or user not found.'
}

function errorMessage() {
  document.querySelector("#errormessage").innerHTML = 'An Error has Occurred. Try again later.'
}
