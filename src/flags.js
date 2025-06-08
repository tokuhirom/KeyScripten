const kCGEventFlagMaskAlphaShift = 65536;
const kCGEventFlagMaskShift = 131072;
const kCGEventFlagMaskControl = 262144;
const kCGEventFlagMaskAlternate = 524288;
const kCGEventFlagMaskCommand = 1048576;
const kCGEventFlagMaskHelp = 4194304;
const kCGEventFlagMaskSecondaryFn = 8388608;
const kCGEventFlagMaskNumericPad = 2097152;
const kCGEventFlagMaskNonCoalesced = 256;

/**
 * @param {number} flags
 */
export function flagsToString(flags) {
	let flagNames = [];

	if (flags & kCGEventFlagMaskAlphaShift) flagNames.push("AlphaShift");
	if (flags & kCGEventFlagMaskShift) flagNames.push("Shift");
	if (flags & kCGEventFlagMaskControl) flagNames.push("Control");
	if (flags & kCGEventFlagMaskAlternate) flagNames.push("Alternate");
	if (flags & kCGEventFlagMaskCommand) flagNames.push("Command");
	if (flags & kCGEventFlagMaskHelp) flagNames.push("Help");
	if (flags & kCGEventFlagMaskSecondaryFn) flagNames.push("SecondaryFn");
	if (flags & kCGEventFlagMaskNumericPad) flagNames.push("NumericPad");
	if (flags & kCGEventFlagMaskNonCoalesced) flagNames.push("NonCoalesced");

	return flagNames.join(" | ");
}
