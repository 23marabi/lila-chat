
//SETTING VARIABLES. GRABBING ELEMENT VALUES FROM FORM. 

let uname = document.querySelector('#uname').value;
let pin = document.querySelector('#pin').value;
let newUname = document.querySelector('#newuname').value;
let newPin = document.querySelector('#newpin').value;
const form = document.querySelector('form');
let selected = document.querySelector('#selected').value;
let custom = document.querySelector('#custom').value;
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


  //SETS NEWPRONOUNS DEPENDING ON WHAT THE USER SELECTED/TYPED
  if (custom === '' && selected === 'none') {
    newPronouns = ''
  } else if (custom !== '') {
    newPronouns = custom
  } else {
    newPronouns = selected
  }

  unameCheck()

  //CHECKS IF THE USER IS CHANGING MORE THAN ONE TEXT FIELD AT A TIME
  if (newUname !== '' && newPin !== '') {
    document.querySelector("#errormessage").innerHTML = 'You can only change one at a time!'
    return;
  } else if (newUname !== '' && newPronouns !== '') {
    document.querySelector("#errormessage").innerHTML = 'You can only change one at a time!'
    return;
  } else if (newPin !== '' && newPronouns !== '') {
    document.querySelector("#errormessage").innerHTML = 'You can only change one at a time!'
    return;
  } else if (newUname !== '' && newPin !== '' && newPronouns !== '') {
    document.querySelector("#errormessage").innerHTML = 'You can only change one at a time!'
    return;
  } else {
  }


  // ASSIGNS VARIABLES TO BE SENT TO API
  if (newUname === '' && newPin === '' && newPronouns !== '') {
    newEvent = newPronouns
    updateEvent = 'Pronouns'
  } else if (newUname === '' && newPronouns === '' && newPin !== '') {
    newEvent = newPin
    updateEvent = 'Pin'
  } else if (newPin === '' && newPronouns === '' && newUname !== '') {
    newEvent = newUname
    updateEvent = 'Name'
  } else if (newPin === '' && newUname === '' && newPronouns === '') {
    document.querySelector("#errormessage").innerHTML = 'Please enter a new name, pin, or pronouns!'
    return;
  } else {
  }


  //IF THE API SUCCESSFULLY UPDATES INFO FOR A USER THEN DO THIS
  try {
    const updateResponse = await loginStatus();

    if (updateResponse == null) {
      return;
    }

    if (updateResponse.status === 'ok') {
      document.querySelector("#errormessage").innerHTML = 'Login Changed!'
      window.location.replace("../html/login.html")
    } else {
      document.querySelector("#errormessage").innerHTML = 'Failed to update info. Try again later.'
    }
  } catch (e) {
    console.log(e);
    document.querySelector("#errormessage").innerHTML = 'An Error has Occurred. Try again later. ' + e.toString();
  }

})

  //CHECKS IF A USERNAME IS TAKEN
  async function unameCheck(){
  if (newUname !== '') {
    const response = await fetch(`api/users/${newUname}/`);
    const isTaken = await response.json();

    if (isTaken.status === "ok") {
      document.querySelector('#errormessage').innerHTML = `${newUname} is already taken.`
      return;
    } else {
    }
  }
}

//CHECKS IF THE LOGIN IS A SUCCESS
async function loginStatus() {
  const loginInfo = await checkLoginInfo();

  if (loginInfo.status === 'ok') {
    return await updateInfo()
  } else {
    incorrectLogin()
  }
}

//TODO ADD CHECKING THE TOKEN WITH LOGIN IN IF STATEMENT
//CHECKING IF THE USER CAN LOGIN WITH GIVEN CURRENT USERNAME AND PIN
// LOGIN FETCH

async function checkLoginInfo() {
  let sendLoginInfo = { "name": uname, "pin": pin }
  const res = await fetch('/api/login/', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(sendLoginInfo),
  });
  return await res.json();
}

//FETCH FUNCTION TO UPDATE USER INFO

async function updateInfo() {
  let sendUpdateInfo = { "name": uname, "pin": pin, "changed_event": updateEvent, "new_event": newEvent }
  const response = await fetch('/api/change', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(sendUpdateInfo),
  });
  return await response.json()
}

function incorrectLogin() {
  document.querySelector("#errormessage").innerHTML = 'Username and pin combination do not match! Or user not found.'
}

function errorMessage() {
  document.querySelector("#errormessage").innerHTML = 'An Error has Occurred. Try again later.'
}
