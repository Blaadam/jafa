import {useCallback, useEffect, useRef, useState} from 'react';
import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const GameList = [
  {
    name: 'Super Sports',
    logo: '/img/supersports-logo.png',
    description:
      'A multi-sport Roblox experience. Jafa was originally built to manage its multi-place development.',
    links: [
      {label: 'Website', href: 'https://supersports.io/'},
      {label: 'Roblox', href: 'https://www.roblox.com/communities/677519632/Super-Sports'},
      {label: 'Discord', href: 'https://discord.gg/A9fejkQFzH'},
    ],
  },
];

function Game({name, logo, description, links}) {
  return (
    <div className={styles.card}>
      <img
        className={styles.logo}
        src={useBaseUrl(logo)}
        alt={`${name} logo`}
        loading="lazy"
      />
      <Heading as="h3" className={styles.name}>{name}</Heading>
      <p>{description}</p>
      <div className={styles.links}>
        {links.map(({label, href}) => (
          <Link
            key={label}
            className="button button--outline button--primary button--sm"
            href={href}>
            {label}
          </Link>
        ))}
      </div>
    </div>
  );
}

export default function HomepageShowcase() {
  const trackRef = useRef(null);
  const [canPrev, setCanPrev] = useState(false);
  const [canNext, setCanNext] = useState(false);

  const update = useCallback(() => {
    const el = trackRef.current;
    if (!el) return;
    setCanPrev(el.scrollLeft > 1);
    setCanNext(el.scrollLeft + el.clientWidth < el.scrollWidth - 1);
  }, []);

  useEffect(() => {
    update();
    window.addEventListener('resize', update);
    return () => window.removeEventListener('resize', update);
  }, [update]);

  const scrollByPage = (dir) => {
    const el = trackRef.current;
    el.scrollBy({left: dir * el.clientWidth, behavior: 'smooth'});
  };

  return (
    <section className={styles.showcase}>
      <div className="container">
        <Heading as="h2" className="text--center">
          Games built with Jafa
        </Heading>
        <div className={styles.carousel}>
          {(canPrev || canNext) && (
            <button
              type="button"
              className={styles.arrow}
              aria-label="Previous games"
              disabled={!canPrev}
              onClick={() => scrollByPage(-1)}>
              ‹
            </button>
          )}
          <div className={styles.track} ref={trackRef} onScroll={update}>
            {GameList.map((props) => (
              <Game key={props.name} {...props} />
            ))}
          </div>
          {(canPrev || canNext) && (
            <button
              type="button"
              className={styles.arrow}
              aria-label="Next games"
              disabled={!canNext}
              onClick={() => scrollByPage(1)}>
              ›
            </button>
          )}
        </div>
        <p className="text--center">
          Built something with Jafa?{' '}
          <Link href="https://github.com/blaadam/jafa">
            Open a PR to add your game.
          </Link>
        </p>
      </div>
    </section>
  );
}
