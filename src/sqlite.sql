begin
CREATE TABLE day_dealing if not exists day_dealing(
	number:INTEGER,
	date:INTEGER,
	open:INTEGER,
	high:INTEGER,
	low:INTEGER,
	close:INTEGER,
	amount:INTEGER,
	vol:INTEGER,
	reservation:INTEGER,
	constraint KeyName primary key (number,date),

)
end;

BEGIN
                CREATE TABLE day_dealing if not exists day_dealing (number integer,date integer,open integer,high integer,low integer,close integer,amount integer,vol integer,reservation integer, constraint keyname primary key (number,date));
END;