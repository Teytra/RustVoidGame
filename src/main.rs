use player::Player;
use input::{get_user_input, wait_for_user_input};

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

    let mut player = Player::new(name);
    loop { // oyun döngüsü
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
            2 => savas(&mut player),
            3 => { vazgec(&player); break; },
            4 => oyuncu_detaylari(&player),
            _ => ()
        }
    }

    Ok(())
}
fn savas(player: &mut Player) {
    println!("Savaş Başladı");
    let kazanilan = fastrand::u16(0..=70);
    let hasar = fastrand::u16(0..=70);
    
    if player.health <= hasar {
        let score = player.calculate_score();
        println!("{} Hasara Dayanamadı Ve Karakterin Vefat Etti kazanılan skor: {}",hasar, score);
        wait_for_user_input();
        std::process::exit(0);
    }
    player.health -= hasar;
    println!("Kazanılan XP: {}\nKaybedilen Can: {}", kazanilan, hasar);

    let delta = player.xp + kazanilan;
    if delta >= 100 {
        player.lvl += 1;
        player.xp = delta - 100;
        player.max_health += 50;
        println!("Xp Değeri Sınıra Ulaştı Level Değerin {} Oldu, Maksimum can değeri arttı.", player.lvl);
    }else {
        player.xp = delta;
    }

    wait_for_user_input();
}
fn oyuncu_detaylari(player: &Player) {
    println!("player_name: {}\nplayer_xp: {}\nplayer_level: {}\nplayer_hp: {}",
            player.name, player.xp, player.lvl, player.health);
    wait_for_user_input();
}
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
fn dinlenmek(player: &mut Player) {
    let delta = fastrand::u16(1..=80);
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