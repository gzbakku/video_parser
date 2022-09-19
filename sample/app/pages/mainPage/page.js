//controllers
const log = false;
const type = 'page';

//ids
const pageId = "page-main";
const pageName = 'mainPage';

//init page
const init = () => {
  engine.make.init.page(pageId,"page");  //init page
  build();                               //start build
}

//build page
function build(){

  const main = engine.make.div({
    parent:pageId,
    class:''
  });

    engine.make.div({
      parent:main,
      text:'record',
      class:'main-page-record-button',
      function:async ()=>{
        make_links(main,await record(main));
      }
    });

}

const no_of_chunks = 3;
const secs_per_chunk = 3;
let mime,audioBitsPerSecond,videoBitsPerSecond;

async function make_links(parent,chunks){
  const JSZip = require("jszip");
  let zip = new JSZip();
  const main = engine.make.div({
    parent:parent,
    class:'main-page-record-links'
  });
  let loop = 1;
  for(let item of chunks){
    zip.file("file_" + loop + ".webm",item);
    loop++;
  }
  zip.file("mime.json",JSON.stringify({
    mime:mime,
    audioBitsPerSecond:audioBitsPerSecond,
    videoBitsPerSecond:videoBitsPerSecond
  },null,1));
  const make = await zip.generateAsync({type:"blob"})
  .then(function(content) {
      return content;
  })
  .catch(()=>{
    console.log("failed to zip");
    return false;
  });
  if(!make){return false;}
  let url = window.URL.createObjectURL(make);
  let link = engine.make.element({
    parent:main,
    tag:'a',
    text:"zip link",
    href:url,
    class:'main-page-record-links-link'
  });
}//zip function ends here

async function record(main){
  return new Promise(async (resolve,reject)=>{
    const stream = await navigator.mediaDevices.getUserMedia({
      video: true,
      audio: true
    }).then(stream => {
      return stream;
    }).catch(()=>{
      return false;
    });
    if(!stream){
      reject();
      console.log("failed strat stream");
      return false;
    }
    const video = engine.make.element({
      parent:main,
      tag:'video',
      srcObject:stream,
      class:'main-page-record-video',
      autoplay:true
    });
    let recorder = new MediaRecorder(stream,{
      mimeType:'video/webm; codecs=vp8, opus'
    });
    let data = [];
    let loop = 0;
    recorder.ondataavailable = (event)=>{
      if(loop > no_of_chunks){return;}
      if(loop === no_of_chunks){recorder.stop();return;}
      data.push(event.data);
      loop++;
    }
    if(!audioBitsPerSecond){
      audioBitsPerSecond = recorder.audioBitsPerSecond;
    }
    if(!mime){
      mime = recorder.mimeType;
    }
    if(!videoBitsPerSecond){
      videoBitsPerSecond = recorder.videoBitsPerSecond;
    }
    recorder.start(secs_per_chunk * 1000);
    recorder.onstop = ()=>{
      stream.getTracks().forEach(track => track.stop());
      engine.view.remove(video);
      resolve(data);
    }
  });

}

//do not change current exports you are free to add your own though.
let pageControllers = {
  init:init,
  ref:pageId,
  type:type,
  name:pageName,
  contModules:{},
  contList:{}
};
module.exports = pageControllers;
window.pageModules[pageName] = pageControllers;
