// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const { themes } = require('prism-react-renderer');
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Quick Test CLI',
  tagline: 'CLI for stress testing in competitive programming contest',
  url: 'https://luchobazz.github.io',
  baseUrl: '/quicktest/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.ico',
  organizationName: 'LuchoBazz',
  projectName: 'quicktest',

  themes: [
    [
      require.resolve("@easyops-cn/docusaurus-search-local"),
      ({
        hashed: true,
        language: ['en', 'es']
      }),
    ],
  ],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/LuchoBazz/quicktest/tree/main/website/',
        },
        blog: {
          showReadingTime: true,
          editUrl: 'https://github.com/LuchoBazz/quicktest/tree/main/website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: 'Quick Test CLI',
        logo: {
          alt: 'Quick Test CLI Logo',
          src: 'https://raw.githubusercontent.com/LuchoBazz/quicktest/main/assets/logo/quicktest-512x512.png',
        },
        items: [
          {
            type: 'doc',
            docId: 'intro',
            position: 'left',
            label: 'Docs',
          },
          { to: '/blog', label: 'Blog', position: 'left' },
          {
            href: 'https://github.com/LuchoBazz/quicktest',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Docs',
                to: '/docs/intro',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Stack Overflow',
                href: 'https://stackoverflow.com/questions/tagged/quicktest',
              },
              {
                label: 'Discord',
                href: 'https://discordapp.com/invite/quicktest',
              },
              {
                label: 'Twitter',
                href: 'https://twitter.com/quicktest',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                to: '/blog',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/LuchoBazz/quicktest',
              },
            ],
          },
        ],
        copyright: `Copyright © 2021-${new Date().getFullYear()} Quick Test CLI, Inc. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
