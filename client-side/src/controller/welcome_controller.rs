use actix_web::web::Json;
use reqwest::{Client, StatusCode};
use crate::error::error::{ErrorResponse, ServerErrorResponse};
use crate::model::dto::generic_dto::SuccessResponse;
use crate::model::dto::student::Student;

pub(crate) async fn welcome_student_client_api()
    -> Result<Json<SuccessResponse<Student>>, ServerErrorResponse> {

    // Step 2: Create a get request with the features ids request and extract the payload from the response.
    match Client::new().get("http://localhost:3000/server/welcome")
        .send().await {
        Ok(response) => {
            println!("{}", response.status());
            if response.status() == StatusCode::OK {
                println!("Yes");
                match response.json::<SuccessResponse<Student>>().await {
                    Ok(student) => {
                        println!("after yes: {:?}", student.data);

                        // Fire the Response.
                        Ok(Json(SuccessResponse {
                            code: 200,
                            message: "Success".to_string(),
                            data: Some(student.data.unwrap()),
                        }))
                    }
                    Err(_) => Err(ServerErrorResponse::InternalServerError(ErrorResponse{
                        code: 500,
                        message: "Cannot Sent the Request".to_string(),
                        data: None,
                    }))
                }
            } else {
                println!("I am in else");
                Err(ServerErrorResponse::InternalServerError(ErrorResponse{
                    code: 500,
                    message: "Status Code is not OK".to_string(),
                    data: None,
                }))
            }
        }
        Err(_) => Err(ServerErrorResponse::InternalServerError(ErrorResponse{
            code: 500,
            message: "Cannot Sent the Request".to_string(),
            data: None,
        })),
    }
}