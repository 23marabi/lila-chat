
//SETTING VARIABLES. GRABBING ELEMENT VALUES FROM FORM. 

let uname = document.querySelector('#uname').value;
let pin = document.querySelector('#pin').value;
let newUname = document.querySelector('#newuname').value;
let newPin = document.querySelector('#newpin').value;
const form = document.querySelector('form');
let responseText;


//TODO
// let selected = document.querySelector('#selected').value;
// let custom = document.querySelector('#custom').value;
// let pronouns = ''

//FORM SUMBIT FUNCTION & GET THE USERS USERNAME AND SEE IF IT IS CORRECT.


form.addEventListener("submit", async function (event) {
  event.preventDefault();
  const formData = new FormData(form);

  uname = formData.get('uname');
  pin = formData.get('pin');
  newUname = formData.get('newuname');
  newPin = formData.get('newpin');

  //TODO
  // selected = formData.get('selected');
  // custom = formData.get('custom')

  // if (custom !== '') {
  //   pronouns = custom
  // } else {
  //   pronouns = selected
  // }

  if (newUname === '') {
    newUname = uname
  }
  if (newPin === '') {
    newPin = pin
  }

  try {
    const userNotFound = await getUname();

    if (userNotFound.status === 'fail') {
      document.querySelector("#errormessage").innerHTML = `user ${uname} was not found`
    } else {
      loginChange()
    }
  } catch {
    document.querySelector("#errormessage").innerHTML = 'An Error has Occurred. Try again later.'
  }
})


//FETCH FUNTIONS. FETCHING USERNAME FROM API AND SUBMITTING NEW PIN AND USERNAME TO API.

async function getUname() {
  let response = await fetch(`/api/users/${uname}`);
  responseJson = await response.json();
  return responseJson;
}

async function loginChange() {
  const rawResponse = await fetch(`/api/users/change/${uname}/${pin}/${newUname}/${newPin}`, {
    method: 'POST',
    headers: {
    },
    body: ""
  });
  document.querySelector("#errormessage").innerHTML = 'Login Changed!'
  window.location.replace("/login.html")
}
