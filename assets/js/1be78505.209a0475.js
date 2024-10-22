"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[714,774],{8067:(e,t,n)=>{n.r(t),n.d(t,{default:()=>ge});var a=n(6540),l=n(53),o=n(1754),r=n(2967),c=n(1003),i=n(7559),s=n(2252),d=n(6588),m=n(6358),u=n(1312),b=n(3104),p=n(5062);const h={backToTopButton:"backToTopButton_sjWU",backToTopButtonShow:"backToTopButtonShow_xfvO"};function E(){const{shown:e,scrollToTop:t}=function(e){let{threshold:t}=e;const[n,l]=(0,a.useState)(!1),o=(0,a.useRef)(!1),{startScroll:r,cancelScroll:c}=(0,b.gk)();return(0,b.Mq)(((e,n)=>{let{scrollY:a}=e;const r=n?.scrollY;r&&(o.current?o.current=!1:a>=r?(c(),l(!1)):a<t?l(!1):a+window.innerHeight<document.documentElement.scrollHeight&&l(!0))})),(0,p.$)((e=>{e.location.hash&&(o.current=!0,l(!1))})),{shown:n,scrollToTop:()=>r(0)}}({threshold:300});return a.createElement("button",{"aria-label":(0,u.T)({id:"theme.BackToTopButton.buttonAriaLabel",message:"Scroll back to top",description:"The ARIA label for the back to top button"}),className:(0,l.A)("clean-btn",i.G.common.backToTopButton,h.backToTopButton,e&&h.backToTopButtonShow),type:"button",onClick:t})}var f=n(6347),g=n(4581),v=n(6342),_=n(3465),C=n(8168);function k(e){return a.createElement("svg",(0,C.A)({width:"20",height:"20","aria-hidden":"true"},e),a.createElement("g",{fill:"#7a7a7a"},a.createElement("path",{d:"M9.992 10.023c0 .2-.062.399-.172.547l-4.996 7.492a.982.982 0 01-.828.454H1c-.55 0-1-.453-1-1 0-.2.059-.403.168-.551l4.629-6.942L.168 3.078A.939.939 0 010 2.528c0-.548.45-.997 1-.997h2.996c.352 0 .649.18.828.45L9.82 9.472c.11.148.172.347.172.55zm0 0"}),a.createElement("path",{d:"M19.98 10.023c0 .2-.058.399-.168.547l-4.996 7.492a.987.987 0 01-.828.454h-3c-.547 0-.996-.453-.996-1 0-.2.059-.403.168-.551l4.625-6.942-4.625-6.945a.939.939 0 01-.168-.55 1 1 0 01.996-.997h3c.348 0 .649.18.828.45l4.996 7.492c.11.148.168.347.168.55zm0 0"})))}const A="collapseSidebarButton_PEFL",S="collapseSidebarButtonIcon_kv0_";function N(e){let{onClick:t}=e;return a.createElement("button",{type:"button",title:(0,u.T)({id:"theme.docs.sidebar.collapseButtonTitle",message:"Collapse sidebar",description:"The title attribute for collapse button of doc sidebar"}),"aria-label":(0,u.T)({id:"theme.docs.sidebar.collapseButtonAriaLabel",message:"Collapse sidebar",description:"The title attribute for collapse button of doc sidebar"}),className:(0,l.A)("button button--secondary button--outline",A),onClick:t},a.createElement(k,{className:S}))}var T=n(5041),I=n(9532);const x=Symbol("EmptyContext"),y=a.createContext(x);function w(e){let{children:t}=e;const[n,l]=(0,a.useState)(null),o=(0,a.useMemo)((()=>({expandedItem:n,setExpandedItem:l})),[n]);return a.createElement(y.Provider,{value:o},t)}var B=n(9169),M=n(1422),L=n(5489),P=n(2303);function G(e){let{categoryLabel:t,onClick:n}=e;return a.createElement("button",{"aria-label":(0,u.T)({id:"theme.DocSidebarItem.toggleCollapsedCategoryAriaLabel",message:"Toggle the collapsible sidebar category '{label}'",description:"The ARIA label to toggle the collapsible sidebar category"},{label:t}),type:"button",className:"clean-btn menu__caret",onClick:n})}function H(e){let{item:t,onItemClick:n,activePath:r,level:c,index:s,...d}=e;const{items:m,label:u,collapsible:b,className:p,href:h}=t,{docs:{sidebar:{autoCollapseCategories:E}}}=(0,v.p)(),f=function(e){const t=(0,P.A)();return(0,a.useMemo)((()=>e.href?e.href:!t&&e.collapsible?(0,o._o)(e):void 0),[e,t])}(t),g=(0,o.w8)(t,r),_=(0,B.ys)(h,r),{collapsed:k,setCollapsed:A}=(0,M.u)({initialState:()=>!!b&&(!g&&t.collapsed)}),{expandedItem:S,setExpandedItem:N}=function(){const e=(0,a.useContext)(y);if(e===x)throw new I.dV("DocSidebarItemsExpandedStateProvider");return e}(),T=function(e){void 0===e&&(e=!k),N(e?null:s),A(e)};return function(e){let{isActive:t,collapsed:n,updateCollapsed:l}=e;const o=(0,I.ZC)(t);(0,a.useEffect)((()=>{t&&!o&&n&&l(!1)}),[t,o,n,l])}({isActive:g,collapsed:k,updateCollapsed:T}),(0,a.useEffect)((()=>{b&&S&&S!==s&&E&&A(!0)}),[b,S,s,A,E]),a.createElement("li",{className:(0,l.A)(i.G.docs.docSidebarItemCategory,i.G.docs.docSidebarItemCategoryLevel(c),"menu__list-item",{"menu__list-item--collapsed":k},p)},a.createElement("div",{className:(0,l.A)("menu__list-item-collapsible",{"menu__list-item-collapsible--active":_})},a.createElement(L.A,(0,C.A)({className:(0,l.A)("menu__link",{"menu__link--sublist":b,"menu__link--sublist-caret":!h&&b,"menu__link--active":g}),onClick:b?e=>{n?.(t),h?T(!1):(e.preventDefault(),T())}:()=>{n?.(t)},"aria-current":_?"page":void 0,"aria-expanded":b?!k:void 0,href:b?f??"#":f},d),u),h&&b&&a.createElement(G,{categoryLabel:u,onClick:e=>{e.preventDefault(),T()}})),a.createElement(M.N,{lazy:!0,as:"ul",className:"menu__list",collapsed:k},a.createElement(K,{items:m,tabIndex:k?-1:0,onItemClick:n,activePath:r,level:c+1})))}var F=n(6654),W=n(2523);const D="menuExternalLink_NmtK";function z(e){let{item:t,onItemClick:n,activePath:r,level:c,index:s,...d}=e;const{href:m,label:u,className:b}=t,p=(0,o.w8)(t,r),h=(0,F.A)(m);return a.createElement("li",{className:(0,l.A)(i.G.docs.docSidebarItemLink,i.G.docs.docSidebarItemLinkLevel(c),"menu__list-item",b),key:u},a.createElement(L.A,(0,C.A)({className:(0,l.A)("menu__link",!h&&D,{"menu__link--active":p}),"aria-current":p?"page":void 0,to:m},h&&{onClick:n?()=>n(t):void 0},d),u,!h&&a.createElement(W.A,null)))}const R="menuHtmlItem_M9Kj";function U(e){let{item:t,level:n,index:o}=e;const{value:r,defaultStyle:c,className:s}=t;return a.createElement("li",{className:(0,l.A)(i.G.docs.docSidebarItemLink,i.G.docs.docSidebarItemLinkLevel(n),c&&[R,"menu__list-item"],s),key:o,dangerouslySetInnerHTML:{__html:r}})}function V(e){let{item:t,...n}=e;switch(t.type){case"category":return a.createElement(H,(0,C.A)({item:t},n));case"html":return a.createElement(U,(0,C.A)({item:t},n));default:return a.createElement(z,(0,C.A)({item:t},n))}}function j(e){let{items:t,...n}=e;return a.createElement(w,null,t.map(((e,t)=>a.createElement(V,(0,C.A)({key:t,item:e,index:t},n)))))}const K=(0,a.memo)(j),q="menu_SIkG",Y="menuWithAnnouncementBar_GW3s";function O(e){let{path:t,sidebar:n,className:o}=e;const r=function(){const{isActive:e}=(0,T.Mj)(),[t,n]=(0,a.useState)(e);return(0,b.Mq)((t=>{let{scrollY:a}=t;e&&n(0===a)}),[e]),e&&t}();return a.createElement("nav",{className:(0,l.A)("menu thin-scrollbar",q,r&&Y,o)},a.createElement("ul",{className:(0,l.A)(i.G.docs.docSidebarMenu,"menu__list")},a.createElement(K,{items:n,activePath:t,level:1})))}const X="sidebar_njMd",Z="sidebarWithHideableNavbar_wUlq",$="sidebarHidden_VK0M",J="sidebarLogo_isFc";function Q(e){let{path:t,sidebar:n,onCollapse:o,isHidden:r}=e;const{navbar:{hideOnScroll:c},docs:{sidebar:{hideable:i}}}=(0,v.p)();return a.createElement("div",{className:(0,l.A)(X,c&&Z,r&&$)},c&&a.createElement(_.A,{tabIndex:-1,className:J}),a.createElement(O,{path:t,sidebar:n}),i&&a.createElement(N,{onClick:o}))}const ee=a.memo(Q);var te=n(9876),ne=n(5600);const ae=e=>{let{sidebar:t,path:n}=e;const o=(0,te.M)();return a.createElement("ul",{className:(0,l.A)(i.G.docs.docSidebarMenu,"menu__list")},a.createElement(K,{items:t,activePath:n,onItemClick:e=>{"category"===e.type&&e.href&&o.toggle(),"link"===e.type&&o.toggle()},level:1}))};function le(e){return a.createElement(ne.GX,{component:ae,props:e})}const oe=a.memo(le);function re(e){const t=(0,g.l)(),n="desktop"===t||"ssr"===t,l="mobile"===t;return a.createElement(a.Fragment,null,n&&a.createElement(ee,e),l&&a.createElement(oe,e))}const ce={expandButton:"expandButton_m80_",expandButtonIcon:"expandButtonIcon_BlDH"};function ie(e){let{toggleSidebar:t}=e;return a.createElement("div",{className:ce.expandButton,title:(0,u.T)({id:"theme.docs.sidebar.expandButtonTitle",message:"Expand sidebar",description:"The ARIA label and title attribute for expand button of doc sidebar"}),"aria-label":(0,u.T)({id:"theme.docs.sidebar.expandButtonAriaLabel",message:"Expand sidebar",description:"The ARIA label and title attribute for expand button of doc sidebar"}),tabIndex:0,role:"button",onKeyDown:t,onClick:t},a.createElement(k,{className:ce.expandButtonIcon}))}const se={docSidebarContainer:"docSidebarContainer_b6E3",docSidebarContainerHidden:"docSidebarContainerHidden_b3ry"};function de(e){let{children:t}=e;const n=(0,d.t)();return a.createElement(a.Fragment,{key:n?.name??"noSidebar"},t)}function me(e){let{sidebar:t,hiddenSidebarContainer:n,setHiddenSidebarContainer:o}=e;const{pathname:r}=(0,f.zy)(),[c,s]=(0,a.useState)(!1),d=(0,a.useCallback)((()=>{c&&s(!1),o((e=>!e))}),[o,c]);return a.createElement("aside",{className:(0,l.A)(i.G.docs.docSidebarContainer,se.docSidebarContainer,n&&se.docSidebarContainerHidden),onTransitionEnd:e=>{e.currentTarget.classList.contains(se.docSidebarContainer)&&n&&s(!0)}},a.createElement(de,null,a.createElement(re,{sidebar:t,path:r,onCollapse:d,isHidden:c})),c&&a.createElement(ie,{toggleSidebar:d}))}const ue={docMainContainer:"docMainContainer_gTbr",docMainContainerEnhanced:"docMainContainerEnhanced_Uz_u",docItemWrapperEnhanced:"docItemWrapperEnhanced_czyv"};function be(e){let{hiddenSidebarContainer:t,children:n}=e;const o=(0,d.t)();return a.createElement("main",{className:(0,l.A)(ue.docMainContainer,(t||!o)&&ue.docMainContainerEnhanced)},a.createElement("div",{className:(0,l.A)("container padding-top--md padding-bottom--lg",ue.docItemWrapper,t&&ue.docItemWrapperEnhanced)},n))}const pe={docPage:"docPage__5DB",docsWrapper:"docsWrapper_BCFX"};function he(e){let{children:t}=e;const n=(0,d.t)(),[l,o]=(0,a.useState)(!1);return a.createElement(m.A,{wrapperClassName:pe.docsWrapper},a.createElement(E,null),a.createElement("div",{className:pe.docPage},n&&a.createElement(me,{sidebar:n.items,hiddenSidebarContainer:l,setHiddenSidebarContainer:o}),a.createElement(be,{hiddenSidebarContainer:l},t)))}var Ee=n(1774),fe=n(1463);function ge(e){const{versionMetadata:t}=e,n=(0,o.mz)(e);if(!n)return a.createElement(Ee.default,null);const{docElement:m,sidebarName:u,sidebarItems:b}=n;return a.createElement(a.Fragment,null,a.createElement(fe.A,{version:t.version,tag:(0,r.tU)(t.pluginId,t.version)}),a.createElement(c.e3,{className:(0,l.A)(i.G.wrapper.docsPages,i.G.page.docsDocPage,e.versionMetadata.className)},a.createElement(s.n,{version:t},a.createElement(d.V,{name:u,items:b},a.createElement(he,null,m)))))}},1774:(e,t,n)=>{n.r(t),n.d(t,{default:()=>c});var a=n(6540),l=n(1312),o=n(1003),r=n(6358);function c(){return a.createElement(a.Fragment,null,a.createElement(o.be,{title:(0,l.T)({id:"theme.NotFound.title",message:"Page Not Found"})}),a.createElement(r.A,null,a.createElement("main",{className:"container margin-vert--xl"},a.createElement("div",{className:"row"},a.createElement("div",{className:"col col--6 col--offset-3"},a.createElement("h1",{className:"hero__title"},a.createElement(l.A,{id:"theme.NotFound.title",description:"The title of the 404 page"},"Page Not Found")),a.createElement("p",null,a.createElement(l.A,{id:"theme.NotFound.p1",description:"The first paragraph of the 404 page"},"We could not find what you were looking for.")),a.createElement("p",null,a.createElement(l.A,{id:"theme.NotFound.p2",description:"The 2nd paragraph of the 404 page"},"Please contact the owner of the site that linked you to the original URL and let them know their link is broken.")))))))}},2252:(e,t,n)=>{n.d(t,{n:()=>r,r:()=>c});var a=n(6540),l=n(9532);const o=a.createContext(null);function r(e){let{children:t,version:n}=e;return a.createElement(o.Provider,{value:n},t)}function c(){const e=(0,a.useContext)(o);if(null===e)throw new l.dV("DocsVersionProvider");return e}}}]);