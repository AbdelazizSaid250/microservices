use actix_web::web::Json;
use crate::error::error::ServerErrorResponse;
use crate::model::dto::generic_dto::SuccessResponse;
use crate::model::dto::student::Student;

pub(crate) async fn welcome_student_api()
    -> Result<Json<SuccessResponse<Student>>, ServerErrorResponse> {

    // Initialize new student.
    let student = Student::new();
    println!("{:#?}", student);

    // Fire the Response.
    Ok(Json(SuccessResponse {
        code: 200,
        message: "Success".to_string(),
        data: Some(student),
    }))
}