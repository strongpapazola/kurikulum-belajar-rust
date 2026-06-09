// ============================================
// MINI PROJECT: REST API Pajak dengan Axum
// ============================================
// Setup:
//   cd 08_mini_projects/rest_api
//   cargo run
//
// Test dengan curl:
//   curl -X POST http://localhost:3000/hitung/ppn \
//     -H "Content-Type: application/json" \
//     -d '{"dpp": 5000000}'
//
//   curl -X POST http://localhost:3000/hitung/pph21 \
//     -H "Content-Type: application/json" \
//     -d '{"penghasilan_setahun": 120000000, "menikah": true}'
//
//   curl http://localhost:3000/health

use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// ==================
// REQUEST TYPES
// ==================
#[derive(Deserialize)]
struct RequestPPN {
    dpp: f64,
}

#[derive(Deserialize)]
struct RequestPPh21 {
    penghasilan_setahun: f64,
    menikah: Option<bool>,
}

#[derive(Deserialize)]
struct RequestPPh23 {
    dpp: f64,
    objek: Option<String>,
}

#[derive(Deserialize)]
struct RequestValidasiNpwp {
    npwp: String,
}

// ==================
// RESPONSE TYPES
// ==================
#[derive(Serialize)]
struct ResponsePPN {
    dpp: f64,
    tarif_persen: f64,
    ppn: f64,
    total: f64,
}

#[derive(Serialize)]
struct ResponsePPh21 {
    penghasilan_setahun: f64,
    ptkp: f64,
    pkp: f64,
    pph21_tahunan: f64,
    pph21_bulanan: f64,
    take_home_bulanan: f64,
}

#[derive(Serialize)]
struct ResponsePPh23 {
    dpp: f64,
    objek: String,
    tarif_persen: f64,
    pajak: f64,
    netto_diterima: f64,
}

#[derive(Serialize)]
struct ResponseValidasi {
    npwp: String,
    valid: bool,
    pesan: String,
}

#[derive(Serialize)]
struct ResponseError {
    error: String,
}

#[derive(Serialize)]
struct ResponseHealth {
    status: String,
    versi: String,
    service: String,
}

// ==================
// BUSINESS LOGIC
// ==================
fn kalkulasi_ppn(dpp: f64) -> Result<ResponsePPN, String> {
    if dpp <= 0.0 {
        return Err("DPP harus lebih dari 0".to_string());
    }
    let ppn = dpp * 0.11;
    Ok(ResponsePPN {
        dpp,
        tarif_persen: 11.0,
        ppn,
        total: dpp + ppn,
    })
}

fn kalkulasi_pph21(penghasilan: f64, menikah: bool) -> Result<ResponsePPh21, String> {
    if penghasilan <= 0.0 {
        return Err("Penghasilan harus lebih dari 0".to_string());
    }
    let ptkp = if menikah { 58_500_000.0 } else { 54_000_000.0 };
    let pkp  = (penghasilan - ptkp).max(0.0);

    let pph21_tahunan = hitung_progresif(pkp);
    let pph21_bulanan = pph21_tahunan / 12.0;
    let gaji_bulanan  = penghasilan / 12.0;

    Ok(ResponsePPh21 {
        penghasilan_setahun: penghasilan,
        ptkp,
        pkp,
        pph21_tahunan,
        pph21_bulanan,
        take_home_bulanan: gaji_bulanan - pph21_bulanan,
    })
}

fn hitung_progresif(pkp: f64) -> f64 {
    let mut pajak = 0.0;
    pajak += pkp.min(60_000_000.0) * 0.05;
    if pkp > 60_000_000.0 {
        pajak += (pkp - 60_000_000.0).min(190_000_000.0) * 0.15;
    }
    if pkp > 250_000_000.0 {
        pajak += (pkp - 250_000_000.0).min(250_000_000.0) * 0.25;
    }
    if pkp > 500_000_000.0 {
        pajak += (pkp - 500_000_000.0) * 0.30;
    }
    pajak
}

fn kalkulasi_pph23(dpp: f64, objek: &str) -> Result<ResponsePPh23, String> {
    if dpp <= 0.0 {
        return Err("DPP harus lebih dari 0".to_string());
    }
    let tarif = match objek.to_lowercase().as_str() {
        "jasa" | "jasa_lain" | "sewa" => 0.02,
        "dividen" | "bunga" | "royalti" => 0.15,
        _ => return Err(format!("Objek '{}' tidak dikenal", objek)),
    };
    let pajak = dpp * tarif;
    Ok(ResponsePPh23 {
        dpp,
        objek: objek.to_string(),
        tarif_persen: tarif * 100.0,
        pajak,
        netto_diterima: dpp - pajak,
    })
}

