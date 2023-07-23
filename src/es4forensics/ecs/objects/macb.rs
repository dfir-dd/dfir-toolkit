
#[derive(Default)]
pub struct Macb {
    pub modified: bool,
    pub accessed: bool,
    pub changed: bool,
    pub created: bool,
}

impl From<&Macb> for String {
    fn from(me: &Macb) -> Self {
        let mut macb = ['.', '.', '.', '.'];
        if me.modified { macb[0] = 'm'; }
        if me.accessed { macb[1] = 'a'; }
        if me.changed { macb[2] = 'c'; }
        if me.created { macb[3] = 'b'; }

        macb.into_iter().collect()
    }
}

impl From<&Macb> for Vec<&str> {
    fn from(me: &Macb) -> Self {
        let mut res = Vec::new();
        if me.modified { res.push("modified"); }
        if me.accessed { res.push("accessed"); }
        if me.changed { res.push("changed"); }
        if me.created { res.push("created"); }
        res
    }
}