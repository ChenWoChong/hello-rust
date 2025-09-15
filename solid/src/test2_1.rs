use anyhow::{anyhow, Result};

// --- 模拟的底层设施 ---
mod infrastructure {
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
pub enum CustomerTier { Regular, Premium }

#[derive(Debug)]
pub enum PaymentMethod {
    CreditCard { token: String },
    PayPal { email: String },
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
pub struct OrderProcessor {
    // 罪状一：直接依赖底层具体实现 (违反 DIP)
    stripe: infrastructure::StripeGateway,
    smtp_client: infrastructure::SmtpClient,
}

impl OrderProcessor {
    pub fn new() -> Self {
        Self {
            stripe: infrastructure::StripeGateway {},
            smtp_client: infrastructure::SmtpClient {},
        }
    }

    /// 罪状二：一个方法承担了所有职责：验证、计价、支付、库存、通知... (严重违反 SRP)
    pub fn process(&self, order: &Order) -> Result<()> {
        // --- 职责1: 价格计算 (包含业务规则) ---
        let mut final_price = order.total_price;
        // 罪状三：每增加一种会员等级，都必须修改这里的代码 (违反 OCP)
        match order.customer_tier {
            CustomerTier::Regular => { /* 无折扣 */ }
            CustomerTier::Premium => {
                println!("应用 Premium 会员折扣: 10%");
                final_price *= 0.9;
            }
        }

        // --- 职责2: 支付处理 ---
        let payment_successful = match &order.payment_method {
            // 罪状四：每增加一种支付方式，都必须修改这里的代码 (违反 OCP)
            PaymentMethod::CreditCard { token } => {
                self.stripe.charge(final_price, token)
            }
            PaymentMethod::PayPal { email } => {
                // 如果要支持 PayPal，就得在这里加代码，可能还需要新的依赖
                println!("（未实现）通过 PayPal 向 {} 收费 ${:.2}", email, final_price);
                true // 假设成功
            }
        };

        if !payment_successful {
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
        customer_tier: CustomerTier::Premium,
        items: vec!["一本好书".to_string(), "一支好笔".to_string()],
        total_price: 100.0,
        payment_method: PaymentMethod::CreditCard { token: "tok_12345".to_string() },
        customer_email: "disciple@example.com".to_string(),
    };

    let processor = OrderProcessor::new();
    processor.process(&order)?;

    Ok(())
}