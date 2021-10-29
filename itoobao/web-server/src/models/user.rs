use crate::state::DbPool;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    #[sqlx(rename = "id")]
    pub uid: i32,
    #[sqlx(rename = "name")]
    pub username: String,
    #[sqlx(rename = "password")]
    pub pwd: String,
}

impl User {
    //增删改查
    pub async fn get_user(&self, pool: &DbPool) {
        let sql = "select * from i_admin_name where name=?";
        // let info = sqlx::query_as::<_, Self>(sql)
        //     .bind(&self.username)
        //     .fetch_one(&pool)
        //     .await;

        let u = sqlx::query_as::<_, Self>(sql)
            .bind(&self.username)
            .fetch_one(pool)
            .await;
        println!("{:#?}", u);
    }
}
