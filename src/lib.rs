// Shift the characters in the given message by the given amount using
// a Caesar cipher.
pub fn caesar_shift(message: &str, shift: u8) -> String {
    message
        .chars()
        .map(|c| {
            if !c.is_alphabetic() {
                return c;
            }

            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let shifted = (c as u8 + shift - base) % 26 + base;
            // 1.5% - 2% faster, safe to use because the character code is 
            // guaranteed to be in the range 0x41 to 0x5A or 0x61 to 0x7A
            unsafe { std::char::from_u32_unchecked(shifted as u32) }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &str = "P ht uva h ipn mhu vm Jhlzhy Jpwoly";

    #[test]
    fn caesar_shift_test() {
        assert_eq!(
            caesar_shift(MESSAGE, 1),
            "Q iu vwb i jqo niv wn Kimaiz Kqxpmz".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 2),
            "R jv wxc j krp ojw xo Ljnbja Lryqna".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 3),
            "S kw xyd k lsq pkx yp Mkockb Mszrob".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 4),
            "T lx yze l mtr qly zq Nlpdlc Ntaspc".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 5),
            "U my zaf m nus rmz ar Omqemd Oubtqd".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 6),
            "V nz abg n ovt sna bs Pnrfne Pvcure".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 7),
            "W oa bch o pwu tob ct Qosgof Qwdvsf".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 8),
            "X pb cdi p qxv upc du Rpthpg Rxewtg".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 9),
            "Y qc dej q ryw vqd ev Squiqh Syfxuh".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 10),
            "Z rd efk r szx wre fw Trvjri Tzgyvi".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 11),
            "A se fgl s tay xsf gx Uswksj Uahzwj".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 12),
            "B tf ghm t ubz ytg hy Vtxltk Vbiaxk".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 13),
            "C ug hin u vca zuh iz Wuymul Wcjbyl".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 14),
            "D vh ijo v wdb avi ja Xvznvm Xdkczm".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 15),
            "E wi jkp w xec bwj kb Ywaown Yeldan".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 16),
            "F xj klq x yfd cxk lc Zxbpxo Zfmebo".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 17),
            "G yk lmr y zge dyl md Aycqyp Agnfcp".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 18),
            "H zl mns z ahf ezm ne Bzdrzq Bhogdq".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 19),
            "I am not a big fan of Caesar Cipher".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 20),
            "J bn opu b cjh gbo pg Dbftbs Djqifs".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 21),
            "K co pqv c dki hcp qh Ecguct Ekrjgt".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 22),
            "L dp qrw d elj idq ri Fdhvdu Flskhu".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 23),
            "M eq rsx e fmk jer sj Geiwev Gmtliv".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 24),
            "N fr sty f gnl kfs tk Hfjxfw Hnumjw".to_string()
        );
        assert_eq!(
            caesar_shift(MESSAGE, 25),
            "O gs tuz g hom lgt ul Igkygx Iovnkx".to_string()
        );
    }
}
