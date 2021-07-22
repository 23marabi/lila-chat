
//SETTING VARIABLES. GRABBING CONTENTS FROM FORM.

let uname = document.querySelector('#uname').value;
let pin = document.querySelector('#pin').value;
const form = document.querySelector('form');

// SUBMIT FORM FUNCTION. AND FETCH USERNAME AND PIN FROM API. 

form.addEventListener("submit", async function (event) {
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


// FUNCTIONS FOR WHETHER THE LOGIN WAS A SUCCESS OR FAILURE

function login() {
  console.log('You have logged in!')
  document.querySelector("#username").innerHTML = `${uname}`
  document.querySelector("#errormessage").innerHTML = ''
  localStorage.setItem("username", `${uname}`);
}

function incorrectLogin() {
  console.log('Incorrect Login!')
  document.querySelector("#errormessage").innerHTML = 'Incorrect Login.'
}