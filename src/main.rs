use player::Player;
use input::{get_user_input, wait_for_user_input};
use enemy::Enemy;
mod enemy;
mod player;
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
    let mut player = Player::new(name.trim().to_owned()); 
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

fn savas(player: &mut Player) -> Result<(), Box<dyn std::error::Error>> {
    clear_terminal();
    println!("\x1b[2;20HSavaş Başladı");
    let mut enemy = Enemy::new(player.lvl); 
    let xp_gain = fastrand::i16(40..=60) + (player.lvl as i16 * (enemy.get_level() as i16 - player.lvl as i16));

    println!("Karşına Gelen Düşman {} Seviyede Savaşmak(E) Kaçmak(H)",enemy.get_level());

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
        println!("Enemy Health: {}\n{} Health: {} \nDevam etmek İçin Herhangi Bir Tuşa Basın",enemy.health,player.name,player.health);
        wait_for_user_input(); 
        let player_give_damage = fastrand::i16(50..=player.max_take_damage as i16);
        if enemy.health as i16 - player_give_damage <= 0 {break;}
        enemy.health -= player_give_damage as u16;
        println!("Düşmana {} Kadar Hasar Verdin Şimdi Sıra Onda", player_give_damage);
        let enemy_give_damage = fastrand::i16(20..=enemy.damage as i16);
        if player.health as i16 - enemy_give_damage <= 0  {
            let score = player.calculate_score();
            println!("{} Hasara Dayanamadı Ve Karakterin Vefat Etti kazanılan skor: {}",enemy_give_damage, score);
            wait_for_user_input();
            std::process::exit(0);
        }
        player.health -= enemy_give_damage as u16;
        println!("Düşman Sana {} Hasar Verdi", enemy_give_damage);
        wait_for_user_input();                              
    }

    println!("Kazanılan XP: {}", xp_gain);

    let delta = player.xp + xp_gain as u16;
    if delta >= 100 {
        player.lvl += 1;
        player.xp = delta - 100;
        player.max_health += 50;
        player.max_take_damage += 50;
        println!("Xp Değeri Sınıra Ulaştı Level Değerin {} Oldu, Maksimum can değeri arttı.", player.lvl);
    }else {
        player.xp = delta;
    }

    wait_for_user_input();
    Ok(())
}

fn oyuncu_detaylari(player: &Player) {
    clear_terminal();
    println!("Name: {}\nXP: {}\nLevel: {}\nHealth: {}",
            player.name, player.xp, player.lvl, player.health);
    wait_for_user_input();
}

fn kacmak() -> bool {
    if fastrand::u16(1..=100) >= 50 {
        true
    } else {
        false
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn dinlenmek(player: &mut Player) {
    let delta = fastrand::u16(1..=player.max_health - 20);
    let yenilenecek = delta.clamp(0, player.max_health - player.health);
    player.health += yenilenecek;

    println!("Karakter dinlendi Ve {} Can Yeniledi", yenilenecek);
    wait_for_user_input();
}

fn vazgec(player: &Player) {
    let score = player.calculate_score();
    println!("Korkup Kaçtın Skorun İse {}", score);
    wait_for_user_input();
}
