use super::states::*;
use actix_web::{HttpResponse, web};
use chrono::NaiveDate;

#[allow(warnings, unused)]
pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let status = &app_state.health_status;
    let mut count = app_state.visite_count.lock().unwrap();

    let response = format!("第{}次访问,{}", count, status);
    *count += 1;

    HttpResponse::Ok().json(&response)
}

//获取所有课程
#[allow(warnings, unused)]
pub async fn get_all_course(app_state: web::Data<AppState>) -> HttpResponse {
    let db = &app_state.db;
    let rows = sqlx::query!(r#"select * from course"#)
        .fetch_all(db)
        .await
        .unwrap();

    let result: Vec<_> = rows
        .iter()
        .map(|row| Course {
            id: row.id,
            teacher_id: row.teacher_id.unwrap(),
            course_name: row.name.clone().unwrap(),
            date: Some(NaiveDate::from(row.date.unwrap())),
        })
        .collect();

    HttpResponse::Ok().json(result)
}

//通过课程Id查询对应课程
#[allow(warnings, unused)]
pub async fn get_course_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let db = &app_state.db;
    let (cid,) = params.into_inner();

    let rows = sqlx::query_as!(
        Record,
        r#" select id,teacher_id,name,date from course where id = $1 "#,
        cid
    )
    .fetch_all(db)
    .await
    .unwrap();

    HttpResponse::Ok().json(rows)
}
//查询某位老师的所有kecheng
pub async fn get_course_by_tid(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let db = &app_state.db;
    let (tid,) = params.into_inner();

    let rows = sqlx::query_as!(
        Record,
        r#"select id,teacher_id,name,date from course where teacher_id = $1 "#,
        tid
    )
    .fetch_all(db)
    .await
    .unwrap();

    HttpResponse::Ok().json(rows)
}
//查询某位老师的某个具体课程
pub async fn get_course_by_t_cid(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let db = &app_state.db;
    let (tid, cid) = params.into_inner();

    let rows = sqlx::query_as!(
        Record,
        r#"select id,teacher_id,name,date from course where teacher_id = $1 and id = $2"#,
        tid,
        cid
    )
    .fetch_all(db)
    .await
    .unwrap();

    HttpResponse::Ok().json(rows)
}
//向表中添加数据,已存在ID时为修改
pub async fn post_course_into_table(
    app_state: web::Data<AppState>,
    payload: web::Json<Vec<Course>>,
) -> HttpResponse {
    let db = &app_state.db;
    let mut insert_results: Vec<ExecuteStatus> = Vec::new();

    for course in payload.into_inner() {
        let result = sqlx::query!(
            r#"
                INSERT INTO course (id, teacher_id, name, date)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (id) DO UPDATE SET
                    teacher_id = EXCLUDED.teacher_id,
                    name = EXCLUDED.name,
                    date = EXCLUDED.date
                "#,
            course.id,
            course.teacher_id,
            course.course_name.clone(),
            course.date.unwrap()
        )
        .execute(db)
        .await;

        //println!("Result:{:?}",result)
        match result {
            Ok(_) => insert_results.push(ExecuteStatus {
                status: "success".to_string(),
                course_name: course.course_name,
            }),
            Err(err) => insert_results.push(ExecuteStatus {
                status: format!("error: {}", err),
                course_name: course.course_name,
            }),
        }
    }
    HttpResponse::Ok().json(insert_results)
}

//删除
pub async fn delete_course_by_cid(
    app_state: web::Data<AppState>,
    params: web::Json<Vec<i32>>,
) -> HttpResponse {
    let db = &app_state.db;
    let mut delete_results: Vec<ExecuteStatus> = Vec::new();
    for c_id in params.into_inner() {
        let result = sqlx::query!(
            r#"
                Delete FROM course where id = $1
                RETURNING id,name
            "#,
            c_id
        ).fetch_optional(db).await;
        match result {
            Ok(Some(row)) => delete_results.push(ExecuteStatus {
                status: "success".to_string(),
                course_name: row.name.unwrap(),
            }),
            Ok(None) => delete_results.push(ExecuteStatus {
                status: "课程不存在".to_string(),
                course_name: format!("课程id{}不存",c_id),
            }),
            Err(err) => delete_results.push(ExecuteStatus{
                status:"error".to_string(),
                course_name:format!("id为{}的课程,删除失败：{}",c_id,err)
            })
        }
    }
    HttpResponse::Ok().json(delete_results)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    async fn get_all_course() {}
}
