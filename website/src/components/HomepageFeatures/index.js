import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'One Workspace, Many Experiences',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        <code>jafa init</code> scaffolds a shared Rojo/Wally/Rokit workspace
        once, then <code>jafa new</code> adds as many Roblox experiences as
        you need inside it — no re-wiring tooling per project.
      </>
    ),
  },
  {
    title: 'Consistent Project Structure',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Every project gets the same <code>client</code>/<code>server</code>/
        <code>shared</code>/<code>ui</code> layout, wired into a Rojo tree
        automatically, so any project in the repo feels familiar.
      </>
    ),
  },
  {
    title: 'Build, Open, Serve — One Command',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        <code>jafa dev</code> builds a project, opens it in Roblox Studio, and
        starts <code>rojo serve</code> for live-sync, in one step.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
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
