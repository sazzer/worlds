use std::collections::HashMap;

use actix_web::{
    error::ResponseError,
    http::{header, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;
use serde_json::Value;

use super::Problem;

/// HTTP representation of an RFC-7807 Problem response.
#[derive(Serialize)]
struct ProblemModel {
    /// The type code for the problem
    pub r#type:   String,
    /// The title string for the problem
    pub title:    String,
    /// The HTTP Status code to use
    pub status:   u16,
    /// An additional detail message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail:   Option<String>,
    /// An additional instance subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Any extra details
    #[serde(flatten)]
    pub extra:    HashMap<String, Value>,
}

impl From<&Problem> for HttpResponse {
    fn from(problem: &Problem) -> Self {
        let body = ProblemModel {
            r#type:   problem.error.problem_type().to_owned(),
            title:    problem.error.to_string(),
            status:   problem.status.as_u16(),
            detail:   problem.detail.clone(),
            instance: problem.instance.clone(),
            extra:    problem.extra.clone(),
        };

        Self::build(problem.status)
            .append_header((header::CONTENT_TYPE, "application/problem+json"))
            .json(body)
    }
}

impl From<Problem> for HttpResponse {
    fn from(problem: Problem) -> Self {
        Self::from(&problem)
    }
}

impl Responder for Problem {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        self.into()
    }
}

impl ResponseError for Problem {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        self.into()
    }
}
