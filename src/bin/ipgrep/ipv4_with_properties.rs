use std::net::Ipv4Addr;

pub struct Ipv4WithProperties {
    wrapped_address: Ipv4Addr
}

impl From<Ipv4Addr> for Ipv4WithProperties {
    fn from(value: Ipv4Addr) -> Self {
        Self {
            wrapped_address: value
        }
    }
}

impl AsRef<Ipv4Addr> for Ipv4WithProperties {
    fn as_ref(&self) -> &Ipv4Addr {
        &self.wrapped_address
    }
}

impl Ipv4WithProperties {
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use std::net::Ipv4Addr;
    /// use ipgrep::Ipv4WithProperties;
    ///
    /// // Most IPv4 addresses are globally reachable:
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(80, 9, 12, 3)).is_global(), true);
    ///
    /// // However some addresses have been assigned a special meaning
    /// // that makes them not globally reachable. Some examples are:
    ///
    /// // The unspecified address (`0.0.0.0`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::UNSPECIFIED).is_global(), false);
    ///
    /// // Addresses reserved for private use (`10.0.0.0/8`, `172.16.0.0/12`, 192.168.0.0/16)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(10, 254, 0, 0)).is_global(), false);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(192, 168, 10, 65)).is_global(), false);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(172, 16, 10, 65)).is_global(), false);
    ///
    /// // Addresses in the shared address space (`100.64.0.0/10`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(100, 100, 0, 0)).is_global(), false);
    ///
    /// // The loopback addresses (`127.0.0.0/8`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::LOCALHOST).is_global(), false);
    ///
    /// // Link-local addresses (`169.254.0.0/16`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(169, 254, 45, 1)).is_global(), false);
    ///
    /// // Addresses reserved for documentation (`192.0.2.0/24`, `198.51.100.0/24`, `203.0.113.0/24`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(192, 0, 2, 255)).is_global(), false);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(198, 51, 100, 65)).is_global(), false);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(203, 0, 113, 6)).is_global(), false);
    ///
    /// // Addresses reserved for benchmarking (`198.18.0.0/15`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(198, 18, 0, 0)).is_global(), false);
    ///
    /// // Reserved addresses (`240.0.0.0/4`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(250, 10, 20, 30)).is_global(), false);
    ///
    /// // The broadcast address (`255.255.255.255`)
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::BROADCAST).is_global(), false);
    ///
    /// // For a complete overview see the IANA IPv4 Special-Purpose Address Registry.
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_global(&self) -> bool {
        !(self.wrapped_address.octets()[0] == 0 // "This network"
            || self.wrapped_address.is_private()
            || self.is_shared()
            || self.wrapped_address.is_loopback()
            || self.wrapped_address.is_link_local()
            // addresses reserved for future protocols (`192.0.0.0/24`)
            ||(self.wrapped_address.octets()[0] == 192 && self.wrapped_address.octets()[1] == 0 && self.wrapped_address.octets()[2] == 0)
            || self.wrapped_address.is_documentation()
            || self.is_benchmarking()
            || self.is_reserved()
            || self.wrapped_address.is_broadcast())
    }

    /// Returns [`true`] if this address is part of the Shared Address Space defined in
    /// [IETF RFC 6598] (`100.64.0.0/10`).
    ///
    /// [IETF RFC 6598]: https://tools.ietf.org/html/rfc6598
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipgrep::Ipv4WithProperties;
    ///
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(100, 64, 0, 0)).is_shared(), true);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(100, 127, 255, 255)).is_shared(), true);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(100, 128, 0, 0)).is_shared(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_shared(&self) -> bool {
        self.wrapped_address.octets()[0] == 100 && (self.wrapped_address.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }

    /// Returns [`true`] if this address part of the `198.18.0.0/15` range, which is reserved for
    /// network devices benchmarking. This range is defined in [IETF RFC 2544] as `192.18.0.0`
    /// through `198.19.255.255` but [errata 423] corrects it to `198.18.0.0/15`.
    ///
    /// [IETF RFC 2544]: https://tools.ietf.org/html/rfc2544
    /// [errata 423]: https://www.rfc-editor.org/errata/eid423
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipgrep::Ipv4WithProperties;
    ///
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(198, 17, 255, 255)).is_benchmarking(), false);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(198, 18, 0, 0)).is_benchmarking(), true);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(198, 19, 255, 255)).is_benchmarking(), true);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(198, 20, 0, 0)).is_benchmarking(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        self.wrapped_address.octets()[0] == 198 && (self.wrapped_address.octets()[1] & 0xfe) == 18
    }

    /// Returns [`true`] if this address is reserved by IANA for future use. [IETF RFC 1112]
    /// defines the block of reserved addresses as `240.0.0.0/4`. This range normally includes the
    /// broadcast address `255.255.255.255`, but this implementation explicitly excludes it, since
    /// it is obviously not reserved for future use.
    ///
    /// [IETF RFC 1112]: https://tools.ietf.org/html/rfc1112
    ///
    /// # Warning
    ///
    /// As IANA assigns new addresses, this method will be
    /// updated. This may result in non-reserved addresses being
    /// treated as reserved in code that relies on an outdated version
    /// of this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipgrep::Ipv4WithProperties;
    ///
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(240, 0, 0, 0)).is_reserved(), true);
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(255, 255, 255, 254)).is_reserved(), true);
    ///
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(239, 255, 255, 255)).is_reserved(), false);
    /// // The broadcast address is not considered as reserved for future use by this implementation
    /// assert_eq!(Ipv4WithProperties::from(Ipv4Addr::new(255, 255, 255, 255)).is_reserved(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_reserved(&self) -> bool {
        self.wrapped_address.octets()[0] & 240 == 240 && !self.wrapped_address.is_broadcast()
    }

    #[inline]
    #[must_use]
    pub const fn is_loopback(&self) -> bool {
        self.wrapped_address.is_loopback()
    }

    #[inline]
    #[must_use]
    pub const fn is_private(&self) -> bool {
        self.wrapped_address.is_private()
    }
}

