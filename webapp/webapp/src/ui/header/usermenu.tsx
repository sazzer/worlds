import React from "react";
import { useAuth0 } from "@auth0/auth0-react";

export const UserMenu: React.FC = () => {
  const { logout, user } = useAuth0();

  return (
    <li className="nav-item dropdown">
      <button
        className="nav-link dropdown-toggle btn btn-link"
        id="navbarDropdown"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        {user?.nickname}
      </button>
      <ul
        className="dropdown-menu dropdown-menu-dark dropdown-menu-end"
        aria-labelledby="navbarDropdown"
      >
        <li>
          <button
            className="btn btn-link nav-link dropdown-item"
            onClick={() => logout()}
          >
            Logout
          </button>
        </li>
      </ul>
    </li>
  );
};
