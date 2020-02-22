struct NetworkInfo {}

#[derive(Copy, Clone, Debug, PartialEq)]
/// The various states a RMercury session can be run in.
pub enum MercuryType {
    /// Start a new Peer 2 Peer session
    Peer2Peer,
    /// Start spectating a game in progress
    Spectator,
    /// Start a session where each frame is rolled back, and then the serialized game state is checked against it. Sucessful runs means no non-deterministic behavior was introduced.
    SyncTest,
    /// Start a replay of a previous match
    Replay,
    /// Start a session where packet loss is simulated to extremes.
    PacketLoss,
}

pub struct RMercuryBuilder {
    m_type: Option<MercuryType>,
    number_of_players: usize,
    number_of_spectators: usize,
}

impl RMercuryBuilder {
    /// Create a new RMercuryBuilder to initialize the network settings.
    pub fn new() -> Self {
        return Self {
            m_type: None,
            number_of_players: 2,
            number_of_spectators: 8,
        };
    }

    /// Sets the type of the network session.
    pub fn with_type(mut self, m_type: MercuryType) -> Self {
        self.m_type = Some(m_type);

        return self;
    }

    /// Sets the number of players for the network session.
    pub fn with_players(mut self, num_players: usize) -> Self {
        self.number_of_players = num_players;
        return self;
    }

    /// Build the configured RMercury instance.
    pub fn build(&self) -> RMercury {
        if self.m_type.is_none() {
            panic!("type not set");
        }

        // TODO: ensure it's a valid config
        //TODO: init the different instances
        let rm = RMercury {
            m_type: self.m_type.unwrap(),
        };
        return rm;
    }
}

pub struct RMercury {
    pub m_type: MercuryType,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn it_should_panic_if_not_built() {
        let mut merc = RMercuryBuilder::new();
        let r_mercury = merc.build();

        assert_eq!(true, true);
    }

    #[test]
    fn it_should_build_with_proper_type() {
        let mut merc = RMercuryBuilder::new();
        let r_mercury = merc
            .with_type(MercuryType::Peer2Peer)
            .with_type(MercuryType::Spectator)
            .build();

        assert_eq!(MercuryType::Spectator, r_mercury.m_type);
    }
}
