import { Link } from "react-router-dom";
import { LoginButton } from "./login";
import React from "react";
import { UserMenu } from "./usermenu";
import { useAuth0 } from "@auth0/auth0-react";
import { useTranslation } from "react-i18next";

export const HeaderBar: React.FC = () => {
  const { t } = useTranslation();
  const { isAuthenticated } = useAuth0();

  return (
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
      <div className="container-fluid">
        <Link className="navbar-brand" to="/">
          {t("page.title")}
        </Link>
        <button
          className="navbar-toggler"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#navbarSupportedContent"
          aria-controls="navbarSupportedContent"
          aria-expanded="false"
          aria-label={t("header.toggleNavigation")}
        >
          <span className="navbar-toggler-icon"></span>
        </button>
        <div className="collapse navbar-collapse" id="navbarSupportedContent">
          <ul className="navbar-nav ms-auto mb-2 mb-lg-0">
            {isAuthenticated ? <UserMenu /> : <LoginButton />}
          </ul>
        </div>
      </div>
    </nav>
  );
};
