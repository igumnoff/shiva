use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use crate::ctx::Ctx;


//Определяет маршруты для действия с билетами
pub fn routes(mc: ModelController) -> Router {
    Router::new() //Создает новый экземпляр маршрута
        .route("/tickets", //Путь по которому будет обрабатываться запрос
               post(create_ticket) //Создает обработчик для HTTP-метода POST и принимает в качестве аргумента функцию-обработчик, которая будет вызвана при получении POST-запроса
                   .get(list_tickets)) //Создает обработчик для HTTP-метода GET и принимает в качестве аргумента функцию-обработчик, которая будет вызвана при получении GET-запроса
        .route("/tickets/:id", //Путь по которому будет обрабатываться запрос
               delete(delete_ticket)) //Создает обработчик для HTTP-метода DELETE и принимает в качестве аргумента функцию-обработчик, которая будет вызвана при получении DELETE-запроса
        .with_state(mc) //Добавляет состояние mc (контроллер моделей) в Router, чтобы оно было доступно для обработчиков
}


// region:    ---REST Handlers
//Создает билет
async fn create_ticket(
    State(mc): State<ModelController>, //Извлекает состояние ModelController из Router с помощью State
    ctx: Ctx, //Контекст запроса, который был определен ранее и передается в функцию
    Json(ticket_fc): Json<TicketForCreate>, //Десериализует данные запроса в формате JSON в структуру TicketForCreate
) -> Result<Json<Ticket>> { //Возвращает JSON-ответ с информацией о созданном билете
    println!("-->> {:<12} - create_ticket", "HANDLER"); //Отладочный вывод, который показывает, что выполняется обработчик create_ticket

    let ticket = mc.create_ticket( //Вызывает метод create_ticket контроллера моделей (mc) для создания нового билета
        ctx, //Контекст запроса
        ticket_fc) //Данные для создания билета (из структуры TicketForCreate)
        .await?; //Ожидает окончания выполнения функции создания билета

    Ok(Json(ticket)) //Возвращает успешный результат с JSON-ответом, содержащим информацию о созданном билете
}

//Создает список билетов
async fn list_tickets(
    State(ms): State<ModelController>, //Извлекает состояние ModelController из Router с помощью State
    ctx: Ctx, //Контекст запроса, который был определен ранее и передается в функцию
) -> Result<Json<Vec<Ticket>>> { //Возвращает JSON-ответ с информацией о всех имеющихся билетах (вектор билетов)
    println!("-->> {:<12} - list_tickets", "HANDLER"); //Отладочный вывод, который показывает, что выполняется обработчик list_tickets

    let tickets = ms.list_tickets(ctx) //Вызывает метод list_tickets контроллера моделей (mc) для создания списка билетов
        .await?; //Ожидает окончания выполнения функции создания списка билетов

    Ok(Json(tickets)) //Возвращает успешный результат с JSON-ответом, содержащим информацию о всех билетах (список билетов)
}

//Удаляет билет
async fn delete_ticket(
    State(ms): State<ModelController>, //Извлекает состояние ModelController из Router с помощью State
    ctx: Ctx, //Контекст запроса, который был определен ранее и передается в функцию
    Path(id): Path<u64>, //Идентификатор билета
) -> Result<Json<Ticket>> { //Возвращает JSON-ответ с информацией об удаленном билете
    println!("-->> {:<12} - delete_ticket", "HANDLER"); //Отладочный вывод, который показывает, что выполняется обработчик delete_ticket

    let ticket = ms.delete_ticket(ctx, id) //Вызывает метод delete_ticket контроллера моделей (mc) для удаления билета
        .await?; //Ожидает окончания выполнения функции удаления билета

    Ok(Json(ticket)) //Возвращает успешный результат с JSON-ответом, содержащим информацию об удаленном билете
}
// endregion: ---REST Handlers