(()=>{"use strict";var e,t,r,n,o,a={492:(e,t,r)=>{r.a(e,(async e=>{var t=r(395),n=e([t]);t=(n.then?await n:n)[0];const o=document.getElementById("canvas");if(!o)throw new Error('No "canvas" element');const a=document.getElementById("prob-view");if(!a)throw new Error('No "prob-view" html element');const s=28*Math.floor(400/28);o.width=s,o.height=s;const i=o.getContext("2d");if(!i)throw new Error("Unsupported 2d");const c=new t.gV(s,i,a);c.clear(),o.addEventListener("mousedown",(e=>{c.mouse_event(t.k8.Down,e.offsetX,e.offsetY,e.buttons)})),o.addEventListener("mouseup",(e=>{c.mouse_event(t.k8.Up,e.offsetX,e.offsetY,e.buttons)})),o.addEventListener("mousemove",(e=>{c.mouse_event(t.k8.Move,e.offsetX,e.offsetY,e.buttons)})),o.addEventListener("mouseenter",(e=>{c.mouse_event(t.k8.Enter,e.offsetX,e.offsetY,e.buttons)})),o.addEventListener("mouseleave",(e=>{c.mouse_event(t.k8.Leave,e.offsetX,e.offsetY,e.buttons)})),document.addEventListener("keydown",(e=>{" "==e.key&&c.clear()}))}))},395:(e,t,r)=>{r.a(e,(async n=>{r.d(t,{k8:()=>d,gV:()=>_,h4:()=>h,ug:()=>w,nn:()=>b,B4:()=>m,lp:()=>v,Or:()=>y});var o=r(985);e=r.hmd(e);var a=n([o]);o=(a.then?await a:a)[0];let s=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let i=null;function c(e,t){return s.decode((null!==i&&i.buffer===o.memory.buffer||(i=new Uint8Array(o.memory.buffer)),i).subarray(e,e+t))}const u=new Array(32).fill(void 0);u.push(void 0,null,!0,!1);let f=u.length;function p(e){f===u.length&&u.push(u.length+1);const t=f;return f=u[t],u[t]=e,t}function l(e){return u[e]}const d=Object.freeze({Down:0,0:"Down",Enter:1,1:"Enter",Leave:2,2:"Leave",Move:3,3:"Move",Up:4,4:"Up"});class _{static __wrap(e){const t=Object.create(_.prototype);return t.ptr=e,t}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();o.__wbg_app_free(e)}constructor(e,t,r){var n=o.app_new(e,p(t),p(r));return _.__wrap(n)}mouse_event(e,t,r,n){o.app_mouse_event(this.ptr,e,t,r,n)}clear(){o.app_clear(this.ptr)}}function h(e,t){return p(c(e,t))}function w(e){!function(e){const t=l(e);(function(e){e<36||(u[e]=f,f=e)})(e)}(e)}function b(e,t){l(e).fillStyle=l(t)}function m(e,t,r,n,o){l(e).fillRect(t,r,n,o)}function v(e,t,r){l(e).innerText=c(t,r)}function y(e,t){throw new Error(c(e,t))}}))},985:(e,t,r)=>{var n=([n])=>r.v(t,e.id,"3dd13ba27e9afe21ab85",{"./torch_onnx_web_rs_bg.js":{__wbindgen_string_new:n.h4,__wbindgen_object_drop_ref:n.ug,__wbg_setfillStyle_528a6a267c863ae7:n.nn,__wbg_fillRect_10e42dc7a5e8cccd:n.B4,__wbg_setinnerText_4f4ec715a9a131a0:n.lp,__wbindgen_throw:n.Or}});r.a(e,(e=>{var t=e([r(395)]);return t.then?t.then(n):n(t)}),1)}},s={};function i(e){var t=s[e];if(void 0!==t)return t.exports;var r=s[e]={id:e,loaded:!1,exports:{}};return a[e](r,r.exports,i),r.loaded=!0,r.exports}e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},n=e=>!--e.r&&e(),o=(e,t)=>e?e.push(t):n(t),i.a=(a,s,i)=>{var c,u,f,p=i&&[],l=a.exports,d=!0,_=!1,h=(t,r,n)=>{_||(_=!0,r.r+=t.length,t.map(((t,o)=>t[e](r,n))),_=!1)},w=new Promise(((e,t)=>{f=t,u=()=>(e(l),r(p),p=0)}));w[t]=l,w[e]=(e,t)=>{if(d)return n(e);c&&h(c,e,t),o(p,e),w.catch(t)},a.exports=w,s((a=>{if(!a)return u();var s,i;c=(a=>a.map((a=>{if(null!==a&&"object"==typeof a){if(a[e])return a;if(a.then){var s=[];a.then((e=>{i[t]=e,r(s),s=0}));var i={};return i[e]=(e,t)=>(o(s,e),a.catch(t)),i}}var c={};return c[e]=e=>n(e),c[t]=a,c})))(a);var f=new Promise(((e,r)=>{(s=()=>e(i=c.map((e=>e[t])))).r=0,h(c,s,r)}));return s.r?f:i})).then(u,f),d=!1},i.d=(e,t)=>{for(var r in t)i.o(t,r)&&!i.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},i.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),i.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),i.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),i.v=(e,t,r,n)=>{var o=fetch(i.p+""+r+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,n).then((t=>Object.assign(e,t.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)))},(()=>{var e;i.g.importScripts&&(e=i.g.location+"");var t=i.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");r.length&&(e=r[r.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),i.p=e})(),i(492)})();