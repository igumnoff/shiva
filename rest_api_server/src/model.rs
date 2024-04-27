///Упрощенный слой модели со встроенным слоем пакетного хранилища
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{Error, Result};
use crate::ctx::Ctx;

// region:   --- Ticket Types


//Структура данных о билете
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64, //Идентификатор билета
    pub cid: u64, //Идентификатор создателя билета (user_id)
    pub title: String, //Название билета
}

//Структура создания нового билета
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String, //Название билета
}
//endregion: --- Ticket Types

//region:    ---Model Controller


//Структура контроллера моделей
#[derive(Clone)]
pub struct ModelController {
    pub tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, //Общее хранилище билетов
}

//Создание нового экземпляра хранилища билетов
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self { //Возвращение успешного результата (Ok) с новым экземпляром ModelController
            tickets_store: Arc::default(), //Инициализация поля tickets_store нового экземпляра ModelController.
            //Arc - это тип атомарного, ссылочного счетчика (Atomic Reference Counted), который позволяет безопасно передавать данные между потоками
            //::default() - это вызов статического метода default(), который возвращает новый экземпляр Arc<Mutex<Vec<Option<Ticket>>>>
        })
    }
}

//CRUD Implementation - создает реализацию методов create, read, update, delete
impl ModelController {

    //Создание билета
    pub async fn create_ticket(
        &self, //Ссылка на экземпляр ModelController
        ctx: Ctx, //Контекст запроса, содержащий информацию о пользователе
        ticket_fs: TicketForCreate, //Данные для создания нового билета
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap(); //Получает мьютех для доступа к хранилищу билетов. Используется lock().unwrap(), чтобы заблокировать мьютекс и получить мутабельную ссылку на хранилище

        let id = store.len() as u64; //Вычисляет уникальный идентификатор для нового билета, используя длину хранилища билетов
        let ticket = Ticket { //Создается новый объект Ticket с заполненными полями
            id, //id билета
            cid: ctx.user_id(), //Идентификатор пользователя, создавшего билет, полученный из контекста запроса
            title: ticket_fs.title, //Название билета
        };
        store.push(Some(ticket.clone())); //Новый билет добавляется в хранилище билетов store. Используется Some(ticket.clone()), чтобы обернуть билет в Option и создать копию билета.


        Ok(ticket) //Возвращается успешный результат с созданным билетом
    }


    //Список билетов

    pub async fn list_tickets(&self, //&self - ссылка на экземпляр ModelController
                              _ctx: Ctx //Не используемый (в данном случае) контекст
    ) -> Result<Vec<Ticket>> {

        let store = self.tickets_store.lock().unwrap(); //Получает мьютех для доступа к хранилищу билетов. Используется lock().unwrap(), чтобы заблокировать мьютекс и получить ссылку на хранилище


        let tickets = store.iter() //Получение итератора по хранилищу билетов
            .filter_map(|t| t.clone()) //Применяется filter_map(), чтобы пройти по всем элементам хранилища и проверить, что элемент не равен None (с помощью |t| t.clone()). Если элемент не None, то клонировать его и добавить в результирующий вектор.
            .collect(); //Собирает все клонированные билеты в новый вектор tickets.

        Ok(tickets) //Возвращается успешный результат с вектором всех билетов из хранилища
    }


    //Удаление билета
    pub async fn delete_ticket(&self, //&self - ссылка на экземпляр ModelController
                               _ctx: Ctx, //Не используемый (в данном случае) контекст
                               id: u64 //Идентификатор билета, который нужно удалить
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap(); //Получает мьютех для доступа к хранилищу билетов. Используется lock().unwrap(), чтобы заблокировать мьютекс и получить мутабельную ссылку на хранилище

        let ticket = store.get_mut(id as usize) //Пытает получить мутабельную ссылку на билет в хранилище по указанному идентификатору id.
            .and_then(|t| t.take()); //Если элемент не равен None, вызвать t.take() для извлечения (перемещения) значения билета из хранилища. Если None, то t.take() не вызывается

        ticket.ok_or(Error::TicketDeleteFailNotFound { id }) //Возвращает успешный результат с извлеченным билетом, если ticket не равно None. Если ticket равно None, то возвращается ошибка Error::TicketDeleteFailNotFound с идентификатором билета, который не был найден.

    }
}

//endregion: ---Model Controller
