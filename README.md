# Image to PDF

Tauri tabanlı basit bir masaüstü uygulaması. Birden fazla resmi seçip tek bir PDF dosyasına dönüştürür.

cargo tauri build --target i686-pc-windows-msvc

## Özellikler

- Çoklu resim seçimi (PNG, JPG, JPEG, GIF, BMP, WEBP, TIFF)
- Her resim ayrı bir PDF sayfasına yerleştirilir (A4)
- Resimler sayfaya sığacak şekilde ölçeklenir (en-boy oranı korunur)
- Modern ve sade arayüz

## Geliştirme

```bash
cd src-tauri
cargo build

# Uygulamayı çalıştır (proje kökünden)
cargo tauri dev
```

## Kullanım

1. **Resimleri Seç** butonuna tıklayın
2. İstediğiniz resimleri seçin
3. **PDF Oluştur** ile kaydetme konumunu belirleyin
4. PDF oluşturulur

## Teknolojiler

- Tauri 2
- Rust (`printpdf` + `image`)
- Vanilla HTML / CSS / JS
