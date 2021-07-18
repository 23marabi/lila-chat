let uname = document.querySelector('#uname').value;
let pin = document.querySelector('#pin').value; 
let newUname = document.querySelector('#newuname').value;
let newPin = document.querySelector('#newpin').value; 
// let selected = document.querySelector('#selected').value;
// let custom = document.querySelector('#custom').value;
// let pronouns = ''
let responseText;
const form = document.querySelector('form');

form.addEventListener("submit", async function(event) {
    event.preventDefault();
    const formData = new FormData(form);

    uname = formData.get('uname');
    pin = formData.get('pin');
    newUname = formData.get('newuname');
    newPin = formData.get('newpin');
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

    if (userNotFound !== `User ${uname}`) {
      document.querySelector("#incorrect").innerHTML = `user ${uname} was not found`
    } else {
      loginChange()
    }
    } catch {
      document.querySelector("#incorrect").innerHTML = 'An Error has Occurred. Try again later.'
    }
})

async function getUname() {
  let response = await fetch(`/api/users/${uname}`);
  responseText = await response.text();
  return responseText;
}

async function loginChange() {
const rawResponse = await fetch(`/api/users/change/${uname}/${pin}/${newUname}/${newPin}`, {
    method: 'POST',
    headers: {
    },
    body: ""
});
document.querySelector("#incorrect").innerHTML = 'Login Changed!'
window.location.replace("http://127.0.0.1:5500/login.html")
}
