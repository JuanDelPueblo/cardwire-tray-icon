Name:           cardwire-tray
Version:        0.1.0
Release:        1%{?dist}
Summary:        Cardwire Tray

License:        MIT
URL:            https://github.com/
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.60
BuildRequires:  cargo
BuildRequires:  make

%description
Tray icon for the Cardwire utility.

%prep
%autosetup -n %{name}

%build
cargo build --release

%install
rm -rf $RPM_BUILD_ROOT
make install DESTDIR=$RPM_BUILD_ROOT PREFIX=/usr

%files
%{_bindir}/cardwire-tray
%{_datadir}/applications/cardwire-tray.desktop
%{_datadir}/icons/hicolor/scalable/apps/cardwire-tray.svg
%{_datadir}/icons/hicolor/scalable/apps/cardwire-gpu.svg
%{_datadir}/icons/hicolor/scalable/apps/cardwire-integrated.svg
%{_datadir}/icons/hicolor/scalable/apps/cardwire-hybrid.svg
%{_datadir}/icons/hicolor/scalable/apps/cardwire-manual.svg

%changelog
* Fri May 08 2026 Edyan Cruz <edyancruz@outlook.com> - 0.1.0-1
- Initial RPM release
