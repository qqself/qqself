import{e,n as t,h as r,T as n,r as a,t as i}from"./vendor.0924b8be.js";let o;!function(){const e=document.createElement("link").relList;if(!(e&&e.supports&&e.supports("modulepreload"))){for(const e of document.querySelectorAll('link[rel="modulepreload"]'))t(e);new MutationObserver((e=>{for(const r of e)if("childList"===r.type)for(const e of r.addedNodes)"LINK"===e.tagName&&"modulepreload"===e.rel&&t(e)})).observe(document,{childList:!0,subtree:!0})}function t(e){if(e.ep)return;e.ep=!0;const t=function(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerpolicy&&(t.referrerPolicy=e.referrerpolicy),"use-credentials"===e.crossorigin?t.credentials="include":"anonymous"===e.crossorigin?t.credentials="omit":t.credentials="same-origin",t}(e);fetch(e.href,t)}}();let s=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let c=null;function d(){return null!==c&&c.buffer===o.memory.buffer||(c=new Uint8Array(o.memory.buffer)),c}function l(e,t){return s.decode(d().subarray(e,e+t))}let u=0,p=new TextEncoder("utf-8");const y="function"==typeof p.encodeInto?function(e,t){return p.encodeInto(e,t)}:function(e,t){const r=p.encode(e);return t.set(r),{read:e.length,written:r.length}};let m=null;function f(){return null!==m&&m.buffer===o.memory.buffer||(m=new Int32Array(o.memory.buffer)),m}function g(e){try{const i=o.__wbindgen_add_to_stack_pointer(-16);var t=function(e,t,r){if("string"!=typeof e)throw new Error("expected a string argument");if(void 0===r){const r=p.encode(e),n=t(r.length);return d().subarray(n,n+r.length).set(r),u=r.length,n}let n=e.length,a=t(n);const i=d();let o=0;for(;o<n;o++){const t=e.charCodeAt(o);if(t>127)break;i[a+o]=t}if(o!==n){0!==o&&(e=e.slice(o)),a=r(a,n,n=o+3*e.length);const t=d().subarray(a+o,a+n),i=y(e,t);if(i.read!==e.length)throw new Error("failed to pass whole string");o+=i.written}return u=o,a}(e,o.__wbindgen_malloc,o.__wbindgen_realloc),r=u;o.parse(i,t,r);var n=f()[i/4+0],a=f()[i/4+1];return l(n,a)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(n,a)}}async function h(e){void 0===e&&(e=new URL("/assets/rsw~wrapper_bg.1ef5cc7c.wasm",window.location));const t={wbg:{}};t.wbg.__wbindgen_throw=function(e,t){throw new Error(l(e,t))},("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:r,module:n}=await async function(e,t){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,t)}catch(r){if("application/wasm"==e.headers.get("Content-Type"))throw r;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r)}const n=await e.arrayBuffer();return await WebAssembly.instantiate(n,t)}{const r=await WebAssembly.instantiate(e,t);return r instanceof WebAssembly.Instance?{instance:r,module:e}:r}}(await e,t);return o=r.exports,h.__wbindgen_wasm_module=n,o}var b=Object.defineProperty,v=Object.getOwnPropertyDescriptor,w=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?v(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&b(t,r,i),i};let q=class extends r{constructor(){super(...arguments),this.text=""}render(){const e=g(this.text);return n`EntryInput <input type="text" value=${e}/>`}};w([e()],q.prototype,"text",2),q=w([t("q-entry-input")],q);var x=Object.defineProperty,_=Object.getOwnPropertyDescriptor,O=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?_(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&x(t,r,i),i};let $=class extends r{constructor(){super(...arguments),this.uncategorized=""}render(){return n`
            <div class="summary">
                <h2>Day summary</h2>
                <div class="uncategorized">Uncategorized ${this.uncategorized}</div>
                <slot></slot>
                <button>Add</button>
                <button>Search</button>
            </div>`}};$.styles=a`
    .uncategorized { border: solid 1px black }
    `,O([e()],$.prototype,"uncategorized",2),$=O([t("q-day-summary")],$);var P=Object.defineProperty,k=Object.getOwnPropertyDescriptor,j=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?k(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&P(t,r,i),i};let z=class extends r{constructor(){super(...arguments),this.name="",this.progress=0}render(){return n`
            <div class="progress">
                <div class="name">${this.name}</div>
                <progress value="${this.progress}" max="100"></progress>
            </div>`}};j([e()],z.prototype,"name",2),j([e()],z.prototype,"progress",2),z=j([t("q-entry-progress")],z);var A=Object.defineProperty,S=Object.getOwnPropertyDescriptor,D=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?S(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&A(t,r,i),i};let I=class extends r{constructor(){super(...arguments),this.text="",this.start="",this.end=""}render(){return n`
            <div class="editor">
                <q-entry-input text="${this.text}"></q-entry-input>
                <div>
                    Start <input type="text" value="${this.start}"/>
                </div>
                <div>
                    End <input type="text" value="${this.end}"/>
                </div>
                <button>Save</button>
            </div>`}};I.styles=a`
    .uncategorized { border: solid 1px black }
    `,D([e()],I.prototype,"text",2),D([e()],I.prototype,"start",2),D([e()],I.prototype,"end",2),I=D([t("q-entry-editor")],I);var L=Object.defineProperty,W=Object.getOwnPropertyDescriptor,E=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?W(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&L(t,r,i),i};let M=class extends r{constructor(){super(...arguments),this.query="",this.data=[]}time(){const e=this.data.reduce(((e,t)=>e+t.time),0);if(e<60)return`${e} min`;const t=Math.floor(e/60);return`${t}:${e-60*t}`}render(){const e=this.data.map((e=>`["${e.name}",${e.time}]`)).join(",");return n`
            <div class="search">
                <h2>Search</h2>
                <q-entry-input text="${this.query}"></q-entry-input>
                <div class="filter">
                    <input type="radio" name="filter" id="all" value="all" checked>
                    <label for="all">All</label>
                    
                    <input type="radio" name="filter" id="year" value="year">
                    <label for="year">Last year</label>

                    <input type="radio" name="filter" id="month" value="month">
                    <label for="month">Last month</label>

                    <input type="radio" name="filter" id="week" value="week">
                    <label for="week">Last week</label>
                </div>
                <div class="summary">
                    Count: ${this.data.length}
                    <br>
                    Time: ${this.time()}
                </div>
                <div class="chart">
                    <google-chart data='[["Period","Time"],${e}]'></google-chart>
                </div>
            </div>`}};M.styles=a`
    .filter { margin: 10px 0; }
    `,E([e()],M.prototype,"query",2),E([e()],M.prototype,"data",2),M=E([t("q-entry-search")],M);var R=Object.defineProperty,C=Object.getOwnPropertyDescriptor,T=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?C(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&R(t,r,i),i};let U=class extends r{constructor(){super(...arguments),this.name=""}render(){const e=window.location.hash.slice(1);return""!=e&&"devcards"!=e&&!this.name.toLowerCase().includes(e)?null:n`
            <div class="card">
                <div class="name">${this.name}</div>
                <div class="content">
                    <slot></slot>
                </div>
            </div>`}};U.styles=a`
    .card { border: solid 1px black; margin-bottom: 20px; }
    .name { text-align: center }
    .content { margin: 10px; }
    `,T([e()],U.prototype,"name",2),U=T([t("q-card")],U);let F=class extends r{render(){return n`
            <div>
                <q-card name="entryInput: Empty">
                    <q-entry-input text=""></q-entry-input>
                </q-card>
                <q-card name="entryInput: With data">
                    <q-entry-input text="tag1 prop1 val1. tag2"></q-entry-input>
                </q-card>
                <q-card name="daySummary: Empty">
                    <q-day-summary uncategorized="1:10m"></q-day-summary>
                </q-card>
                <q-card name="entryProgress: Zero">
                    <q-entry-progress progress="0" name="Work"></q-entry-progress>
                </q-card>
                <q-card name="entryProgress: Partial">
                    <q-entry-progress progress="85" name="Work"></q-entry-progress>
                </q-card>
                <q-card name="entryProgress: Complete">
                    <q-entry-progress progress="100" name="Work"></q-entry-progress>
                </q-card>
                <q-card name="entryEditor: Simple">
                    <q-entry-editor text="tag1 prop1 val2" start="11:20" end="11:56"></q-entry-editor>
                </q-card>
                <q-card name="entrySearch: Found">
                    <q-entry-search query="tag1" .data=${[{name:"January",time:22},{name:"February",time:49},{name:"March",time:12},{name:"April",time:24}]}></q-entry-search>
                </q-card>
            </div>`}};F=T([t("q-devcards")],F);const N=e=>console.log(`${(new Date).toISOString()} ${e}`);var B=Object.defineProperty,H=Object.getOwnPropertyDescriptor,J=(e,t,r,n)=>{for(var a,i=n>1?void 0:n?H(t,r):t,o=e.length-1;o>=0;o--)(a=e[o])&&(i=(n?a(t,r,i):a(i))||i);return n&&i&&B(t,r,i),i};let K=class extends r{constructor(){super(),this.initialized=!1,(async()=>{N("Initializing..."),await h(),N("Initialization done"),this.initialized=!0})()}route(){return""==window.location.hash.slice(1)?"main":"devcards"}renderDevcards(){return n`<q-devcards></q-devcards>`}renderMain(){return n`
            <p>
                Main page with input
                <q-entry-input text="Hello World"></q-entry-input>
            </p>
        `}render(){if(!this.initialized)return n`Loading...`;const e=this.route();switch(N(`Rendering ${e}`),e){case"main":return this.renderMain();case"devcards":return this.renderDevcards()}}};K.styles=a`p { color: blue }`,J([i()],K.prototype,"initialized",2),K=J([t("q-main")],K);
