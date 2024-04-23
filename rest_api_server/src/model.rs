///Упрощенный слой модели со встроенным слоем пакетного хранилища
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{Error, Result};

// region:   --- Ticket Types

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
//endregion: --- Ticket Types

//region:    ---Model Controller

//контроллер моделей используется для подключения к базе данных
#[derive(Clone)]
pub struct ModelController {
    pub tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

//Конструктор
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

//CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_fs: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fs.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailNotFound { id })
    }
}

//endregion: ---Model Controller
