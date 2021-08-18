// TODO:
// - Successfully render map

pub struct BattleScreen {
    scene: Scene,
}

impl Screen for BattleScreen {
    fn update(&mut self, duration: Duration) -> AResult<StackCommand> {
        // TODO: This should, if on enemy turn, execute a series of action by
        // the AI
        self.scene.tick(duration);
        Ok(StackCommand::None)
    }

    fn draw(&self) -> AResult {
        // TODO
        self.scene.draw();
        Ok(())
    }

    fn click(&mut self, pos: Vec2) -> AResult<StackCommand> {
        // TODO: This should handle click if turn is player turn
        Ok(StackCommand::None)
    }

    fn resize(&mut self, aspect_ratio: f32) {
        // TODO
    }

    fn move_mouse(&mut self, pos: Vec2) -> AResult {
        // TODO
        Ok(())
    }
}

