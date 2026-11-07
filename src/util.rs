//! vlc model — media player.

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub path: String,
    pub duration: u64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub tracks: Vec<Track>,
    pub current: Option<String>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { tracks: vec![], current: None, next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_track(&mut self, title: &str, path: &str, duration: u64) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.tracks.push(Track { id: id.clone(), title: title.into(), path: path.into(), duration });
        id
    }

    pub fn play(&mut self, id: &str) -> bool {
        if self.tracks.iter().any(|t| t.id == id) { self.current = Some(id.into()); true } else { false }
    }

    pub fn pause(&mut self) -> bool {
        self.current.is_some()
    }

    pub fn next(&mut self) -> bool {
        if let Some(pos) = self.tracks.iter().position(|t| Some(&t.id) == self.current.as_ref()) {
            if pos + 1 < self.tracks.len() { self.current = Some(self.tracks[pos + 1].id.clone()); true } else { false }
        } else { false }
    }

    pub fn prev(&mut self) -> bool {
        if let Some(pos) = self.tracks.iter().position(|t| Some(&t.id) == self.current.as_ref()) {
            if pos > 0 { self.current = Some(self.tracks[pos - 1].id.clone()); true } else { false }
        } else { false }
    }

    pub fn list(&self) -> &[Track] { &self.tracks }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn track_play_next() {
        let mut v = Model::new();
        let a = v.add_track("A", "/tmp/a.mp4", 100);
        let b = v.add_track("B", "/tmp/b.mp4", 200);
        assert!(v.play(&a));
        assert!(v.next());
        assert!(v.prev());
        let _ = b;
    }
}
