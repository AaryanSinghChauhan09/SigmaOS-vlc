//! vlc model — media player.

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub path: String,
    pub title: String,
    pub duration: u64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub playlist: Vec<PlaylistItem>,
    pub current: Option<usize>,
    pub playing: bool,
}

impl Default for Model {
    fn default() -> Self { Self { playlist: vec![], current: None, playing: false } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, item: PlaylistItem) { self.playlist.push(item); }

    pub fn play(&mut self, index: usize) -> bool {
        if index < self.playlist.len() { self.current = Some(index); self.playing = true; true } else { false }
    }

    pub fn next(&mut self) -> bool {
        if let Some(i) = self.current {
            if i + 1 < self.playlist.len() { self.current = Some(i + 1); return true; }
        }
        false
    }

    pub fn list(&self) -> &[PlaylistItem] { &self.playlist }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_play_next() {
        let mut v = Model::new();
        v.add(PlaylistItem { path: "a.mp4".into(), title: "A".into(), duration: 100 });
        v.add(PlaylistItem { path: "b.mp4".into(), title: "B".into(), duration: 200 });
        assert!(v.play(0));
        assert!(v.next());
        assert_eq!(v.current, Some(1));
    }
}
