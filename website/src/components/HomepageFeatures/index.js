import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';
import useBaseUrl from "@docusaurus/useBaseUrl";

const FeatureList = [
  {
    title: 'quicktest cmp',
    imageUrl: '/gif/cmp.gif',
    redirect: '/docs/usage/cmp#demo',
    description: (
      <>
        Verify that the code does not have incorrect answers for some test cases, using a random generator and a slower version which is sure what is correct with which the answers will be compared.
      </>
    ),
  },
  {
    title: 'quicktest stress',
    imageUrl: '/gif/stress.gif',
    redirect: '/docs/usage/stress#demo',
    description: (
      <>
        Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.
      </>
    ),
  },
  {
    title: 'quicktest check',
    imageUrl: '/gif/check.gif',
    redirect: '/docs/usage/check#demo',
    description: (
      <>
        Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.
      </>
    ),
  },
  {
    title: 'quicktest output',
    imageUrl: '/gif/output.gif',
    redirect: '/docs/usage/output#demo',
    description: (
      <>
        Run a target file with test case files matching a prefix
      </>
    ),
  },
  {
    title: 'quicktest setup',
    imageUrl: '/gif/setup.gif',
    redirect: '/docs/usage/setup#demo',
    description: (
      <>
        Subcommand that allows to change settings
      </>
    ),
  },
  {
    title: 'quicktest example',
    imageUrl: '/gif/example.gif',
    redirect: '/docs/usage/example#demo',
    description: (
      <>
        Shows examples of the selected subcommand
      </>
    ),
  },
];

function Feature({imageUrl, redirect, title, description}) {
  const imgUrl = useBaseUrl(imageUrl);
  const to = useBaseUrl(redirect);

  return (
    <div className={clsx('col col--4')}>
      {imgUrl && (
        <div className={clsx("text--center")}>
          <a href={to}>
            <img className={styles.gif} src={imgUrl} alt={title} />
          </a>
        </div>
      )}
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
