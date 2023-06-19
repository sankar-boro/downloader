/**
 * Get video ID.
 *
 * There are a few type of video URL formats.
 *  - https://www.youtube.com/watch?v=VIDEO_ID
 *  - https://m.youtube.com/watch?v=VIDEO_ID
 *  - https://youtu.be/VIDEO_ID
 *  - https://www.youtube.com/v/VIDEO_ID
 *  - https://www.youtube.com/embed/VIDEO_ID
 *  - https://music.youtube.com/watch?v=VIDEO_ID
 *  - https://gaming.youtube.com/watch?v=VIDEO_ID
 *
 * @param {string} link
 * @return {string}
 * @throws {Error} If unable to find a id
 * @throws {TypeError} If videoid doesn't match specs
 */

static VALID_QUERY_DOMAINS: [&str; 5] = [
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "music.youtube.com",
    "gaming.youtube.com",
];
static VALID_PATH_DOMAINS: Regex = Regex::new(r"^https?:\/\/(youtu\.be\/|(www\.)?youtube\.com\/(embed|v|shorts)\/)$");
pub fn getURLVideoID(link: &str) -> Result<String, String> {
    let parsed = link.trim();
    let id = parsed.searchParams.get('v');
    if (validPathDomains.test(link.trim()) && !id) {
        let paths = parsed.pathname.split('/');
        if parsed.host == "youtu.be" {
            id = paths[1];
        } else {
            id = paths[2];
        }
    } else if parsed.hostname && !validQueryDomains.has(parsed.hostname) {
        return (String::from("Not a YouTube domain"));
    }
    if !id {
        return format!("No video id found: {}", link);
    }
    id = id.substring(0, 11);
    if !exports.validateID(id) {
        return format!("Video id ({}) does not match expected format ({}))", idRegex.toString());
    }
    return id;
} 

  
/**
   * Gets video ID either from a url or by checking if the given string
   * matches the video ID format.
   *
   * @param {string} str
   * @returns {string}
   * @throws {Error} If unable to find a id
   * @throws {TypeError} If videoid doesn't match specs
   */
static urlRegex: RegeEx = Regex::new(r"^https?:\/\/$");
pub fn getVideoID(str_: &str) -> Result<&str, String> {
    if exports.validateID(str_) {
      return str_;
    } else if urlRegex.test(str.trim()) {
      return exports.getURLVideoID(str);
    } else {
      return format!("No video id found: ", str_);
    }
}
  
  
/**
   * Returns true if given id satifies YouTube's id format.
   *
   * @param {string} id
   * @return {boolean}
   */
static idRegex: Regex = Regex::new("^[a-zA-Z0-9-_]{11}$");
pub fn validateID(id: &str) {
    idRegex.test(id.trim())
}
  
  
  /**
   * Checks wether the input string includes a valid id.
   *
   * @param {string} string
   * @returns {boolean}
   */
pub fn validateURL(str_: &str) {
    // try {
    //     exports.getURLVideoID(string);
    //     return true;
    //   } catch (e) {
    //     return false;
    //   }
}
  