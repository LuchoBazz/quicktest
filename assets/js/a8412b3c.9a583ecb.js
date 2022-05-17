"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[199],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return d}});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function s(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?s(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):s(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},s=Object.keys(e);for(r=0;r<s.length;r++)n=s[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var s=Object.getOwnPropertySymbols(e);for(r=0;r<s.length;r++)n=s[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var o=r.createContext({}),u=function(e){var t=r.useContext(o),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=u(e.components);return r.createElement(o.Provider,{value:t},e.children)},c={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,s=e.originalType,o=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),m=u(n),d=a,k=m["".concat(o,".").concat(d)]||m[d]||c[d]||s;return n?r.createElement(k,i(i({ref:t},p),{},{components:n})):r.createElement(k,i({ref:t},p))}));function d(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var s=n.length,i=new Array(s);i[0]=m;var l={};for(var o in t)hasOwnProperty.call(t,o)&&(l[o]=t[o]);l.originalType=e,l.mdxType="string"==typeof e?e:a,i[1]=l;for(var u=2;u<s;u++)i[u]=n[u];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},6538:function(e,t,n){n.r(t),n.d(t,{assets:function(){return p},contentTitle:function(){return o},default:function(){return d},frontMatter:function(){return l},metadata:function(){return u},toc:function(){return c}});var r=n(7462),a=n(3366),s=(n(7294),n(3905)),i=["components"],l={sidebar_position:1,title:"Stress subcommand",sidebar_label:"Stress"},o=void 0,u={unversionedId:"usage/stress",id:"usage/stress",title:"Stress subcommand",description:"quicktest stress | qt stress",source:"@site/docs/usage/stress.md",sourceDirName:"usage",slug:"/usage/stress",permalink:"/docs/usage/stress",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/usage/stress.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Stress subcommand",sidebar_label:"Stress"},sidebar:"tutorialSidebar",previous:{title:"Errors",permalink:"/docs/getting-started/errors"},next:{title:"Cmp",permalink:"/docs/usage/cmp"}},p={},c=[{value:"<code>quicktest stress | qt stress</code>",id:"quicktest-stress--qt-stress",level:3},{value:"Demo",id:"demo",level:3},{value:"Subcommand Structure",id:"subcommand-structure",level:3}],m={toc:c};function d(e){var t=e.components,l=(0,a.Z)(e,i);return(0,s.kt)("wrapper",(0,r.Z)({},m,l,{components:t,mdxType:"MDXLayout"}),(0,s.kt)("h3",{id:"quicktest-stress--qt-stress"},(0,s.kt)("inlineCode",{parentName:"h3"},"quicktest stress | qt stress")),(0,s.kt)("p",null,"Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases."),(0,s.kt)("p",null,(0,s.kt)("strong",{parentName:"p"},"Note:")," In this scenario there is no slower solution with which to compare the correction."),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress --target-file=main.cpp --gen-file=gen.cpp\n")),(0,s.kt)("p",null,"or shorter"),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"qt stress -t main.cpp -g gen.cpp\n")),(0,s.kt)("h3",{id:"demo"},"Demo"),(0,s.kt)("p",null,(0,s.kt)("img",{alt:"stress gif",src:n(7289).Z,width:"2051",height:"983"})),(0,s.kt)("h3",{id:"subcommand-structure"},"Subcommand Structure"),(0,s.kt)("ul",null,(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"quicktest stress | qt stress")),(0,s.kt)("p",{parentName:"li"},"  ",(0,s.kt)("strong",{parentName:"p"},"Required Options")),(0,s.kt)("ul",{parentName:"li"},(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"-t=<value> | --target-file=<value>"))),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"-g=<value> | --gen-file=<value>")),(0,s.kt)("p",{parentName:"li"},(0,s.kt)("strong",{parentName:"p"},"Other Options"))),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"--test-cases=<value> | --tc=<value> [default: 1000]"))),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"--timeout=<value> | --tout=<value> [default: 2000]")," Unit of time: ",(0,s.kt)("inlineCode",{parentName:"p"},"ms"))),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]")," Unit of time: ",(0,s.kt)("inlineCode",{parentName:"p"},"bytes"))),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("inlineCode",{parentName:"p"},"--prefix=<value> | -p=<value>")," conflict with ",(0,s.kt)("inlineCode",{parentName:"p"},"--gen-file")," (Only one can be used at a time)")))),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("p",{parentName:"li"},(0,s.kt)("strong",{parentName:"p"},"Flags of the ",(0,s.kt)("inlineCode",{parentName:"strong"},"cmp"),", ",(0,s.kt)("inlineCode",{parentName:"strong"},"stress")," and ",(0,s.kt)("inlineCode",{parentName:"strong"},"check")," subcommands")),(0,s.kt)("ul",{parentName:"li"},(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--break-bad | --break"),"  Break if WA, TLE or RTE states occurs"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-ac"),"     Run test cases Accepted"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-all"),"    Run all test cases"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-rte"),"    Run test cases Run Time Error"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-tle"),"    Run test cases Time Limited Exceeded"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-wa"),"     Run test cases Wrong Answer"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--save-all"),"   Save all test cases"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--save-bad"),"   Save only bad cases with WA, TLE or RTE states")))))}d.isMDXComponent=!0},7289:function(e,t,n){t.Z=n.p+"assets/images/stress-5b3f4114f5d04d7541ee4714c4115da5.gif"}}]);