use crate::test2::infrastructure::{Charger, EmailSender, SmtpClient, StripeGateway};
use anyhow::{Result, anyhow};

// --- 模拟的底层设施 ---
pub mod infrastructure {
    // 一个非常具体的支付网关
    pub trait Charger {
        fn charge(&self, amount: f64, card_token: &str) -> bool;
    }
    // 一个非常具体的支付网关
    #[derive(Default)]
    pub struct StripeGateway;
    impl Charger for StripeGateway {
        fn charge(&self, amount: f64, card_token: &str) -> bool {
            println!("使用 Stripe 对 token '{}' 收费 ${:.2}", card_token, amount);
            // 真实世界中会有复杂的 API 调用
            amount > 0.0 && !card_token.is_empty()
        }
    }

    pub trait EmailSender {
        fn send_email(&self, recipient: &str, subject: &str, body: &str);
    }

    // 一个非常具体的邮件客户端
    pub struct SmtpClient;
    impl EmailSender for SmtpClient {
        fn send_email(&self, recipient: &str, subject: &str, body: &str) {
            println!("--- 发送邮件 ---");
            println!("收件人: {}", recipient);
            println!("主题: {}", subject);
            println!("正文: {}", body);
            println!("-----------------");
        }
    }
}

// --- 领域模型 ---
#[derive(Debug)]
pub enum CustomerTier {
    Regular,
    Premium,
}

impl CustomerTier {
    pub fn get_tier_price_calculator(&self) -> &dyn TierPriceCalculator {
        match self {
            CustomerTier::Regular => &RegularTier,
            CustomerTier::Premium => &PremiumTier,
        }
    }
}

pub trait TierPriceCalculator {
    fn calculate_price(&self, raw_price: f64) -> f64;
}

pub struct RegularTier;
impl TierPriceCalculator for RegularTier {
    fn calculate_price(&self, raw_price: f64) -> f64 {
        raw_price
    }
}

pub struct PremiumTier;
impl TierPriceCalculator for PremiumTier {
    fn calculate_price(&self, raw_price: f64) -> f64 {
        raw_price * 0.8
    }
}

#[derive(Debug)]
pub enum PaymentMethod {
    CreditCard { token: String },
    PayPal { email: String },
}

// pub struct OrderPayment<'a> {
//     pub payment: &'a dyn Payment,
// }

pub trait Payment {
    fn pay(&self, price: f64) -> bool;
}

// pub fn new_payment<'a>(order: &Order, charger: &'a dyn Charger) -> Box<dyn Payment> {
//     match &order.payment_method {
//         PaymentMethod::CreditCard { token } => {
//             Box::new(CreditCardPayment::new(token.into(), charger))
//         }
//         PaymentMethod::PayPal { email } => Box::new(PayPalPayment::new(email.into())),
//     }
// }

// impl<'a> OrderPayment<'a> {
//     pub fn new(payment: &'a dyn Payment) -> Self {
//         Self { payment }
//     }
//
//     fn pay(&self, price: f64) -> bool {
//         self.payment.pay(price)
//     }
// }

pub struct CreditCardPayment<'a> {
    token: String,
    charger: &'a dyn Charger,
}

impl<'a> CreditCardPayment<'a> {
    pub fn new(token: String, charger: &'a dyn Charger) -> Self {
        Self { token, charger }
    }
}

impl<'a> Payment for CreditCardPayment<'a> {
    fn pay(&self, price: f64) -> bool {
        self.charger.charge(price, &self.token)
    }
}

pub struct PayPalPayment {
    email: String,
}

impl PayPalPayment {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

impl Payment for PayPalPayment {
    fn pay(&self, final_price: f64) -> bool {
        // 如果要支持 PayPal，就得在这里加代码，可能还需要新的依赖
        println!(
            "（未实现）通过 PayPal 向 {} 收费 ${:.2}",
            self.email, final_price
        );
        true // 假设成功
    }
}

#[derive(Debug)]
pub struct Order {
    pub customer_tier: CustomerTier,
    pub items: Vec<String>,
    pub total_price: f64,
    pub payment_method: PaymentMethod,
    pub customer_email: String,
}

// --- 违反 SOLID 的“上帝”对象 ---
pub struct OrderProcessor<'a, 'b, 'c> {
    // 罪状一：直接依赖底层具体实现 (违反 DIP)
    payment: &'a dyn Payment,
    db: &'b dyn DBRepository,
    smtp_client: &'c dyn EmailSender,
}

pub trait DBRepository {
    fn save(&self, order: &Order) -> Result<()>;
}

pub struct MysqlRepository {
    pub host: String,
    pub password: String,
    pub ip: String,
    pub port: i32,
}

impl MysqlRepository {
    fn new(host: String, password: String, ip: String, port: i32) -> Self {
        Self {
            host,
            password,
            ip,
            port,
        }
    }
}

impl DBRepository for MysqlRepository {
    fn save(&self, order: &Order) -> Result<()> {
        println!("save order to db {}", order.customer_email);
        Ok(())
    }
}

impl<'a, 'b, 'c> OrderProcessor<'a, 'b, 'c> {
    pub fn new(
        payment: &'a dyn Payment,
        db: &'b dyn DBRepository,
        smtp_client: &'c dyn EmailSender,
    ) -> Self {
        Self {
            payment,
            db,
            smtp_client,
        }
    }

    /// 罪状二：一个方法承担了所有职责：验证、计价、支付、库存、通知... (严重违反 SRP)
    pub fn process(&self, order: &Order) -> Result<()> {
        // --- 职责1: 价格计算 (包含业务规则) ---
        let final_price = order
            .customer_tier
            .get_tier_price_calculator()
            .calculate_price(order.total_price);

        // --- 职责2: 支付处理 ---
        if !self.payment.pay(final_price) {
            return Err(anyhow!("支付失败"));
        }
        println!("支付成功！");

        // --- 职责3: 库存管理 (硬编码) ---
        println!("正在更新库存...(直接调用数据库)");

        if let Err(e) = self.db.save(order) {
            return Err(e);
        }

        // --- 职责4: 发送通知 (硬编码) ---
        self.smtp_client.send_email(
            &order.customer_email,
            "订单确认",
            &format!("您的订单已处理，总金额: ${:.2}", final_price),
        );

        Ok(())
    }
}

// --- 使用示例 ---
fn main() -> Result<()> {
    let order = Order {
        customer_tier: CustomerTier::Premium,
        items: vec!["一本好书".to_string(), "一支好笔".to_string()],
        total_price: 100.0,
        payment_method: PaymentMethod::CreditCard {
            token: "tok_12345".to_string(),
        },
        customer_email: "disciple@example.com".to_string(),
    };

    let charger = StripeGateway::default();
    let payment: &dyn Payment = match &order.payment_method {
        PaymentMethod::CreditCard { token } => &CreditCardPayment::new(token.into(), &charger),
        PaymentMethod::PayPal { email } => &PayPalPayment::new(email.into()),
    };

    let mysql_db = MysqlRepository::new("test".into(), "test".into(), "locahost".into(), 5432);
    let smtp_client = SmtpClient;

    let processor = OrderProcessor::new(payment, &mysql_db, &smtp_client);
    processor.process(&order)?;

    Ok(())
}
