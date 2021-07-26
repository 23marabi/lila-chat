
//DECLARING VARIABLES AND GRABBING VALUES FROM FORM. 

let uname = document.querySelector('#uname').value;
let pin = document.querySelector('#pin').value;
let selected = document.querySelector('#selected').value;
let custom = document.querySelector('#custom').value;
let pronouns = ''
const form = document.querySelector('form');

//SUBMIT FUNCTION & CHECKING IF USERNAME IS TAKEN

form.addEventListener("submit", async function (event) {
  event.preventDefault();
  const formData = new FormData(form);

  uname = formData.get('uname');
  pin = formData.get('pin');
  selected = formData.get('selected');
  custom = formData.get('custom')

  if (custom === '' && selected === 'none') {
    pronouns = ''
  } else if (custom !== '') {
    pronouns = custom
  } else {
    pronouns = selected
  }


  //IF THE API SUCCESSFULLY REGISTERS A USER THEN DO THIS
  try {
    const regRes = await isUnameTaken();

    if (regRes == null) {
      return;
    }

    if (regRes.status === 'ok') {
      document.querySelector("#errormessage").innerHTML = 'Registered!'
      window.location.replace("../html/login.html")
    } else {
      document.querySelector("#errormessage").innerHTML = 'Failed to register. Try again later.'
    }
  } catch (e) {
    console.log(e);
    document.querySelector("#errormessage").innerHTML = 'An Error has Occurred. Try again later. ' + e.toString();
  }

})

//CHECKS IF A USERNAME IS TAKEN
async function isUnameTaken() {

  const response = await fetch(`/api/users/${uname}/`);
  const isTaken = await response.json();

  //YES THIS IS CONFUSING I KNOW.
  if (isTaken.status === "fail") {
    return await register()
  } else {
    document.querySelector('#errormessage').innerHTML = `${uname} is already taken.`
  }
}

//FETCH FUNCTIONS. GETTING USERNAME FROM API & REGISTERING USER ASSIGNED NAME AND PIN. 

async function register() {
  let sendRegisterInfo = { "name": uname, "pin": pin, "pronouns": pronouns }
  const response = await fetch('../api/register/', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(sendRegisterInfo),
  });
  return await response.json()
}

// function errorMessage() {
//   document.querySelector("#errormessage").innerHTML = 'An error has occurrred. Please try again later.'
// }
