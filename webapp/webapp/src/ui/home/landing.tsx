import React from "react";
import landing from "./landing.png";

export const Landing: React.FC = ({ children }) => {
  return (
    <div className="row">
      <div className="col-12 col-lg-3 order-lg-3">{children}</div>
      <div className="col-12 col-lg-9">
        <div className="card border-1">
          <img
            src={landing}
            alt="Tanalaeth - Western Lands"
            className="card-img-top img-fluid"
          />
          <div className="card-body">
            <h5 className="card-title">Tanalaeth - Western Lands</h5>
            <p className="card-text">By 5HTRonin</p>
          </div>
        </div>
      </div>
    </div>
  );
};
