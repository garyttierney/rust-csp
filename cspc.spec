# Generated by rust2rpm
%bcond_without check

%global crate cspc

Name:           %{crate}
Version:        0.1.0
Release:        1%{?dist}
Summary:        Line oriented search tool using Rust's regex library

License:        Unlicense
URL:            https://github.com/garyttierney/rust-csp
Source0:        https://crates.io/api/v1/crates/%{crate}/%{version}/download#/%{crate}-%{version}.crate

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging
BuildRequires:  (crate(nom) >= 3.2.0 with crate(nom) < 3.3.0)
BuildRequires:  (crate(clap) >= 2.24.1 with crate(clap) < 3.0.0)

%description
%{summary}.

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate}
High-level SELinux policy language.

%prep
%autosetup -n %{crate}-%{version} -p1
%cargo_prep

%build
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%files       -n %{crate}
%doc README.md
%{_bindir}/cspc

%changelog
* Sun Sep 03 2017 Gary Tierney <gary.tierney@gmx.com> - 0.1.0-1
- Initial package

