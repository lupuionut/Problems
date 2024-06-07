// 648. Replace Words
// ------------------

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut sentence = sentence.split(" ").collect::<Vec<&str>>();
        let mut dictionary = dictionary.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        for i in 0..dictionary.len() {
            for j in 0..sentence.len() {
                if sentence[j].starts_with(dictionary[i]) {
                    sentence[j] = dictionary[i];
                }
            }
        }
        let ans = &sentence.join(" ");
        ans.clone()
    }
}
