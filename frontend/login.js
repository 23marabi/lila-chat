let uname = document.querySelector('#uname').value; // grabbing the username submitted and putting it in the variable uname
let pin = document.querySelector('#pin').value; // grabbing the pin submitted and putting it in the variable pin
const form = document.querySelector('form'); // grabbing an element on the page

form.addEventListener("submit", async function(event) {
    event.preventDefault();
    const formData = new FormData(form);

    uname = formData.get('uname');
    pin = formData.get('pin');

    try {
    const loginInfo = await loginFetch();

    if (loginInfo === "pin matches") {
        login()
    } else if (loginInfo === "Incorrect pin" || loginInfo === `User ${uname} does not exist.`) {
        incorrectLogin()
    }
    } catch {
        document.querySelector("#incorrect").innerHTML = 'An Error has Occurred. Try again later.'
    }
})

async function loginFetch() {
    const rawResponse = await fetch(`/api/users/${uname}/${pin}`, {
        // credentials: "include",
        method: 'GET',
        headers: {
            'Accept': 'text/plain'
          },
});
const content = await rawResponse.text();
return content
}

function login() {
    console.log('You have logged in!')
    document.querySelector("#username").innerHTML = `${uname}`
    document.querySelector("#incorrect").innerHTML = ''
}
   
function incorrectLogin() {
    console.log('Incorrect Login!')
    document.querySelector("#incorrect").innerHTML = 'Incorrect Login.'
}
 