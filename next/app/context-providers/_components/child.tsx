import { useContext } from "react";

import { CxParent, TailwindClasses } from "./common";

export default function Child({
  className: cls0,
}: {
  className: TailwindClasses;
}) {
  const setParentCls = useContext(CxParent);

  const [, color] = cls0.split("-");

  return (
    <>
      <p>
        Lorem ipsum dolor sit amet, consectetur adipisci elit, sed eiusmod
        tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea
        commodi consequat. Quis aute iure reprehenderit in voluptate velit esse
        cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat
        cupiditat non provident, sunt in culpa qui official deserunt mollit anim
        id est laborum.
      </p>

      <button className={cls0} onClick={() => setParentCls(cls0)}>
        set bg of parent to {color}
      </button>
    </>
  );
}
