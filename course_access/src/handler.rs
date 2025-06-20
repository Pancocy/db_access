use actix_web::{HttpResponse, web};
use chrono::NaiveDate;
use super::states::*;

#[allow(warnings, unused)]
pub async fn  health_check(app_state: web::Data<AppState>)-> HttpResponse{
    let status = &app_state.health_status;
    let mut count = app_state.visite_count.lock().unwrap();

    let response = format!("第{}次访问,{}",count,status);
    *count += 1;

   HttpResponse::Ok().json(&response)
}

//获取所有课程
#[allow(warnings, unused)]
pub async fn get_all_course(app_state: web::Data<AppState>)->HttpResponse{
    let db = &app_state.db;
    let rows = sqlx::query!(
        r#"select * from course"#
    ).fetch_all(db).await.unwrap();

    let result:Vec<_> = rows.iter().map(|row| Course{
        id:row.id,
        teacher_id:row.teacher_id.unwrap(),
        course_name:row.name.clone().unwrap(),
        date:Some(NaiveDate::from(row.date.unwrap()))
    }).collect();

    HttpResponse::Ok().json(result)
}

//通过课程Id查询对应课程
#[allow(warnings, unused)]
pub async fn get_course_by_id(app_state: web::Data<AppState>,params:web::Path<(i32,)>) -> HttpResponse{
    let db = &app_state.db;
    let (cid,) = params.into_inner();

    let rows = sqlx::query_as!(
        Record,
        r#" select id,teacher_id,name,date from course where id = $1 "#,
        cid
    ).fetch_all(db).await.unwrap();

    HttpResponse::Ok().json(rows)
}
//查询某位老师的所有kecheng
pub async fn get_course_by_tid(app_state: web::Data<AppState>,params: web::Path<(i32,)>)->HttpResponse{
    let db = &app_state.db;
    let (tid,) = params.into_inner();

    let rows = sqlx::query_as!(
        Record,
        r#"select id,teacher_id,name,date from course where teacher_id = $1 "#,
        tid
    ).fetch_all(db).await.unwrap();

    HttpResponse::Ok().json(rows)
}
//查询某位老师的某个具体课程
pub async fn get_course_by_t_cid(app_state:web::Data<AppState>,params:web::Path<(i32,i32)>) -> HttpResponse{
    let db = &app_state.db;
    let (tid,cid) = params.into_inner();

    let rows = sqlx::query_as!(
        Record,
        r#"select id,teacher_id,name,date from course where teacher_id = $1 and id = $2"#,
        tid,
        cid
    ).fetch_all(db).await.unwrap();

    HttpResponse::Ok().json(rows)
}
//向表中添加数据
pub async fn post_course_into_table(app_state: web::Data<AppState>, payload: web::Json<Vec<Course>>) -> HttpResponse{

        let db = &app_state.db;
        let mut InsertResults:Vec<InsertStatus> = Vec::new();

        for course in payload.into_inner() {
            let result = sqlx::query!(
                "INSERT INTO course (id, teacher_id, name, date) VALUES ($1, $2, $3, $4)",
                course.id,
                course.teacher_id,
                course.course_name.clone(),
                course.date.unwrap()
            )
                .execute(db)
                .await;

            //println!("Result:{:?}",result)
            match result {
                Ok(_) => InsertResults.push(InsertStatus{
                    status:"success".to_string(),
                    course_name:course.course_name
                }),
                Err(err) => InsertResults.push(InsertStatus{
                    status: format!("error: {}", err),
                    course_name:course.course_name,
                })
            }
        }
        HttpResponse::Ok().json(InsertResults)
    }

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    async fn get_all_course(){

    }
}
