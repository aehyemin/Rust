struct Monster {
    health : i32,
}

struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!("Your strike with your sword! Yout opponent's health is now{}", opponent.health);
    }

    fn attack_with_hand(&self, opponent: &mut Monster){
        opponent.health -= 2;

    }

}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance{
    fn attack_with_bow(&self, opponent: &mut Monster, distance : u32){
        if distance < 10 {
            opponent.health -= 10;
            println!("Your strike with your bow! Yout opponent's health is now {}.", opponent.health);


        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance : u32){
        if distance< 3 {
            opponent.health -= 3;
            println!("Your strike with your rock! Yout opponent's health is now {}.", opponent.health);
        }
    }
}

impl FightFromDistance for Ranger {}

fn main(){
    let radagast = Wizard {};
    let aragorn = Ranger {};

    let mut uruk_hai = Monster { health : 40};

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 7);

}