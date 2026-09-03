use input::{get_user_input, wait_for_user_input};

use crate::creature::Creature;


mod creature;

mod input;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let name = loop { 
        print!("İsim Gir ");
        let name = get_user_input()?;

        if name.trim().is_empty() {
            eprintln!("İsmin boşluk olamaz!!!!!");
            continue;
        }
        if name.chars().next().unwrap_or_default().is_numeric() {
            eprintln!("Ismini degistir");
            continue;
        }

        break name;
    };
    let mut player = Creature::create_player(name.trim().to_string(), 1.0, 0.0, 100.0, 100.0, 70.0, 5.0);
    loop { 
        clear_terminal();
        println!("[-------------------]");
        println!("Dinlemek: 1\nSavaşmak: 2\nVazgeçmek: 3\nOyuncu İstatistikleri: 4");
        let girdi = get_user_input()?;

        let sayi = match girdi.trim().parse::<i16>() {
            Ok(num) => num,
            Err(why) => return Err(format!("String değeri sayıya çevrilemedi: {}", why).into())
        };
        if sayi < 1 || sayi > 4 {
            eprintln!("Kullanıcı Seçenek Dışı Değer Girdi, sadece: (1,2,3 ve 4)");
            continue;
        }

        match sayi {
            1 => dinlenmek(&mut player),
            2 => savas(&mut player)?,
            3 => { vazgec(&player); break; },
            4 => oyuncu_detaylari(&player),
            _ => ()
        }
    }

    Ok(())
}

fn savas(player: &mut Creature) -> Result<(), Box<dyn std::error::Error>> { // (player.lvl - 2.0).max(1.0)..=player.lvl +2.0

    clear_terminal();
    println!("\x1b[2;20HSavaş Başladı");
    let mut enemy = Creature::create_enemy("Vermil".to_string(), (fastrand::f32() * (player.lvl + 2.0) - (player.lvl - 2.0)).round(), 31.0);
    let xp_gain =  calculate_xp(enemy.lvl, player.lvl);

    println!("Karşına Gelen Düşman {} Seviyede Savaşmak(E) Kaçmak(H)",enemy.lvl);

    let girdi = loop {
        if let Ok(girdi) = get_user_input() {
            let girdi = girdi.trim(); 
            if "e".eq_ignore_ascii_case(girdi) {
                break true;
            }else if "h".eq_ignore_ascii_case(girdi)  {
                break false;
            }
        }
        
    };
    let kacabilme = kacmak();
    if !girdi && kacabilme {println!("Kaçabildin Dayı" ); wait_for_user_input();return Ok(())}
     else if !girdi && !kacabilme {println!("Kaçamadın Dayı"); wait_for_user_input();}
    loop {
        clear_terminal();
        println!("{} Health: {}\n{} Health: {} \nDevam etmek İçin Herhangi Bir Tuşa Basın",
        enemy.name,enemy.health,player.name,player.health);
        let enemy_take_damage: f32 = player.damage * ((100.0 - enemy.defense).max(0.0) / 100.0);
        let enemy_give_damage: f32 = enemy.damage * ((100.0 - player.defense).max(0.0) / 100.0);
        wait_for_user_input(); 

        

        if enemy.health - enemy_take_damage <= 0.0 {
            println!("{} {} Hasar Yedi Ve Geberdi",
            enemy.name,enemy_take_damage);
            break;
        }
        enemy.health -= enemy_take_damage;

        println!("{} {} Hasar Yedi",
        enemy.name,enemy_take_damage);


        if  player.health - enemy_give_damage <= 0.0  {
            let score = player.calculate_score();

            println!("{} Hasara Dayanamadı Ve Karakterin Vefat Etti kazanılan skor: {}",
            enemy_give_damage, score);

            wait_for_user_input();

            std::process::exit(0);
        }
        player.health -= enemy_give_damage  ;

        println!("Düşman Sana {} Hasar Verdi",
        enemy_give_damage);

        wait_for_user_input();                              
    }

    println!("Kazanılan XP: {}", xp_gain);

    let delta = player.xp + xp_gain;

    if delta >= 100.0 {
        player.lvl += 1.0;

        player.xp = delta - 100.0;

        player.max_health += 50.0;

        player.damage += 50.0;

        player.defense += 2.5;

        println!("Xp Değeri Sınıra Ulaştı Level Değerin {} Oldu, Maksimum can değeri arttı.", player.lvl);
    }else {
        player.xp = delta;
    }

    wait_for_user_input();
    Ok(())
}

fn oyuncu_detaylari(player: &Creature) {
    clear_terminal();
    println!("Name: {}\nXP: {}\nLevel: {}\nHealth: {}\n Damage: {}\n Defense: {}",
            player.name, player.xp, player.lvl, player.health, player.damage, player.defense);
    wait_for_user_input();
}

fn kacmak() -> bool {
    if fastrand::f32() * 100.0 >= 50.0 {
        true
    } else {
        false
    }
}
fn calculate_xp(enemy_lvl: f32, player_lvl: f32) -> f32 {
    let base_xp = 70.0;

    let enemy_multiplier = enemy_lvl.powf(1.15);

    let level_difference = enemy_lvl - player_lvl;
    let difficulty_multiplier = (1.0 + level_difference * 0.15).clamp(0.1, 3.0);

    let random_multiplier = 0.9 + fastrand::f32() * 0.2;

    (base_xp
        * enemy_multiplier
        * difficulty_multiplier
        * random_multiplier)
        .round()
}


fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn dinlenmek(player: &mut Creature) {
    let delta = fastrand::f32().round() * player.max_health - 20.0;
    let yenilenecek = delta.clamp(0.0, player.max_health - player.health);
    player.health += yenilenecek;

    println!("Karakter dinlendi Ve {} Can Yeniledi", yenilenecek);
    wait_for_user_input();
}

fn vazgec(player: &Creature) {
    let score = player.calculate_score();
    println!("Korkup Kaçtın Skorun İse {}", score);
    wait_for_user_input();
}
