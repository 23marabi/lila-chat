//VARIBLES
myStorage = window.localStorage;

//LOGOUT FETCH FUNCTION

async function logout() {
  let sendLogoutInfo = { "name": username }
  fetch('/api/logout/', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(sendLogoutInfo),
  });
  document.querySelector("#errormessage").innerHTML = 'Logged out.'
  document.getElementById("logoutbutton").style.display = "none";
  localStorage.removeItem('username')
  username = null;
  loggedIn()
}

//CHECKS TO SEE IF USERNAME MATCHES TOKEN 
let tokenUpdate = window.setInterval(checkToken, 1000);

async function checkToken() {
  const response = await fetch(`api/token/${username}/`);
  const matches = await response.json();

  //YES THIS IS CONFUSING I KNOW.
  if (matches.status === "fail") {
    loggedOut()
  }

  // IF NO USERNAME BUT HAS A TOKEN THEN LOGOUT

  if (matches.status === "ok" && myStorage.length === 0) {
    logout()
  }
}

//AND IF THEY DON'T HAVE A TOKEN CLEARS THE LOCAL STORED USERNAME

function loggedOut() {
  localStorage.removeItem('username')
  document.querySelector("#loggeduser").innerHTML = 'You are not logged in'
}