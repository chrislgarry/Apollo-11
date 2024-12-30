const COMANCHE055 = "Comanche055";
const LUMINARY099 = "Luminary099";
const COMANCHE055_PAGES = 1751;
const LUMINARY099_PAGES = 1743;

function changeDir() {
  showPage();
}

function showFirst() {
  changePage(1);
}

function showLast() {
  const directory = document.form.directory.value;
  if (directory === COMANCHE055) changePage(COMANCHE055_PAGES);
  else if (directory === LUMINARY099) changePage(LUMINARY099_PAGES);
}

function showPrevious() {
  const newpage = parseInt(document.form.pagenum.value) - 1;
  if (newpage >= 1) {
    changePage(newpage);
  }
}

function showNext() {
  const newpage = parseInt(document.form.pagenum.value) + 1;
  const directory = document.form.directory.value;
  if (
    (directory === COMANCHE055 && newpage <= COMANCHE055_PAGES) ||
    (directory === LUMINARY099 && newpage <= LUMINARY099_PAGES)
  ) {
    changePage(newpage);
  }
}

function changePage(page) {
  document.form.pagenum.value = parseInt(page);
  showPage();
}

function showPage() {
  let page = parseInt(document.form.pagenum.value);
  const directory = document.form.directory.value;

  if (page < 1) {
    document.form.pagenum.value = 1;
    page = 1;
  } else if (directory === COMANCHE055 && page > COMANCHE055_PAGES) {
    document.form.pagenum.value = COMANCHE055_PAGES;
    page = COMANCHE055_PAGES;
  } else if (directory === LUMINARY099 && page > LUMINARY099_PAGES) {
    document.form.pagenum.value = LUMINARY099_PAGES;
    page = LUMINARY099_PAGES;
  }

  const formattedPage = page.toString().padStart(4, "0");
  const imageURL = `https://www.ibiblio.org/apollo/ScansForConversion/${directory}/${formattedPage}.jpg`;
  document.image.src = imageURL;
  document.body.style.cursor = "progress";
}
