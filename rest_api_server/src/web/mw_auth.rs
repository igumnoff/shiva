use async_trait::async_trait;
use crate::{Error, Result};
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;


//Проверяет, что контекст запроса (Ctx) существует
pub async fn mw_require_auth(
    ctx: Result<Ctx>, //Результат, содержащий контекст запроса Ctx
    req: Request<Body>, //Запрос, который приходит в middleware
    next: Next, //Следующий обработчик в цепочке middleware
) -> Result<Response> {
    println!("-->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE"); //Выводит отладочное сообщение, указывающее на начало выполнения промежуточного обработчика

    ctx?; //Оператор ? используется для распаковки Result. Если ctx содержит ошибку, она будет возвращена из метода. Если ctx содержит успешное значение (контекст запроса), выполнение продолжится

    Ok(next.run(req).await) //Вызывает следующий обработчик в цепочке middleware с помощью next.run(req).await
}



pub async fn mw_ctx_resolver(
    _mc: State<ModelController>, //Состояние контроллера моделей, которое в данном случае не используется
    cookies: Cookies, //Куки из запроса, содержащие токен авторизации
    mut req: Request<Body>, //Запрос, который приходит в middleware
    next: Next //Следующий обработчик в цепочке middleware
) -> Result<Response> {
    println!("-->> {:<12} - mw_ctx_resolver", "MIDDLEWARE"); //Отладочное сообщение, указывающее на начало выполнения промежуточного обработчика

    let auth_token = cookies.get(AUTH_TOKEN) //Извлекается токен авторизации из кукис
        .map(|c| c.value().to_string()); //Преобразование токена в строку

    let result_ctx  = match auth_token //обработка токена авторизации auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie) //Преобразует Option в Result, возвращая ошибку AuthFailNoAuthTokenCookie, если auth_token равен None
        .and_then(parse_token) //Вызывает функцию parse_token для разбора токена, если auth_token содержит значение
    {
        Ok((user_id, _exp, _sing)) => { //Если разбор токена прошел успешно и вернул кортеж с элементами (user_id, _exp, _sing), где _exp и _sing - это время и подпись токена, соответственно
            //TODO: проверка компонентов токена
            Ok(Ctx::new(user_id)) //Создается новый контекст запроса Ctx на основе user_id
        }
        Err(e) => Err(e), //Если разбор токена завершился ошибкой - возвращает ту же ошибку
    };

    //очистка файла куки
    if result_ctx.is_err() //Проверяет не является ли result_ctx ошибкой
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) //Проверяет, что ошибка не является Error::AuthFailNoAuthTokenCookie
    {
        cookies.remove(Cookie::from(AUTH_TOKEN)) //Если оба условия истинны, то удаляет куку с именем AUTH_TOKEN из объекта cookies
    }

    //сохранение ctx_result в расширении запросов
    req.extensions_mut() //Возвращает мутабельную ссылку на расширения запроса, позволяя вставить в них новое значение
        .insert(result_ctx); //Сохраняет result_ctx (который может содержать либо успешный контекст запроса, либо ошибку) в расширениях запроса

    Ok(next //next это объект, представляющий следующий обработчик в цепочке
        .run(req) //Вызывает этот следующий обработчик с текущим запросом req
        .await) //Ожидает завершения выполнения следующего обработчика
}


// region:    ---Ctx Extractor

#[async_trait] //Позволяет использовать асинхронные методы в реализации трейта

//объявление реализации для типа Ctx трейта FromRequestParts<S>, где S должен реализовывать трейты Send и Sync
//Send гарантирует, что тип можно безопасно передавать между потоками
//Sync гарантирует, что тип можно безопасно использовать в параллельных потоках для чтения
impl<S: Send + Sync> FromRequestParts<S> for Ctx {

    type Rejection = Error; //Ассоциированный тип, определяющий, какой тип ошибки будет возвращаться, если извлечение данных из запроса не удалось. В данном случае, это тип Error

    async fn from_request_parts(
        parts: &mut Parts, //Мутабельная ссылка на части запроса, из которых нужно извлечь контекст
        _state: &S //Ссылка на состояние приложения, которая в данном случае не используется
    ) -> Result<Self> { //Self - это тип Ctx
        println!("-->> {:<12} -Ctx", "EXTRACTOR"); //Отладочный вывод, который показывает, что выполняется экстрактор контекста запроса

       parts
           .extensions //метод, который позволяет получить доступ к расширениям запроса
           .get::<Result<Ctx>>() //Получает значение запроса
           .ok_or(Error::AuthFailCtxNotInRequestExt)? //Если предыдущий метод вернул None, то этот метод преобразует None в ошибку Error::AuthFailCtxNotInRequestExt
           .clone() //Если значение было успешно получено из расширений запроса, то оно клонируется
    }
}

// endregion: ---Ctx Extractor

///Парсинг (анализ) токена, его времени действия и подписи
//Return (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sing) = regex_captures!( //Используется для извлечения частей строки, соответствующих заданному регулярному выражению
        r#"^user-(\d+)\.(.+)\.(.+)"#, //Регулярное выражение
//^user-: Начало строки с префиксом "user-"
//(\d+): Захватывающая группа, которая извлекает идентификатор пользователя (целое число).
// \.: Точка, разделяющая части токена.
// (.+): Захватывающая группа, которая извлекает время истечения срока действия.
// \.: Еще одна точка, разделяющая части токена.
// (.+): Захватывающая группа, которая извлекает подпись.
        &token //Ссылка на входную строку token, которая будет сопоставлена с регулярным выражением
    )
        .ok_or(Error::AuthFailTokenWrongFormat)?; //Преобразует None в ошибку Error::AuthFailTokenWrongFormat, если regex_captures! возвращает None
//Если регулярное выражение успешно совпало с token, то его результат (кортеж) распаковывается в переменные _whole, user_id, exp и sing
// _whole содержит всю строку, соответствующую регулярному выражению
// user_id содержит идентификатор пользователя, извлеченный из первой захватывающей группы
// exp содержит время истечения срока действия, извлеченное из второй захватывающей группы
// sing содержит подпись, извлеченную из третьей захватывающей группы



    let user_id: u64 = user_id.parse() //Преобразовывает строковое значение user_id, извлеченное из токена, в целочисленное значение типа u64
        .map_err(|_| Error::AuthFailTokenWrongFormat)?; //Если parse() вернул ошибку, то map_err() создает новую ошибку типа Error::AuthFailTokenWrongFormat и возвращает ее
//|_| - это замыкание, которое игнорирует значение ошибки, возвращенное parse(), и всегда возвращает Error::AuthFailTokenWrongFormat

    Ok((user_id, exp.to_string(), sing.to_string())) //Возвращаем кортеж
}
