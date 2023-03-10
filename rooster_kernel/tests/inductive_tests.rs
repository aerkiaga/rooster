mod common;

#[test]
fn nat() {
    common::execute(&[("nat", "Set", "饾悩self:Set.鈭?T:? Set.鈭?_:T.鈭?_:鈭?_:self.T.T")]);
}

#[test]
fn nat_constructors() {
    common::execute(&[
        ("nat", "Set", "饾悩self:Set.鈭?T:? Set.鈭?_:T.鈭?_:鈭?_:self.T.T"),
        ("O", "nat", "位T:? nat.位a:T.位b:鈭?_:nat.T.a"),
        ("S", "鈭?_:nat.nat", "位x:nat.位T:? nat.位a:T.位b:鈭?_:nat.T.b x"),
    ]);
}

#[test]
fn add() {
    common::execute(&[
        ("nat", "Set", "饾悩self:Set.鈭?T:? Set.鈭?_:T.鈭?_:鈭?_:self.T.T"),
        ("O", "nat", "位T:? nat.位a:T.位b:鈭?_:nat.T.a"),
        ("S", "鈭?_:nat.nat", "位x:nat.位T:? nat.位a:T.位b:鈭?_:nat.T.b x"),
        (
            "add",
            "鈭?_:nat.鈭?_:nat.nat",
            "饾悩self:(鈭?_:nat.鈭?_:nat.nat).位n:nat.位m:nat.n (nat) m (位p:(nat).S (self p m))",
        ),
    ]);
}

#[test]
fn nat_induction() {
    common::execute(&[
        ("nat", "Set", "饾悩self:Set.鈭?T:? Set.鈭?_:T.鈭?_:鈭?_:self.T.T"),
        ("O", "nat", "位T:? nat.位a:T.位b:鈭?_:nat.T.a"),
        ("S", "鈭?_:nat.nat", "位x:nat.位T:? nat.位a:T.位b:鈭?_:nat.T.b x"),
        (
            "nat_induction",
            "鈭?P:鈭?_:nat.Prop.鈭?_:P O.鈭?_:鈭?n:nat.鈭?_:P n.P (S n).鈭?n:nat.P n",
            "饾悩self:鈭?P:鈭?_:nat.Prop.鈭?_:P O.鈭?_:鈭?n:nat.鈭?_:P n.P (S n).鈭?n:nat.P n.位P:鈭?_:nat.Prop.位pO:P O.位h:鈭?n:nat.鈭?_:P n.P (S n).位n:nat.n (P n) pO (位p:nat.h p (self P pO h p))",
        ),
    ]);
}
