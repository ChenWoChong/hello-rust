use std::sync::atomic::Ordering::SeqCst;
use crate::test2::infrastructure::{Charger, EmailSender, StripeGateway};
use anyhow::{Result, anyhow};
use crate::test2::PaymentMethod::{CreditCard as OtherCreditCard, PayPal as OtherPayPal};

// --- 模拟的底层设施 ---
mod infrastructure {
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
    Regular(Regular),
    Premium(Premium),
}

pub trait TierPriceCalculator {
    fn calculate_price(&self, raw_price: f64) -> f64;
}

#[derive(Debug)]
pub struct Regular;
impl TierPriceCalculator for Regular {
    fn calculate_price(&self, raw_price: f64) -> f64 {
        raw_price
    }
}
#[derive(Debug)]
pub struct Premium;
impl TierPriceCalculator for Premium {
    fn calculate_price(&self, raw_price: f64) -> f64 {
        raw_price * 0.8
    }
}

pub fn calculate_price(raw_price: f64, price_calculator: &dyn TierPriceCalculator) -> f64 {
    price_calculator.calculate_price(raw_price)
}

pub enum PaymentMethod {
    CreditCard,
    PayPal
}

pub trait Payment {
    fn pay(&self, price: f64) -> bool;
}

pub struct CreditCard {
    token: String,
    strip: Box<dyn Charger>,
}

impl CreditCard {
    pub fn new(token: String, strip: Box<dyn Charger>) -> Self {
        Self{
            token,
            strip,
        }
    }
}

impl Payment for CreditCard {
    fn pay(&self, price: f64) -> bool {
        self.strip.charge(price, &self.token)
    }
}

pub struct PayPalPay {
    email: String,
}

impl PayPalPay {
    pub fn new(email: String) -> Self{
        Self{
            email,
        }
    }
}

impl Payment for PayPalPay {
    fn pay(&self, final_price: f64) -> bool {
        // 如果要支持 PayPal，就得在这里加代码，可能还需要新的依赖
        println!(
            "（未实现）通过 PayPal 向 {} 收费 ${:.2}",
            self.email, final_price
        );
        true // 假设成功
    }
}


pub struct OrderPayment<'a>{
    pub payment: &'a dyn Payment
}

impl OrderPayment {

    pub fn new(order: &Order) -> Self{
        match order.payment_method {
            PaymentMethod::CreditCard => {
                let strip = StripeGateway;
                let payment = CreditCard::new(order.customer_email, Box::new(strip));
                Self{
                payment: &payment,
                }
            }
            PaymentMethod::PayPal => {
                let paypal = PayPalPay::new(order.customer_email);
                Self{
                    payment: &paypal,
                }
            }
        }
    }

    pub fn order_payment(&self, final_price: f64) -> bool {
        self.payment.pay(final_price)
    }
}




pub struct Order<'a> {
    pub customer_tier: &'a dyn TierPriceCalculator,
    pub items: Vec<String>,
    pub total_price: f64,
    pub payment_method: PaymentMethod,
    pub customer_email: String,
}

// --- 违反 SOLID 的“上帝”对象 ---
pub struct OrderProcessor<'b> {
    // 罪状一：直接依赖底层具体实现 (违反 DIP)
    smtp_client: &'b dyn EmailSender,
}

impl<'b> OrderProcessor<'b> {
    pub fn new(email_sender: &'b dyn EmailSender) -> Self {
        Self {
            smtp_client: email_sender,
        }
    }

    pub fn calculate_price(&self, order: &Order) -> f64 {
        calculate_price(order.total_price, order.customer_tier)
    }


    /// 罪状二：一个方法承担了所有职责：验证、计价、支付、库存、通知... (严重违反 SRP)
    pub fn process(&self, order: &Order) -> Result<()> {
        // --- 职责1: 价格计算 (包含业务规则) ---
        let final_price = self.calculate_price(order);

        if !self.order_payment(order, final_price) {
            return Err(anyhow!("支付失败"));
        }
        println!("支付成功！");

        // --- 职责3: 库存管理 (硬编码) ---
        println!("正在更新库存...(直接调用数据库)");

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
        customer_tier: &Premium,
        items: vec!["一本好书".to_string(), "一支好笔".to_string()],
        total_price: 100.0,
        payment_method: PaymentMethod::CreditCard {
            token: "tok_12345".to_string(),
        },
        customer_email: "disciple@example.com".to_string(),
    };

    let processor = OrderProcessor::new();
    processor.process(&order)?;

    Ok(())
}
