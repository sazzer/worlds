import React from "react";
import { useTranslation } from "react-i18next";

export const App: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="App">
      <header className="App-header">{t("page.title")}</header>
    </div>
  );
};
