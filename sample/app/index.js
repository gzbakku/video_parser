
//import all the pages here which you want to be in the app and use engine.get.pageModule api to get the page
const mainPage = require('./pages/mainPage/page');

//declare the first page module here
const startPage = mainPage;

/*set the base url to the native vegana cdn,
or if hosting on non native platform please
set the baseurl to where the files for the project are held*/
const baseHref = null;

//------------------------------------------------------------------------------
//dont fuck with anything below
engine.router.set.baseHref(baseHref);
if(engine.router.active.page == null){
  startPage.init();
}
