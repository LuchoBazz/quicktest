"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[326],{3905:function(e,t,r){r.d(t,{Zo:function(){return l},kt:function(){return f}});var a=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function p(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function s(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?p(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):p(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function u(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},p=Object.keys(e);for(a=0;a<p.length;a++)r=p[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var p=Object.getOwnPropertySymbols(e);for(a=0;a<p.length;a++)r=p[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var o=a.createContext({}),i=function(e){var t=a.useContext(o),r=t;return e&&(r="function"==typeof e?e(t):s(s({},t),e)),r},l=function(e){var t=i(e.components);return a.createElement(o.Provider,{value:t},e.children)},c={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,p=e.originalType,o=e.parentName,l=u(e,["components","mdxType","originalType","parentName"]),m=i(r),f=n,d=m["".concat(o,".").concat(f)]||m[f]||c[f]||p;return r?a.createElement(d,s(s({ref:t},l),{},{components:r})):a.createElement(d,s({ref:t},l))}));function f(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var p=r.length,s=new Array(p);s[0]=m;var u={};for(var o in t)hasOwnProperty.call(t,o)&&(u[o]=t[o]);u.originalType=e,u.mdxType="string"==typeof e?e:n,s[1]=u;for(var i=2;i<p;i++)s[i]=r[i];return a.createElement.apply(null,s)}return a.createElement.apply(null,r)}m.displayName="MDXCreateElement"},5256:function(e,t,r){r.r(t),r.d(t,{assets:function(){return l},contentTitle:function(){return o},default:function(){return f},frontMatter:function(){return u},metadata:function(){return i},toc:function(){return c}});var a=r(7462),n=r(3366),p=(r(7294),r(3905)),s=["components"],u={sidebar_position:4,title:"Output Examples",sidebar_label:"Output"},o=void 0,i={unversionedId:"examples/output",id:"examples/output",title:"Output Examples",description:"Run Examples",source:"@site/docs/examples/output.md",sourceDirName:"examples",slug:"/examples/output",permalink:"/quicktest/docs/examples/output",draft:!1,editUrl:"https://github.com/LuisMBaezCo/quicktest/tree/main/website/docs/examples/output.md",tags:[],version:"current",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"Output Examples",sidebar_label:"Output"},sidebar:"tutorialSidebar",previous:{title:"Check",permalink:"/quicktest/docs/examples/check"},next:{title:"Supported Languages",permalink:"/quicktest/docs/languages/supported-languages"}},l={},c=[{value:"Run Examples",id:"run-examples",level:2}],m={toc:c};function f(e){var t=e.components,r=(0,n.Z)(e,s);return(0,p.kt)("wrapper",(0,a.Z)({},m,r,{components:t,mdxType:"MDXLayout"}),(0,p.kt)("h2",{id:"run-examples"},"Run Examples"),(0,p.kt)("pre",null,(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"git clone https://github.com/LuisMBaezCo/quicktest.git\n\ncd quicktest/examples/output\n")),(0,p.kt)("ul",null,(0,p.kt)("li",{parentName:"ul"},(0,p.kt)("h3",{parentName:"li",id:"cpp-examples"},"cpp examples"),(0,p.kt)("pre",{parentName:"li"},(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest output --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac\n")),(0,p.kt)("p",{parentName:"li"},"  Or which saves the output to a file"),(0,p.kt)("pre",{parentName:"li"},(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest output --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac --save-out\n")),(0,p.kt)("p",{parentName:"li"},"  or shorter"),(0,p.kt)("pre",{parentName:"li"},(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest output -t cpp/main.cpp -p test_cases/testcase_ac --save-out\n"))),(0,p.kt)("li",{parentName:"ul"},(0,p.kt)("h3",{parentName:"li",id:"python-examples"},"python examples"),(0,p.kt)("pre",{parentName:"li"},(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest output --target-file=python/main.py --prefix=test_cases/testcase_ac\n")),(0,p.kt)("p",{parentName:"li"},"  Or which saves the output to a file"),(0,p.kt)("pre",{parentName:"li"},(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest output --target-file=python/main.py --prefix=test_cases/testcase_ac --save-out\n")),(0,p.kt)("p",{parentName:"li"},"  or shorter"),(0,p.kt)("pre",{parentName:"li"},(0,p.kt)("code",{parentName:"pre",className:"language-shell"},"quicktest output -t python/main.py -p test_cases/testcase_ac --save-out\n")))))}f.isMDXComponent=!0}}]);