"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[874],{3905:function(e,t,n){n.d(t,{Zo:function(){return c},kt:function(){return d}});var a=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function s(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?s(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):s(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function p(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},s=Object.keys(e);for(a=0;a<s.length;a++)n=s[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var s=Object.getOwnPropertySymbols(e);for(a=0;a<s.length;a++)n=s[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var i=a.createContext({}),u=function(e){var t=a.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},c=function(e){var t=u(e.components);return a.createElement(i.Provider,{value:t},e.children)},o={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,s=e.originalType,i=e.parentName,c=p(e,["components","mdxType","originalType","parentName"]),m=u(n),d=r,k=m["".concat(i,".").concat(d)]||m[d]||o[d]||s;return n?a.createElement(k,l(l({ref:t},c),{},{components:n})):a.createElement(k,l({ref:t},c))}));function d(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var s=n.length,l=new Array(s);l[0]=m;var p={};for(var i in t)hasOwnProperty.call(t,i)&&(p[i]=t[i]);p.originalType=e,p.mdxType="string"==typeof e?e:r,l[1]=p;for(var u=2;u<s;u++)l[u]=n[u];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},9723:function(e,t,n){n.r(t),n.d(t,{assets:function(){return c},contentTitle:function(){return i},default:function(){return d},frontMatter:function(){return p},metadata:function(){return u},toc:function(){return o}});var a=n(7462),r=n(3366),s=(n(7294),n(3905)),l=["components"],p={sidebar_position:1,title:"Stress Examples",sidebar_label:"Stress"},i=void 0,u={unversionedId:"examples/stress",id:"examples/stress",title:"Stress Examples",description:"Run Examples",source:"@site/docs/examples/stress.md",sourceDirName:"examples",slug:"/examples/stress",permalink:"/docs/examples/stress",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/examples/stress.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Stress Examples",sidebar_label:"Stress"},sidebar:"tutorialSidebar",previous:{title:"Example",permalink:"/docs/usage/example"},next:{title:"Cmp",permalink:"/docs/examples/cmp"}},c={},o=[{value:"Run Examples",id:"run-examples",level:2},{value:"Save Test Cases",id:"save-test-cases",level:2},{value:"Run Saved Test Cases",id:"run-saved-test-cases",level:2}],m={toc:o};function d(e){var t=e.components,n=(0,r.Z)(e,l);return(0,s.kt)("wrapper",(0,a.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,s.kt)("h2",{id:"run-examples"},"Run Examples"),(0,s.kt)("ul",null,(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("h3",{parentName:"li",id:"cpp-examples"},"cpp examples"),(0,s.kt)("pre",{parentName:"li"},(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"git clone https://github.com/LuisMBaezCo/quicktest.git\n\ncd quicktest/examples/stress/cpp\n")),(0,s.kt)("pre",{parentName:"li"},(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000\n")),(0,s.kt)("p",{parentName:"li"},"  or shorter"),(0,s.kt)("pre",{parentName:"li"},(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress -t main.cpp -g gen.cpp --tc 15 --tout 1000\n")))),(0,s.kt)("ul",null,(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("h3",{parentName:"li",id:"python-examples"},"python examples"),(0,s.kt)("pre",{parentName:"li"},(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"git clone https://github.com/LuisMBaezCo/quicktest.git\n\ncd quicktest/examples/stress/python\n")),(0,s.kt)("pre",{parentName:"li"},(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress --target-file=main.py --gen-file=gen.py --test-cases=15 --timeout=1000\n")),(0,s.kt)("p",{parentName:"li"},"  or shorter"),(0,s.kt)("pre",{parentName:"li"},(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress -t main.py -t gen.py --tc 15 --tout 1000\n")))),(0,s.kt)("h2",{id:"save-test-cases"},"Save Test Cases"),(0,s.kt)("p",null,"you can use the following flags"),(0,s.kt)("ul",null,(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--save-all"),"   Save all test cases"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--save-bad"),"   Save only bad cases with WA, TLE or RTE states")),(0,s.kt)("p",null,"for example"),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000 --save-bad\n")),(0,s.kt)("p",null,"or shorter"),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress --t main.cpp -g gen.cpp --tc 15 --tout 1000 --save-bad\n")),(0,s.kt)("h2",{id:"run-saved-test-cases"},"Run Saved Test Cases"),(0,s.kt)("p",null,"you can use the following flags"),(0,s.kt)("ul",null,(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-ac"),"     Run test cases Accepted"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-all"),"    Run all test cases"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-rte"),"    Run test cases Run Time Error"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-tle"),"    Run test cases Time Limited Exceeded"),(0,s.kt)("li",{parentName:"ul"},(0,s.kt)("inlineCode",{parentName:"li"},"--run-wa"),"     Run test cases Wrong Answer")),(0,s.kt)("p",null,"for example"),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000 --run-tle\n")),(0,s.kt)("p",null,"or shorter"),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest stress -t main.cpp -g gen.cpp --tc 15 --tout 1000 --run-tle\n")))}d.isMDXComponent=!0}}]);