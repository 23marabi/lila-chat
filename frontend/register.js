let uname = document.querySelector('#uname').value; // grabbing the username submitted and putting it in the variable uname
let pin = document.querySelector('#pin').value; // grabbing the pin submitted and putting it in the variable pin
let selected = document.querySelector('#selected').value;
let custom = document.querySelector('#custom').value;
let pronouns = ''
let responseText;
const form = document.querySelector('form'); // grabbing an element on the page

form.addEventListener("submit", async function(event) {
    event.preventDefault();
    const formData = new FormData(form);

    uname = formData.get('uname');
    pin = formData.get('pin');
    selected = formData.get('selected');
    custom = formData.get('custom')

    if (custom !== '') {
      pronouns = custom
    } else {
      pronouns = selected
    }
    
    try {
    const isTaken = await getUname();

    if (isTaken === `User ${uname}`) {
      console.log("This username is taken.")
      document.querySelector("#taken").innerHTML = `${uname} is already taken.`
    } else {
      register()
    }
    } catch {
      document.querySelector("#taken").innerHTML = 'An Error has Occurred. Try again later.'
    }
})

async function getUname() {
  let response = await fetch(`$/api/users/${uname}`);
  responseText = await response.text();
  return responseText;
}

async function register() {
const rawResponse = await fetch(`/api/register/${uname.toString().toLowerCase()}/${pin.toString()}/${pronouns.toString().toLowerCase().replace("/", ".")}`, {
    method: 'POST',
    headers: {
      'Accept': 'text/plain'
    },
    body: ""
});
document.querySelector("#taken").innerHTML = 'Registered!'
window.location.replace("http://127.0.0.1:5500/login.html")
}

