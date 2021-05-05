import { HeaderBar } from "./header";
import React from "react";

export const App: React.FC = () => {
  return (
    <div>
      <HeaderBar />
      <div className="container-fluid"></div>
    </div>
  );
};
