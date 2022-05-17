"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[320],{3905:function(e,t,n){n.d(t,{Zo:function(){return u},kt:function(){return k}});var a=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var o=a.createContext({}),p=function(e){var t=a.useContext(o),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},u=function(e){var t=p(e.components);return a.createElement(o.Provider,{value:t},e.children)},s={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,o=e.parentName,u=c(e,["components","mdxType","originalType","parentName"]),m=p(n),k=r,d=m["".concat(o,".").concat(k)]||m[k]||s[k]||i;return n?a.createElement(d,l(l({ref:t},u),{},{components:n})):a.createElement(d,l({ref:t},u))}));function k(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,l=new Array(i);l[0]=m;var c={};for(var o in t)hasOwnProperty.call(t,o)&&(c[o]=t[o]);c.originalType=e,c.mdxType="string"==typeof e?e:r,l[1]=c;for(var p=2;p<i;p++)l[p]=n[p];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},1298:function(e,t,n){n.r(t),n.d(t,{assets:function(){return u},contentTitle:function(){return o},default:function(){return k},frontMatter:function(){return c},metadata:function(){return p},toc:function(){return s}});var a=n(7462),r=n(3366),i=(n(7294),n(3905)),l=["components"],c={sidebar_position:3,title:"Check subcommand",sidebar_label:"Check"},o=void 0,p={unversionedId:"usage/check",id:"usage/check",title:"Check subcommand",description:"quicktest check | qt check",source:"@site/docs/usage/check.md",sourceDirName:"usage",slug:"/usage/check",permalink:"/docs/usage/check",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/usage/check.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3,title:"Check subcommand",sidebar_label:"Check"},sidebar:"tutorialSidebar",previous:{title:"Cmp",permalink:"/docs/usage/cmp"},next:{title:"Output",permalink:"/docs/usage/output"}},u={},s=[{value:"<code>quicktest check | qt check</code>",id:"quicktest-check--qt-check",level:3},{value:"Demo",id:"demo",level:3},{value:"Subcommand Structure",id:"subcommand-structure",level:3}],m={toc:s};function k(e){var t=e.components,c=(0,r.Z)(e,l);return(0,i.kt)("wrapper",(0,a.Z)({},m,c,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h3",{id:"quicktest-check--qt-check"},(0,i.kt)("inlineCode",{parentName:"h3"},"quicktest check | qt check")),(0,i.kt)("p",null,"In some problems more than one answer is accepted, so the ",(0,i.kt)("inlineCode",{parentName:"p"},"quicktest cmp")," command would not work correctly, in this case a script checker is used to verify the correctness of the algorithm."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp\n")),(0,i.kt)("p",null,"or shorter"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-shell"},"qt check --t main.cpp -c correct.cpp -g gen.cpp\n")),(0,i.kt)("h3",{id:"demo"},"Demo"),(0,i.kt)("p",null,(0,i.kt)("img",{alt:"check gif",src:n(8042).Z,width:"2051",height:"983"})),(0,i.kt)("h3",{id:"subcommand-structure"},"Subcommand Structure"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"quicktest check | qt check")),(0,i.kt)("p",{parentName:"li"},"  ",(0,i.kt)("strong",{parentName:"p"},"Required Options")),(0,i.kt)("ul",{parentName:"li"},(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"-t=<value> | --target-file=<value>"))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"-c=<value> | --checker-file=<value>"))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"-g=<value> | --gen-file=<value>")),(0,i.kt)("p",{parentName:"li"},(0,i.kt)("strong",{parentName:"p"},"Other Options"))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"--test-cases=<value> | --tc=<value> [default: 1000]"))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"--timeout=<value> | --tout=<value> [default: 2000]")," Unit of time: ",(0,i.kt)("inlineCode",{parentName:"p"},"ms"))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]")," Unit of time: ",(0,i.kt)("inlineCode",{parentName:"p"},"bytes"))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("inlineCode",{parentName:"p"},"--prefix=<value> | -p=<value>")," conflict with ",(0,i.kt)("inlineCode",{parentName:"p"},"--gen-file")," (Only one can be used at a time)1")))),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("p",{parentName:"li"},(0,i.kt)("strong",{parentName:"p"},"Flags of the ",(0,i.kt)("inlineCode",{parentName:"strong"},"cmp"),", ",(0,i.kt)("inlineCode",{parentName:"strong"},"stress")," and ",(0,i.kt)("inlineCode",{parentName:"strong"},"check")," subcommands")),(0,i.kt)("ul",{parentName:"li"},(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--break-bad | --break"),"  Break if WA, TLE or RTE states occurs"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--run-ac"),"     Run test cases Accepted"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--run-all"),"    Run all test cases"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--run-rte"),"    Run test cases Run Time Error"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--run-tle"),"    Run test cases Time Limited Exceeded"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--run-wa"),"     Run test cases Wrong Answer"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--save-all"),"   Save all test cases"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("inlineCode",{parentName:"li"},"--save-bad"),"   Save only bad cases with WA, TLE or RTE states")))))}k.isMDXComponent=!0},8042:function(e,t,n){t.Z=n.p+"assets/images/check-e947f099ca199e2c4fa30a6208980be5.gif"}}]);