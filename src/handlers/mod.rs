use rocket::{delete, get, post, serde::json::Json};

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: question.title.to_owned(),
        description: question.description.to_owned(),
        created_at: "created_at".to_owned(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    // ...
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: answer.question_uuid.to_owned(),
        content: answer.content.to_owned(),
        created_at: "created_at".to_owned(),
    })
}

#[get("/answers", data = "<question_id>")]
pub async fn read_answers(question_id: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: question_id.question_uuid.to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

#[delete("/answer", data = "<answer_id>")]
pub async fn delete_answer(answer_id: Json<AnswerId>) {
    //...
}