/**
 * Normalise an IP Address
 *
 * @param {string} ip the IPv6 Addr
 * @returns {number[]} the 8 parts of the IPv6 as Integers
 */
const normalizeIP = ip => {
    // Split by fill position
    const parts = ip.split('::').map(x => x.split(':'));
    // Normalize start and end
    const partStart = parts[0] || [];
    const partEnd = parts[1] || [];
    partEnd.reverse();
    // Placeholder for full ip
    const fullIP = new Array(8).fill(0);
    // Fill in start and end parts
    for (let i = 0; i < Math.min(partStart.length, 8); i++) {
      fullIP[i] = parseInt(partStart[i], 16) || 0;
    }
    for (let i = 0; i < Math.min(partEnd.length, 8); i++) {
      fullIP[7 - i] = parseInt(partEnd[i], 16) || 0;
    }
    return fullIP;
};

// eslint-disable-next-line max-len
const IPV6_REGEX = /^(([0-9a-f]{1,4}:)(:[0-9a-f]{1,4}){1,6}|([0-9a-f]{1,4}:){1,2}(:[0-9a-f]{1,4}){1,5}|([0-9a-f]{1,4}:){1,3}(:[0-9a-f]{1,4}){1,4}|([0-9a-f]{1,4}:){1,4}(:[0-9a-f]{1,4}){1,3}|([0-9a-f]{1,4}:){1,5}(:[0-9a-f]{1,4}){1,2}|([0-9a-f]{1,4}:){1,6}(:[0-9a-f]{1,4})|([0-9a-f]{1,4}:){1,7}(([0-9a-f]{1,4})|:))\/(1[0-1]\d|12[0-8]|\d{1,2})$/;
/**
 * Quick check for a valid IPv6
 * The Regex only accepts a subset of all IPv6 Addresses
 *
 * @param {string} ip the IPv6 block in CIDR-Notation to test
 * @returns {boolean} true if valid
 */
const isIPv6 = ip => IPV6_REGEX.test(ip);

function getRandomIPv6(ip) {
    // Start with a fast Regex-Check
    if (!isIPv6(ip)) throw Error('Invalid IPv6 format');
    // Start by splitting and normalizing addr and mask
    const [rawAddr, rawMask] = ip.split('/');
    let base10Mask = parseInt(rawMask);
    if (!base10Mask || base10Mask > 128 || base10Mask < 24) throw Error('Invalid IPv6 subnet');
    const base10addr = normalizeIP(rawAddr);
    console.log("base10addr", base10addr);
    // Get random addr to pad with
    // using Math.random since we're not requiring high level of randomness
    const randomAddr = new Array(8).fill(1).map(() => Math.floor(Math.random() * 0xffff));
    console.log("randomAddr", randomAddr);
    
    // Merge base10addr with randomAddr
    const mergedAddr = randomAddr.map((randomItem, idx) => {
      // Calculate the amount of static bits
      const staticBits = Math.min(base10Mask, 16);
      // Adjust the bitmask with the staticBits
      base10Mask -= staticBits;
      // Calculate the bitmask
      // lsb makes the calculation way more complicated
      const mask = 0xffff - ((2 ** (16 - staticBits)) - 1);
      console.log('mask', mask)
      // Combine base10addr and random
      return (base10addr[idx] & mask) + (randomItem & (mask ^ 0xffff));
    });
    console.log("mergedAddr", mergedAddr);
    // Return new addr
    let x = mergedAddr.map(x => x.toString('16')).join(':');
    console.log(x);
    return x;
}

getRandomIPv6(`ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/24`)