"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[436],{3905:function(n,e,t){t.d(e,{Zo:function(){return l},kt:function(){return d}});var r=t(7294);function o(n,e,t){return e in n?Object.defineProperty(n,e,{value:t,enumerable:!0,configurable:!0,writable:!0}):n[e]=t,n}function i(n,e){var t=Object.keys(n);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(n);e&&(r=r.filter((function(e){return Object.getOwnPropertyDescriptor(n,e).enumerable}))),t.push.apply(t,r)}return t}function a(n){for(var e=1;e<arguments.length;e++){var t=null!=arguments[e]?arguments[e]:{};e%2?i(Object(t),!0).forEach((function(e){o(n,e,t[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(n,Object.getOwnPropertyDescriptors(t)):i(Object(t)).forEach((function(e){Object.defineProperty(n,e,Object.getOwnPropertyDescriptor(t,e))}))}return n}function c(n,e){if(null==n)return{};var t,r,o=function(n,e){if(null==n)return{};var t,r,o={},i=Object.keys(n);for(r=0;r<i.length;r++)t=i[r],e.indexOf(t)>=0||(o[t]=n[t]);return o}(n,e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(n);for(r=0;r<i.length;r++)t=i[r],e.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(n,t)&&(o[t]=n[t])}return o}var s=r.createContext({}),u=function(n){var e=r.useContext(s),t=e;return n&&(t="function"==typeof n?n(e):a(a({},e),n)),t},l=function(n){var e=u(n.components);return r.createElement(s.Provider,{value:e},n.children)},p={inlineCode:"code",wrapper:function(n){var e=n.children;return r.createElement(r.Fragment,{},e)}},f=r.forwardRef((function(n,e){var t=n.components,o=n.mdxType,i=n.originalType,s=n.parentName,l=c(n,["components","mdxType","originalType","parentName"]),f=u(t),d=o,g=f["".concat(s,".").concat(d)]||f[d]||p[d]||i;return t?r.createElement(g,a(a({ref:e},l),{},{components:t})):r.createElement(g,a({ref:e},l))}));function d(n,e){var t=arguments,o=e&&e.mdxType;if("string"==typeof n||o){var i=t.length,a=new Array(i);a[0]=f;var c={};for(var s in e)hasOwnProperty.call(e,s)&&(c[s]=e[s]);c.originalType=n,c.mdxType="string"==typeof n?n:o,a[1]=c;for(var u=2;u<i;u++)a[u]=t[u];return r.createElement.apply(null,a)}return r.createElement.apply(null,t)}f.displayName="MDXCreateElement"},2783:function(n,e,t){t.r(e),t.d(e,{assets:function(){return l},contentTitle:function(){return s},default:function(){return d},frontMatter:function(){return c},metadata:function(){return u},toc:function(){return p}});var r=t(7462),o=t(3366),i=(t(7294),t(3905)),a=["components"],c={sidebar_position:2,title:"Configuration",sidebar_label:"Configuration"},s=void 0,u={unversionedId:"getting-started/configuration",id:"getting-started/configuration",title:"Configuration",description:"~/.quicktest/config.yaml",source:"@site/docs/getting-started/configuration.md",sourceDirName:"getting-started",slug:"/getting-started/configuration",permalink:"/docs/getting-started/configuration",editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/getting-started/configuration.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2,title:"Configuration",sidebar_label:"Configuration"},sidebar:"tutorialSidebar",previous:{title:"Installation",permalink:"/docs/getting-started/installation"},next:{title:"Stress",permalink:"/docs/usage/stress"}},l={},p=[],f={toc:p};function d(n){var e=n.components,t=(0,o.Z)(n,a);return(0,i.kt)("wrapper",(0,r.Z)({},f,t,{components:e,mdxType:"MDXLayout"}),(0,i.kt)("p",null,(0,i.kt)("inlineCode",{parentName:"p"},"~/.quicktest/config.yaml")),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-json"},'{\n    "languages":[\n        {\n            "id":"Language::Cpp",\n            "name":"GNU C++17 (64)",\n            "extensions":[\n                "h",\n                "hh",\n                "hpp",\n                "hxx",\n                "h++",\n                "cc",\n                "cpp",\n                "cxx",\n                "c++"\n            ],\n            "description":"Compilador GNU C++ version 17",\n            "env":{\n                "PROGRAM":"g++",\n                "STANDARD":"-std=c++17"\n            },\n            "compile":{\n                "unix":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qtest/${FILE_NAME_BINARY}.o",\n                "windows":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qtest/${FILE_NAME_BINARY}.exe"\n            },\n            "execute":{\n                "unix":".qtest/${FILE_NAME_BINARY}.o",\n                "windows":".qtest/${FILE_NAME_BINARY}.exe"\n            },\n            "check_installed":"${PROGRAM} --help",\n            "libraries":[\n                {\n                    "name":"ac-library",\n                    "version":"1.4",\n                    "source":"https://github.com/atcoder/ac-library"\n                }\n            ]\n        },\n        {\n            "id":"Language::Python",\n            "name":"Python 3",\n            "extensions":[\n                "py"\n            ],\n            "description":"Interprete del Lenguaje Python 3",\n            "env":{\n                "PROGRAM":"python3"\n            },\n            "execute":{\n                "unix":"${PROGRAM} ${FILE_NAME}.py",\n                "windows":"${PROGRAM} ${FILE_NAME}.py"\n            },\n            "check_installed":"${PROGRAM} --help",\n            "libraries":[\n                {\n                    "name":"numba",\n                    "version":"*"\n                }\n            ]\n        }\n    ]\n}\n')))}d.isMDXComponent=!0}}]);