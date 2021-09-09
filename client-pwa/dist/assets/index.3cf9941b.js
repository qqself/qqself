import{r as e,o as t,e as r,h as n,T as i,n as a,t as s}from"./vendor.ec35c5b3.js";let o;!function(){const e=document.createElement("link").relList;if(!(e&&e.supports&&e.supports("modulepreload"))){for(const e of document.querySelectorAll('link[rel="modulepreload"]'))t(e);new MutationObserver((e=>{for(const r of e)if("childList"===r.type)for(const e of r.addedNodes)"LINK"===e.tagName&&"modulepreload"===e.rel&&t(e)})).observe(document,{childList:!0,subtree:!0})}function t(e){if(e.ep)return;e.ep=!0;const t=function(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerpolicy&&(t.referrerPolicy=e.referrerpolicy),"use-credentials"===e.crossorigin?t.credentials="include":"anonymous"===e.crossorigin?t.credentials="omit":t.credentials="same-origin",t}(e);fetch(e.href,t)}}();let d=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});d.decode();let c=null;function l(){return null!==c&&c.buffer===o.memory.buffer||(c=new Uint8Array(o.memory.buffer)),c}function u(e,t){return d.decode(l().subarray(e,e+t))}let p=0,y=new TextEncoder("utf-8");const h="function"==typeof y.encodeInto?function(e,t){return y.encodeInto(e,t)}:function(e,t){const r=y.encode(e);return t.set(r),{read:e.length,written:r.length}};let m=null;function f(){return null!==m&&m.buffer===o.memory.buffer||(m=new Int32Array(o.memory.buffer)),m}function g(e){try{const a=o.__wbindgen_add_to_stack_pointer(-16);var t=function(e,t,r){if("string"!=typeof e)throw new Error("expected a string argument");if(void 0===r){const r=y.encode(e),n=t(r.length);return l().subarray(n,n+r.length).set(r),p=r.length,n}let n=e.length,i=t(n);const a=l();let s=0;for(;s<n;s++){const t=e.charCodeAt(s);if(t>127)break;a[i+s]=t}if(s!==n){0!==s&&(e=e.slice(s)),i=r(i,n,n=s+3*e.length);const t=l().subarray(i+s,i+n),a=h(e,t);if(a.read!==e.length)throw new Error("failed to pass whole string");s+=a.written}return p=s,i}(e,o.__wbindgen_malloc,o.__wbindgen_realloc),r=p;o.parse(a,t,r);var n=f()[a/4+0],i=f()[a/4+1];return u(n,i)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(n,i)}}async function v(e){void 0===e&&(e=new URL("/assets/rsw~wrapper_bg.1ef5cc7c.wasm",window.location));const t={wbg:{}};t.wbg.__wbindgen_throw=function(e,t){throw new Error(u(e,t))},("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:r,module:n}=await async function(e,t){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,t)}catch(r){if("application/wasm"==e.headers.get("Content-Type"))throw r;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r)}const n=await e.arrayBuffer();return await WebAssembly.instantiate(n,t)}{const r=await WebAssembly.instantiate(e,t);return r instanceof WebAssembly.Instance?{instance:r,module:e}:r}}(await e,t);return o=r.exports,v.__wbindgen_wasm_module=n,o}var b=Object.defineProperty,w=Object.getOwnPropertyDescriptor,x=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?w(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&b(t,r,a),a};let q=class extends n{constructor(){super(...arguments),this.onText=()=>{},this.mode="string"}onEnterInput(){const e=g(this.input.value.trim());this.onText(e),this.input.value=""}onEnterText(){const e=g(this.textarea.value.trim());this.onText(e),this.textarea.value=""}render(){return"string"==this.mode?i` <div class="container">
        <input class="input" type="text" />
        <button @click=${this.onEnterInput}>Add</button>
      </div>`:i`
      <div>
        <textarea class="text"></textarea>
        <button class="text_btn" @click=${this.onEnterText}>Add</button>
      </div>
    `}};q.styles=e`
    .container {
      display: flex;
      justify-content: space-between;
    }
    .input {
      width: 80%;
    }
    .text {
      margin: auto;
      display: block;
      width: 95%;
      min-height: 400px;
    }
    .textBtn {
      float: right;
      width: 100px;
      margin-top: 20px;
    }
  `,x([t("input",!0)],q.prototype,"input",2),x([t("textarea",!0)],q.prototype,"textarea",2),x([r()],q.prototype,"onText",2),x([r()],q.prototype,"mode",2),q=x([a("q-entry-input")],q);var $=Object.defineProperty,k=Object.getOwnPropertyDescriptor,A=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?k(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&$(t,r,a),a};let _=class extends n{constructor(){super(...arguments),this.uncategorized=""}render(){return i` <div class="summary">
      <h2>Day summary</h2>
      <div class="uncategorized">Uncategorized ${this.uncategorized}</div>
      <slot></slot>
      <button>Add</button>
      <button>Search</button>
    </div>`}};_.styles=e`
    .uncategorized {
      border: solid 1px black;
    }
  `,A([r()],_.prototype,"uncategorized",2),_=A([a("q-day-summary")],_);var O=Object.defineProperty,P=Object.getOwnPropertyDescriptor,j=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?P(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&O(t,r,a),a};let E=class extends n{constructor(){super(...arguments),this.name="",this.progress=0}render(){return i` <div class="progress">
      <div class="name">${this.name}</div>
      <progress value="${this.progress}" max="100"></progress>
    </div>`}};j([r()],E.prototype,"name",2),j([r({type:Number})],E.prototype,"progress",2),E=j([a("q-entry-progress")],E);var M=Object.defineProperty,S=Object.getOwnPropertyDescriptor,D=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?S(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&M(t,r,a),a};let I=class extends n{constructor(){super(...arguments),this.text="",this.start="",this.end=""}render(){return i` <div class="editor">
      <q-entry-input text="${this.text}"></q-entry-input>
      <div>Start <input type="text" value="${this.start}" /></div>
      <div>End <input type="text" value="${this.end}" /></div>
      <button>Save</button>
    </div>`}};I.styles=e`
    .uncategorized {
      border: solid 1px black;
    }
  `,D([r()],I.prototype,"text",2),D([r()],I.prototype,"start",2),D([r()],I.prototype,"end",2),I=D([a("q-entry-editor")],I);var z=Object.defineProperty,T=Object.getOwnPropertyDescriptor,L=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?T(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&z(t,r,a),a};let W=class extends n{constructor(){super(...arguments),this.query="",this.data=[]}time(){const e=this.data.reduce(((e,t)=>e+t.time),0);if(e<60)return`${e} min`;const t=Math.floor(e/60);return`${t}:${e-60*t}`}render(){const e=this.data.map((e=>`["${e.name}",${e.time}]`)).join(",");return i` <div class="search">
      <h2>Search</h2>
      <q-entry-input text="${this.query}"></q-entry-input>
      <div class="filter">
        <input type="radio" name="filter" id="all" value="all" checked />
        <label for="all">All</label>

        <input type="radio" name="filter" id="year" value="year" />
        <label for="year">Last year</label>

        <input type="radio" name="filter" id="month" value="month" />
        <label for="month">Last month</label>

        <input type="radio" name="filter" id="week" value="week" />
        <label for="week">Last week</label>
      </div>
      <div class="summary">
        Count: ${this.data.length}
        <br />
        Time: ${this.time()}
      </div>
      <div class="chart">
        <google-chart data='[["Period","Time"],${e}]'></google-chart>
      </div>
    </div>`}};W.styles=e`
    .filter {
      margin: 10px 0;
    }
  `,L([r()],W.prototype,"query",2),L([r({type:Array})],W.prototype,"data",2),W=L([a("q-entry-search")],W);var C=Object.defineProperty,R=Object.getOwnPropertyDescriptor,U=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?R(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&C(t,r,a),a};let B=class extends n{constructor(){super(...arguments),this.name=""}render(){const e=window.location.hash.slice(1);return""!=e&&"devcards"!=e&&!this.name.toLowerCase().includes(e)?null:i` <div class="card">
      <div class="name">${this.name}</div>
      <div class="content">
        <slot></slot>
      </div>
    </div>`}};B.styles=e`
    .card {
      border: solid 1px black;
      margin-bottom: 20px;
    }
    .name {
      text-align: center;
    }
    .content {
      margin: 10px;
    }
  `,U([r()],B.prototype,"name",2),B=U([a("q-card")],B);let N=class extends n{render(){return i` <div>
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
        <q-entry-search
          query="tag1"
          .data=${[{name:"January",time:22},{name:"February",time:49},{name:"March",time:12},{name:"April",time:24}]}
        ></q-entry-search>
      </q-card>
    </div>`}};N=U([a("q-devcards")],N);var F=Object.defineProperty,J=Object.getOwnPropertyDescriptor,K=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?J(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&F(t,r,a),a};let Z=class extends n{constructor(){super(...arguments),this.mode="recent",this.text=""}render(){const e=this.text.split("\n").slice("recent"==this.mode?-20:0);return i` <div class="list">
      <ul>
        ${e.map((e=>i` <li>${e}</li>`))}
      </ul>
    </div>`}};Z.styles=e``,K([r()],Z.prototype,"mode",2),K([r()],Z.prototype,"text",2),Z=K([a("q-entry-list")],Z);const G=e=>console.log(`${(new Date).toISOString()} ${e}`);class H{constructor(){this.key="data"}async read(){const e=window.localStorage.getItem(this.key);return e||""}async write(e){window.localStorage.setItem(this.key,e)}}var Q=Object.defineProperty,V=Object.getOwnPropertyDescriptor,X=(e,t,r,n)=>{for(var i,a=n>1?void 0:n?V(t,r):t,s=e.length-1;s>=0;s--)(i=e[s])&&(a=(n?i(t,r,a):i(a))||a);return n&&a&&Q(t,r,a),a};let Y=class extends n{constructor(){super(),this.initialized=!1,this.addMode="activity",this.entries="",this.onClick=e=>()=>{this.addMode=e},this.onEntryAdded=async e=>{console.log("Add to storage: ",e);const t=this.entries+"\n"+e;await(new H).write(t),this.entries=t,console.log(`Local storage updated ${this.entries}`)},(async()=>{G("Initializing..."),await v();const e=await(new H).read();this.initialized=!0,this.entries=e,G(`Initialization done, entries ${this.entries}`)})()}route(){return""==window.location.hash.slice(1)?"main":"devcards"}renderDevcards(){return i`<q-devcards></q-devcards>`}renderAddElement(){switch(this.addMode){case"activity":return i`
          <h3>Add ${this.addMode}</h3>
          <q-entry-list mode="recent" text=${this.entries}></q-entry-list>
          <q-entry-input mode="string" .onText=${this.onEntryAdded}></q-entry-input>
        `;case"data":return i`
          <h3>Add ${this.addMode}</h3>
          <q-entry-input mode="text" .onText=${this.onEntryAdded}></q-entry-input>
        `;case"reflexion":return i`
          <h3>Add ${this.addMode}</h3>
          <div style="display: flex; justify-content: space-between">
            <a href="">Day</a>
            <a href="">Week</a>
            <a href="">Month</a>
          </div>
          <br />
          <q-entry-input mode="text" .onText=${this.onEntryAdded}></q-entry-input>
        `}}renderMain(){const e={[this.addMode]:"active"};return i`
      <div class="container">
        <a href="">Search</a>
        <a href="">Dashboard</a>
        <a href="">Shortcuts</a>
      </div>
      <hr />
      <div class="container">
        <button @click=${this.onClick("activity")} class="${e.activity}">
          Add Activity
        </button>
        <button @click=${this.onClick("data")} class="${e.data}">Add Data</button>
        <button @click=${this.onClick("reflexion")} class="${e.reflexion}">
          Add Reflexion
        </button>
      </div>
      ${this.renderAddElement()}
    `}render(){if(!this.initialized)return i`Loading...`;const e=this.route();switch(G(`Rendering ${e}`),e){case"main":return this.renderMain();case"devcards":return this.renderDevcards()}}};Y.styles=e`
    .container {
      display: flex;
      justify-content: space-between;
    }
    .active {
      border: 1px solid Black;
    }
    h3 {
      width: 100%;
      text-align: center;
    }
    a:visited {
      color: blue;
    }
  `,X([s()],Y.prototype,"initialized",2),X([s()],Y.prototype,"addMode",2),X([s()],Y.prototype,"entries",2),Y=X([a("q-main")],Y);
