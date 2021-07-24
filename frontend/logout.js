//VARIBLES

// //IF NOT LOGGED IN DON'T SHOW LOG IN BUTTON
//  if (username === '') {
//     document.getElementById("logoutlink").style.display = "none";
//  }

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
  localStorage.removeItem('username')
  username = null;
  loggedIn()
}
