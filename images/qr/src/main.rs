use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use qrcodegen::QrSegmentMode;
use qrcodegen::Version;


fn main() {
    println!("Hello, world!");
}

// Creates a single QR Code, then prints it to the console.
fn do_basic_demo() {
    let text: &'static str = "Hello, world!";   // User-supplied Unicode text
    let errcorlvl: QrCodeEcc = QrCodeEcc::Low;  // Error correction level

    // Make and print the QR Code symbol
    let mut outbuffer  = vec![0u8; Version::MAX.buffer_len()];
    let mut tempbuffer = vec![0u8; Version::MAX.buffer_len()];
    let qr: QrCode = QrCode::encode_text(text, &mut tempbuffer, &mut outbuffer,
        errcorlvl, Version::MIN, Version::MAX, None, true).unwrap();
    // Note: qr has a reference to outbuffer, so outbuffer needs to outlive qr
    std::mem::drop(tempbuffer);  // Optional, because tempbuffer is only needed during encode_text()
    print_qr(&qr);
    println!("{}", to_svg_string(&qr, 4));
}
