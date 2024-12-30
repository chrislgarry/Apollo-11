const darkBG = "#141414";
const darkLink = "#ff1c5c";
const darkText = "Dark Mode";
const darkColor = "black";

const lightBG = "white";
const lightLink = "#9e1bd6";
const lightText = "Light Mode";
const lightColor = "white";

darkMode = false;

function lightDark(s) {
  let root = document.documentElement;
  document.querySelector("img").classList.toggle("invert");
  document.querySelector("button").classList.toggle("hover");
  document.querySelector("form select").classList.toggle("hover");
  document
    .querySelectorAll("form input")
    .forEach((e) => e.classList.toggle("hover"));
  document.querySelector("form input[type='text']").classList.toggle("white");
  document.querySelector("form span").classList.toggle("muted-color");
  if (!darkMode) {
    darkMode = true;
    s.innerText = lightText;
    root.style.setProperty("--link-color", darkLink);
    root.style.setProperty("--bg-color", darkBG);
  } else {
    darkMode = false;
    s.innerText = darkText;
    root.style.setProperty("--link-color", lightLink);
    root.style.setProperty("--bg-color", lightBG);
  }

  console.log(darkMode);
}
