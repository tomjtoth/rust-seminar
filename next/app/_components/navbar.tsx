import Link from "next/link";

const RE_REPLACER = /\W/g;

export default function NavBar() {
  return (
    <nav>
      <ul className="p-2 flex flex-wrap gap-2 *:border *:rounded **:p-2 *:text-nowrap">
        {[
          "counters",
          "controlled-input",
          "context-providers",
          "global-state",
          "callbacks",
          "fullstack",
        ].map((href) => (
          <li key={href}>
            <Link {...{ href }}>{href.replaceAll(RE_REPLACER, " ")}</Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}
