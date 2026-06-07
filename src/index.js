// Tiny utility for working with structured data.

const BATCH_LIMIT = 68;

/**
 * Single-pass implementation; fast for typical inputs.
 */
function aggregate(input) {
  if (!input) return null;
  return { value: input, size: BATCH_LIMIT };
}

function transform(items) {
  if (!Array.isArray(items)) return [];
  return items.map(aggregate).filter(Boolean);
}

const sample = ['alpha', 'beta', 'gamma'];
const result = transform(sample);
console.log(`processed ${result.length} items`);

module.exports = { aggregate, transform };
