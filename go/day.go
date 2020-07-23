package main

import (
	"database/sql"
	"fmt"
	"time"

	_ "github.com/mattn/go-sqlite3"
)
func main() {
	//打开数据库，如果不存在，则创建
	db, err := sql.Open("sqlite3", "./foo.db")
	checkErr(err)

	//创建表
	sqlTable := `
    CREATE TABLE IF NOT EXISTS userinfo(
        uid INTEGER PRIMARY KEY AUTOINCREMENT,
        username VARCHAR(64) NULL,
        departname VARCHAR(64) NULL,
        created DATE NULL
    );
    `
	sqlTable1:=`CREATE TABLE IF NOT EXISTS userinfo(
		date INTEGER,
		open INTEGER,
		high INTEGER,
		low INTEGER,
		close INTEGER ,
		amount INTEGER ,
		vol INTEGER ,
		reservation INTEGER 
	);`
	db.Exec(sqlTable1)
	db.Exec(sqlTable)
	/*
	   date:u32, //integer
	   open:u32,
	   high:u32,
	   low:u32,
	   close:u32,
	   amount:u32,
	   vol:u32,
	   reservation:u32
	*/
	// insert
	stmt, err := db.Prepare("INSERT INTO userinfo(username, departname, created) values(?,?,?)")
	checkErr(err)

	res, err := stmt.Exec("wangshubo", "国务院", "2017-04-21")
	checkErr(err)

	id, err := res.LastInsertId()
	checkErr(err)

	fmt.Println(id)

	// update
	stmt, err = db.Prepare("update userinfo set username=? where uid=?")
	checkErr(err)

	res, err = stmt.Exec("wangshubo_new", id)
	checkErr(err)

	affect, err := res.RowsAffected()
	checkErr(err)

	fmt.Println(affect)

	// query
	rows, err := db.Query("SELECT * FROM userinfo")
	checkErr(err)
	var uid int
	var username string
	var department string
	var created time.Time

	for rows.Next() {
		err = rows.Scan(&uid, &username, &department, &created)
		checkErr(err)
		fmt.Println(uid)
		fmt.Println(username)
		fmt.Println(department)
		fmt.Println(created)
	}

	rows.Close()

	// delete
	stmt, err = db.Prepare("delete from userinfo where uid=?")
	checkErr(err)

	res, err = stmt.Exec(id)
	checkErr(err)

	affect, err = res.RowsAffected()
	checkErr(err)

	fmt.Println(affect)

	db.Close()

}

func checkErr(err error) {
	if err != nil {
		panic(err)
	}
}
