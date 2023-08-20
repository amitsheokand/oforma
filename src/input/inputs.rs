use bevy::prelude::*;
pub struct InputPlugin;

impl Plugin for InputPlugin 
{
    fn build(&self, app: &mut App) 
    {
        app.add_system(save)
        .add_system(undo)
        .add_system(copy)
        .add_system(paste)
        .add_system(redo);
    }
}

pub fn save(input: Res<Input<KeyCode>>)
{
    if (input.pressed(KeyCode::LWin)||input.pressed(KeyCode::LControl)) && input.just_pressed(KeyCode::S)
    {
        println!("Scene Saved!");
    }
}
pub fn undo(input: Res<Input<KeyCode>>)
{
    if (input.pressed(KeyCode::LWin)||input.pressed(KeyCode::LControl)) && input.just_pressed(KeyCode::Z)
    {
        println!("Undo!");
    }
}
pub fn copy(input: Res<Input<KeyCode>>)
{
    if (input.pressed(KeyCode::LWin)||input.pressed(KeyCode::LControl)) && input.just_pressed(KeyCode::C)
    {
        println!("Copied!");
    }
}
pub fn paste(input: Res<Input<KeyCode>>)
{
    if (input.pressed(KeyCode::LWin)||input.pressed(KeyCode::LControl)) && input.just_pressed(KeyCode::V)
    {
        println!("Pasted!");
    }
}
pub fn redo(input: Res<Input<KeyCode>>)
{
    if (input.pressed(KeyCode::LWin)||input.pressed(KeyCode::LControl)) && input.just_pressed(KeyCode::Y)
    {
        println!("Redo!");
    }
}
