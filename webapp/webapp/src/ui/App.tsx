import { HeaderBar } from "./header";
import { HomePage } from "./home";
import React from "react";

export const App: React.FC = () => {
  return (
    <div>
      <HeaderBar />
      <div className="container-fluid mt-2">
        <HomePage />
      </div>
    </div>
  );
};
