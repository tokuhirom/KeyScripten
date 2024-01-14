use v5.10;
while (<DATA>) {
    if (/(CGEventFlags_(\S+)):/) {
    push @f, $2;
    #    say qq!self.register_constant("$2", $1)?;!;
    }
}
say join(", ", map {"$_"} @f);

__DATA__
pub const CGEventFlags_kCGEventFlagMaskAlphaShift: CGEventFlags = 65536;
pub const CGEventFlags_kCGEventFlagMaskShift: CGEventFlags = 131072;
pub const CGEventFlags_kCGEventFlagMaskControl: CGEventFlags = 262144;
pub const CGEventFlags_kCGEventFlagMaskAlternate: CGEventFlags = 524288;
pub const CGEventFlags_kCGEventFlagMaskCommand: CGEventFlags = 1048576;
pub const CGEventFlags_kCGEventFlagMaskHelp: CGEventFlags = 4194304;
pub const CGEventFlags_kCGEventFlagMaskSecondaryFn: CGEventFlags = 8388608;
pub const CGEventFlags_kCGEventFlagMaskNumericPad: CGEventFlags = 2097152;
pub const CGEventFlags_kCGEventFlagMaskNonCoalesced: CGEventFlags = 256;