fn validasi_npwp_str(npwp: &str) -> ResponseValidasi {
    let digit: String = npwp.chars().filter(|c| c.is_ascii_digit()).collect();
    let valid = digit.len() == 15;
    ResponseValidasi {
        npwp: npwp.to_string(),
        valid,
        pesan: if valid {
            "NPWP valid".to_string()
        } else {
            format!("NPWP tidak valid: {} digit ditemukan, harus 15", digit.len())
        },
    }
}

// ==================
// HANDLERS
// ==================
async fn health_check() -> impl IntoResponse {
    Json(ResponseHealth {
        status: "ok".to_string(),
        versi: "1.0.0".to_string(),
        service: "REST API Pajak".to_string(),
    })
}

async fn hitung_ppn(
    Json(body): Json<RequestPPN>,
) -> impl IntoResponse {
    match kalkulasi_ppn(body.dpp) {
        Ok(hasil) => (StatusCode::OK, Json(serde_json::to_value(hasil).unwrap())),
        Err(e)    => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::to_value(ResponseError { error: e }).unwrap()),
        ),
    }
}

async fn hitung_pph21(
    Json(body): Json<RequestPPh21>,
) -> impl IntoResponse {
    let menikah = body.menikah.unwrap_or(false);
    match kalkulasi_pph21(body.penghasilan_setahun, menikah) {
        Ok(hasil) => (StatusCode::OK, Json(serde_json::to_value(hasil).unwrap())),
        Err(e)    => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::to_value(ResponseError { error: e }).unwrap()),
        ),
    }
}

async fn hitung_pph23(
    Json(body): Json<RequestPPh23>,
) -> impl IntoResponse {
    let objek = body.objek.unwrap_or_else(|| "jasa".to_string());
    match kalkulasi_pph23(body.dpp, &objek) {
        Ok(hasil) => (StatusCode::OK, Json(serde_json::to_value(hasil).unwrap())),
        Err(e)    => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::to_value(ResponseError { error: e }).unwrap()),
        ),
    }
}

async fn validasi_npwp(
    Json(body): Json<RequestValidasiNpwp>,
) -> impl IntoResponse {
    let hasil = validasi_npwp_str(&body.npwp);
    let status = if hasil.valid { StatusCode::OK } else { StatusCode::UNPROCESSABLE_ENTITY };
    (status, Json(serde_json::to_value(hasil).unwrap()))
}

// ==================
// ROUTER & MAIN
// ==================
fn buat_router() -> Router {
    Router::new()
        .route("/health",           get(health_check))
        .route("/hitung/ppn",       post(hitung_ppn))
        .route("/hitung/pph21",     post(hitung_pph21))
        .route("/hitung/pph23",     post(hitung_pph23))
        .route("/validasi/npwp",    post(validasi_npwp))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app  = buat_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("🦀 REST API Pajak berjalan di http://{addr}");
    println!("\nEndpoint yang tersedia:");
    println!("  GET  /health");
    println!("  POST /hitung/ppn      {{ \"dpp\": 5000000 }}");
    println!("  POST /hitung/pph21    {{ \"penghasilan_setahun\": 120000000, \"menikah\": true }}");
    println!("  POST /hitung/pph23    {{ \"dpp\": 10000000, \"objek\": \"jasa\" }}");
    println!("  POST /validasi/npwp   {{ \"npwp\": \"12.345.678.9-012.345\" }}");
    println!("\nTekan Ctrl+C untuk berhenti\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ==================
// TESTS
// ==================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppn() {
        let hasil = kalkulasi_ppn(5_000_000.0).unwrap();
        assert_eq!(hasil.ppn, 550_000.0);
        assert_eq!(hasil.total, 5_550_000.0);
    }

    #[test]
    fn test_ppn_negatif() {
        assert!(kalkulasi_ppn(-1000.0).is_err());
        assert!(kalkulasi_ppn(0.0).is_err());
    }

    #[test]
    fn test_pph21_menikah() {
        // Gaji 10jt/bulan = 120jt/tahun, menikah
        let hasil = kalkulasi_pph21(120_000_000.0, true).unwrap();
        assert_eq!(hasil.ptkp, 58_500_000.0);
        assert_eq!(hasil.pkp, 61_500_000.0);
        // PPh: 60jt*5% + 1.5jt*15% = 3_000_000 + 225_000 = 3_225_000
        assert_eq!(hasil.pph21_tahunan, 3_225_000.0);
    }

    #[test]
    fn test_validasi_npwp() {
        assert!(validasi_npwp_str("12.345.678.9-012.345").valid);
        assert!(!validasi_npwp_str("123-INVALID").valid);
        assert!(!validasi_npwp_str("").valid);
    }

    #[test]
    fn test_pph23_objek_tidak_dikenal() {
        assert!(kalkulasi_pph23(1_000_000.0, "tidak_ada").is_err());
    }
}
