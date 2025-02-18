# Maintainer: Canmi21 <9997200@qq.com>
# Contributor: Canmi(Canmi21)

pkgname=hypraway
pkgver=1.1.0
pkgrel=1
pkgdesc="Lock screen automatically when you leave."
arch=('x86_64')
url="https://github.com/canmi21/hypraway"
license=('MIT')
depends=('glibc')
makedepends=('cargo')
source=("git+https://github.com/canmi21/hypraway.git#branch=master" "LICENSE")
sha256sums=('SKIP' 'SKIP')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 target/release/hypraway "$pkgdir/usr/bin/hypraway"
  local username=$(whoami)
  cat <<EOF > "$pkgdir/etc/systemd/system/hypraway.service"
[Unit]
Description=Hypraway Service
After=multi-user.target

[Service]
ExecStart=/usr/bin/hypraway
WorkingDirectory=/home/$username
Restart=always
User=$username
Group=$username
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

post_install() {
  echo "Hypraway service has been installed."
  echo "To enable and start the service, use the following commands:"
  echo "  sudo systemctl enable hypraway.service"
  echo "  sudo systemctl start hypraway.service"
}
