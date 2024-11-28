use super::message::{Message, MessageKind};
use crate::move_mod::{self, Move};
use crate::server::reciever::recieve_message;
use crate::{combatant, Battle};
use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/*
An instance of the Handler is created by whoever hosts the game, and it listens for incoming connections.
The handler will then create a stream for each client that connects and will listen for messages from them.
Afterward, the handler will process the messages and send them to the battle object.
*/

pub struct Handler {
    pub ipaddr: String,
    pub port: u16,
    pub listener: TcpListener,
    pub battle: Arc<Mutex<Battle>>,
    pub clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
    pub next_client_id: Arc<Mutex<usize>>,
}

impl Handler {
    pub fn new(ipaddr: String, port: u16, battle: Arc<Mutex<Battle>>) -> Arc<Self> {
        let listener = TcpListener::bind(format!("{}:{}", ipaddr, port))
            .unwrap_or_else(|e| panic!("Failed to bind to address: {}", e));
        println!("Server listening on {}:{}", ipaddr, port);

        Arc::new(Handler {
            ipaddr,
            port,
            listener,
            battle,
            clients: Arc::new(Mutex::new(HashMap::new())),
            next_client_id: Arc::new(Mutex::new(0)),
        })
    }

    pub fn run_server(self: Arc<Self>) {
        // Accept incoming connections in a loop
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Generate a unique client ID
                    let mut id_lock = self.next_client_id.lock().unwrap();
                    let client_id = *id_lock;
                    *id_lock += 1;
                    drop(id_lock); // Release the lock early

                    // Add the client to the clients map
                    {
                        let mut clients_lock = self.clients.lock().unwrap();
                        clients_lock.insert(
                            client_id,
                            Arc::new(Mutex::new(stream.try_clone().unwrap())),
                        );
                    }

                    // Clone self to move into thread
                    let handler_clone = Arc::clone(&self);

                    // Spawn a new thread to handle the connection
                    thread::spawn(move || {
                        // Handle the client in a separate function
                        Handler::handle_client(handler_clone, client_id, stream);
                    });
                }
                Err(e) => {
                    println!("Failed to accept client: {}", e);
                }
            }
        }
    }

    fn handle_client(handler: Arc<Self>, client_id: usize, stream: TcpStream) {
        loop {
            // Receive messages from the client
            let message_tuple = recieve_message(&stream);
            match message_tuple {
                Ok((msg, _)) => {
                    println!("Received message from client {}: {:?}", client_id, msg.kind);
                    match &msg.kind {
                        MessageKind::Action { .. } => {
                            handler.handle_move_message(&msg);
                            // Optionally, send a response to the client
                        }
                        MessageKind::Event { code } => {
                            println!("Event code: {}", code);
                        }
                        MessageKind::Text { text } => {
                            println!("{}", text);
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Failed to receive message from client {}: {}",
                        client_id, e
                    );
                    // Remove client from clients map
                    {
                        let mut clients_lock = handler.clients.lock().unwrap();
                        clients_lock.remove(&client_id);
                    }
                    break; // Exit the loop if there's an error
                }
            }
        }
    }

    fn handle_move_message(&self, msg: &Message) {
        match &msg.kind {
            MessageKind::Action { name, user, target } => {
                println!("{} used {} on {}", user, name, target);
                let mut battle_guard = self.battle.lock().unwrap();
                // Match the user and target to the combatants in the battle
                let user_combatant = self.match_combatant(user);
                let target_combatant = self.match_combatant(target);
                // Match the move to the move in the move dictionary
                let mov = self.match_move(name);
                // Process the move
                battle_guard.process_server_move(
                    mov,
                    user_combatant.unwrap(),
                    target_combatant.unwrap(),
                );

                // After processing, broadcast the updated battle state
                // You need to define how to serialize the battle state into a Message
                let update_msg = Message::new_battle_update(&*battle_guard);
                if let Err(e) = self.broadcast_message(&update_msg) {
                    println!("Failed to broadcast message: {}", e);
                }
            }
            _ => panic!("Invalid message type"),
        }
    }

    fn match_combatant(
        &self,
        name: &str,
    ) -> Option<Arc<Mutex<dyn combatant::Combatant>>> {
        let battle_guard = self.battle.lock().unwrap();
        for combatant in battle_guard
            .player_team
            .iter()
            .chain(battle_guard.enemy_team.iter())
        {
            let combatant_guard = combatant.lock().unwrap();
            if combatant_guard.name() == name {
                return Some(Arc::clone(&combatant));
            }
        }
        None
    }

    fn match_move(&self, name: &str) -> Arc<Move> {
        move_mod::MOVE_DICTIONARY.get(name).unwrap().clone()
    }

    pub fn send_message(&self, client_id: usize, msg: &Message) -> Result<(), std::io::Error> {
        let serialized = msg.serialize().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Serialization error: {}", e))
        })?;
        let clients_lock = self.clients.lock().unwrap();
        if let Some(stream_arc) = clients_lock.get(&client_id) {
            let mut stream = stream_arc.lock().unwrap();
            stream.write_all(&serialized)?;
            stream.flush()?;
        } else {
            println!("Client {} not found", client_id);
        }
        Ok(())
    }
    
    pub fn broadcast_message(&self, msg: &Message) -> Result<(), std::io::Error> {
        let serialized = msg.serialize().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Serialization error: {}", e))
        })?;
        let clients_lock = self.clients.lock().unwrap();
        for (client_id, stream_arc) in clients_lock.iter() {
            let mut stream = stream_arc.lock().unwrap();
            match stream.write_all(&serialized) {
                Ok(_) => {
                    stream.flush()?;
                }
                Err(e) => {
                    println!("Failed to send message to client {}: {}", client_id, e);
                }
            }
        }
        Ok(())
    }
    }
