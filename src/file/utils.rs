use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::share_key_secret, error::Error};

#[derive(Serialize, Deserialize)]
pub struct KeyClaim {
	sub: String,
	exp: usize,
}

pub enum Expiry {
	M2,
	M5,
	M10,
	M15,
	M30,
	H1,
	H2,
	H4,
	H8,
	H12,
	H24,
	H48,
	H72,
}

impl From<String> for Expiry {
	fn from(s: String) -> Self {
		match s.as_str() {
			"2m" => Expiry::M2,
			"5m" => Expiry::M5,
			"10m" => Expiry::M10,
			"15m" => Expiry::M15,
			"30m" => Expiry::M30,
			"1h" => Expiry::H1,
			"2h" => Expiry::H2,
			"4h" => Expiry::H4,
			"8h" => Expiry::H8,
			"12h" => Expiry::H12,
			"24h" => Expiry::H24,
			"48h" => Expiry::H48,
			"72h" => Expiry::H72,
			_ => Expiry::M2,
		}
	}
}

impl From<Expiry> for Duration {
	fn from(expiry: Expiry) -> Self {
		match expiry {
			Expiry::M2 => Duration::minutes(2),
			Expiry::M5 => Duration::minutes(5),
			Expiry::M10 => Duration::minutes(10),
			Expiry::M15 => Duration::minutes(15),
			Expiry::M30 => Duration::minutes(30),
			Expiry::H1 => Duration::hours(1),
			Expiry::H2 => Duration::hours(2),
			Expiry::H4 => Duration::hours(4),
			Expiry::H8 => Duration::hours(8),
			Expiry::H12 => Duration::hours(12),
			Expiry::H24 => Duration::hours(24),
			Expiry::H48 => Duration::hours(48),
			Expiry::H72 => Duration::hours(72),
		}
	}
}

pub fn encode_key(file_id: &str, duration: Expiry) -> Result<String, Error> {
	let duration: Duration = duration.into();

	let exp = Utc::now().checked_add_signed(duration).unwrap().timestamp() as usize;

	let claims = KeyClaim {
		sub: file_id.to_string(),
		exp,
	};

	let header = Header::new(Algorithm::HS512);

	let key = EncodingKey::from_secret(share_key_secret().as_bytes());

	let encode = encode(&header, &claims, &key)?;

	Ok(encode)
}

pub fn decode_key(token: String) -> Result<String, Error> {
	let key = DecodingKey::from_secret(share_key_secret().as_bytes());

	let validation = Validation::new(Algorithm::HS512);

	let decoded = decode::<KeyClaim>(&token, &key, &validation)?;

	Ok(decoded.claims.sub)
}
