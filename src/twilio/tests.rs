#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestApp;
    use uuid::Uuid;
    use std::env;

    #[tokio::test]
    async fn test_criar_twilio() {
        let app = TestApp::new().await;
        let empresa_id = Uuid::new_v4();

        let input = TwilioInput {
            account_sid: "test_sid".to_string(),
            auth_token: "test_token".to_string(),
            from_number: "whatsapp:+14155238886".to_string(),
            empresa_id,
        };

        let response = app.post("/twilio/create", input).await;
        assert_eq!(response.status(), 200);

        let twilio: TwilioResponse = response.json().await;
        assert_eq!(twilio.account_sid, "test_sid");
        assert_eq!(twilio.from_number, "whatsapp:+14155238886");
        assert_eq!(twilio.empresa_id, empresa_id);
    }

    #[tokio::test]
    async fn test_enviar_sms() {
        let app = TestApp::new().await;
        let empresa_id = Uuid::new_v4();

        // Configurar ambiente sandbox para teste
        env::set_var("TWILIO_SANDBOX", "true");

        // Primeiro criar configuração do Twilio
        let twilio_input = TwilioInput {
            account_sid: env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID não configurado"),
            auth_token: env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN não configurado"),
            from_number: "whatsapp:+14155238886".to_string(),
            empresa_id,
        };
        app.post("/twilio/create", twilio_input).await;

        // Tentar enviar mensagem WhatsApp
        let sms_input = SendSmsInput {
            to_number: "whatsapp:+5511910114427".to_string(), // Número verificado no sandbox
            message: "Test message".to_string(),
            empresa_id,
        };

        let response = app.post("/twilio/send-sms", sms_input).await;
        assert_eq!(response.status(), 200);
    }
} 