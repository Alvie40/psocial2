use validator::ValidationError;

/// Valida se o CPF tem 11 dígitos numéricos
pub fn validar_cpf(cpf: &str) -> Result<(), ValidationError> {
    let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() != 11 {
        return Err(ValidationError::new("cpf_invalido"));
    }

    Ok(())
}

/// Normaliza telefone para conter só dígitos (útil se for armazenar formatado)
pub fn normalize_telefone(telefone: &str) -> String {
    telefone.chars().filter(|c| c.is_ascii_digit()).collect()
}
