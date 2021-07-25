// // //VARIBLES
// myStorage = window.localStorage;
// allCookies = document.cookie;

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

// // IF THERE IS A TOKEN BUT NO USERNAME LOGOUT
// if (allCookies !== '' && myStorage.length === 0) {
//   logout()
// }

// // IF THERE IS NO COOKIE BUT A USERNAME GET RID OF USERNAME LOCALLY.
// if (allCookies === '' && myStorage.length !== 0) {
//   localStorage.removeItem('username')
//   document.querySelector("#loggeduser").innerHTML = 'You are not logged in'
// }