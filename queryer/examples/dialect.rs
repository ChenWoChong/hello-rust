use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "SELECT a a1, b, 123, myfunc(b), * \
    FROM DATA_SOURCE \
    WHERE A>B AND B<100 AND C BETWEEN 10 AND 20 \
    ORDER BY A DESC, B \
    LIMIT 50 OFFSET 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}
