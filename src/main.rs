mod game;
use game::*;


fn main() {
    let players = vec![Player::new(), Player::new()];
    let mut game = Game::new(players);
    game.start();

    print!("PLAYERS: ");
    println!("{:?}", game.players[0]);
    println!("{:?}", game.players[0].hand.len());
    println!("");
    println!("{:?}", game.played);

    game.play(0, 0, CardColor::None);
    println!("{:?}", game.players[0]);
    println!("{:?}", game.players[0].hand.len());
    println!("");
    println!("{:?}", game.played);


}
