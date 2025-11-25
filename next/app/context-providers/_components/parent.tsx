import { ReactNode, useState } from "react";

import { CxParent, TailwindClasses } from "./common";

export default function Parent({
  children,
  className: cls0,
}: {
  children: ReactNode;
  className: TailwindClasses;
}) {
  const [cls, setCls] = useState(cls0);

  const [, color] = cls0.split("-");

  return (
    <CxParent.Provider value={setCls}>
      <div className="flex p-2 gap-2">
        <button className={cls0} onClick={() => setCls(cls0)}>
          set bg of this to {color}
        </button>

        <div className={`border rounded p-2 ${cls} duration-500`}>
          {children}
        </div>
      </div>
    </CxParent.Provider>
  );
}
