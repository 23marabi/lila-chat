let uname = document.querySelector('#uname').value; // grabbing the username submitted and putting it in the variable uname
let pin = document.querySelector('#pin').value; // grabbing the pin submitted and putting it in the variable pin
const form = document.querySelector('form'); // grabbing an element on the page

form.addEventListener("submit", async function(event) {
    event.preventDefault();
    const formData = new FormData(form);

    uname = formData.get('uname');
    pin = formData.get('pin');

    const response = await fetch(`/api/users/${uname}/${pin}`);
    const loginInfo = await response.json();
    
    if (loginInfo.status === "ok") {
      login()
    } else {
      incorrectLogin()
    }
})

function login() {
    console.log('You have logged in!')
    document.querySelector("#username").innerHTML = `${uname}`
    document.querySelector("#incorrect").innerHTML = ''
}
   
function incorrectLogin() {
    console.log('Incorrect Login!')
    document.querySelector("#incorrect").innerHTML = 'Incorrect Login.'
}
 