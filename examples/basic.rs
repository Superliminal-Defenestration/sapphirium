pub struct SomeCollection;

pub struct SomeOtherCollection;


#[derive(Default)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    End,
}


impl Collection for SomeCollection {
    fn init(&mut self, app: &mut App) {
        // init code
        // init components and systems
    }
}

impl Component for SomeOtherCollection {
    fn init(&mut self, app: &mut App) {

    }
}

pub fn some_system() {
    
}

fn main() {
    let mut app = App::new();

    app.register_collection::<SomeCollection>()
        .register_collection::<SomeOtherCollection>()
        .register_system::<some_system>()
        .register_component::<>
        .add_state(GameState)
        .add_resource::<SomeResource>();
}
