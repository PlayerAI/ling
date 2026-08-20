"use strict";

const {
  UNICODE_VERSION,
  DERIVED_CORE_PROPERTIES_SHA256,
  XID_START_RANGES,
  XID_CONTINUE_RANGES,
} = require("./unicode-identifiers.generated.js");

function normalizeRanges(ranges) {
  const sorted = ranges
    .map(([start, end]) => [start, end])
    .sort(([left], [right]) => left - right);
  const normalized = [];
  for (const [start, end] of sorted) {
    const previous = normalized.at(-1);
    if (previous && start <= previous[1] + 1) {
      previous[1] = Math.max(previous[1], end);
    } else {
      normalized.push([start, end]);
    }
  }
  return normalized;
}

function scalarEscape(codepoint) {
  return `\\x{${codepoint.toString(16).toUpperCase()}}`;
}

function characterClass(ranges) {
  const body = ranges
    .map(([start, end]) =>
      start === end
        ? scalarEscape(start)
        : `${scalarEscape(start)}-${scalarEscape(end)}`,
    )
    .join("");
  return `[${body}]`;
}

const identifierStartRanges = normalizeRanges([
  ...XID_START_RANGES,
  [0x5f, 0x5f],
]);
const identifierStartPattern = characterClass(identifierStartRanges);
const identifierContinuePattern = characterClass(XID_CONTINUE_RANGES);
const IDENTIFIER_PATTERN = `${identifierStartPattern}${identifierContinuePattern}*`;

module.exports = Object.freeze({
  UNICODE_VERSION,
  DERIVED_CORE_PROPERTIES_SHA256,
  IDENTIFIER_PATTERN,
});
