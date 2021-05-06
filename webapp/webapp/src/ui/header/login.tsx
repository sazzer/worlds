import React from "react";
import { useAuth0 } from "@auth0/auth0-react";

export const LoginButton: React.FC = () => {
  const { loginWithRedirect } = useAuth0();

  return (
    <li className="nav-link">
      <button
        className="btn btn-link nav-link"
        onClick={() => loginWithRedirect()}
      >
        Login
      </button>
    </li>
  );
};
