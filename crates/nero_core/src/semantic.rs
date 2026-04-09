use std::collections::HashSet;

use crate::{
    ast::{Req, Stmt},
    resolver::{Resolver, Value},
};

pub struct SemanticChecker<'a> {
    pub resolver: &'a Resolver,
    pub labels: std::collections::HashSet<String>,
}

#[derive(Debug)]
pub enum SemanticError {
    BodyNotAllowed(String),
    HeaderValueMustBeString,
    UrlMustBeString,
    DuplicateLabel(String),
}

impl std::error::Error for SemanticError {}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BodyNotAllowed(_) => write!(f, "Body not allowed"),
            Self::DuplicateLabel(l) => write!(f, "Duplicate label: {}", l),
            Self::HeaderValueMustBeString => write!(f, "Headers value must be string"),
            Self::UrlMustBeString => write!(f, "Url must be string"),
        }
    }
}

impl<'a> SemanticChecker<'a> {
    pub fn new(resolver: &'a Resolver) -> Self {
        Self {
            resolver,
            labels: HashSet::new(),
        }
    }

    fn check_request(&mut self, req: &Req) -> Result<(), SemanticError> {
        // cek label tetap unik/tidak duplikat
        let label = &req.label;
        // if let Some(label) = &req.label {
        if !self.labels.insert(label.clone()) {
            return Err(SemanticError::DuplicateLabel(label.clone()));
        }
        // }

        // cek url harus string
        match self.resolver.resolve_expression(&req.url).unwrap() {
            Value::String(_) => {}
            _ => return Err(SemanticError::UrlMustBeString),
        }

        // cek tidak boleh ada body di get/delete
        if matches!(req.method.as_str(), "GET" | "DELETE") && req.body.is_some() {
            return Err(SemanticError::BodyNotAllowed(req.method.clone()));
        }

        // cek header value harus string
        for (_, expr) in &req.headers {
            let v = self.resolver.resolve_expression(expr).unwrap();
            if !matches!(v, Value::String(_)) {
                return Err(SemanticError::HeaderValueMustBeString);
            }
        }

        Ok(())
    }

    pub fn check_statement(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::Request(req) => self.check_request(req),
            _ => Ok(()),
        }
    }
}
