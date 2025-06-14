use pelican_ui::Context;
use pelican_ui::drawable::Align;
use profiles::components::AvatarContentProfiles;
use profiles::service::{Profiles, Name};
use profiles::OrangeName;

use pelican_ui_std::{
    IconButton, Header, Row,
    Offset, Padding, Size,
    AvatarContent, Column,
    AvatarRow, Text, TextStyle,
    HeaderContent, HeaderIcon,
};


pub struct HeaderMessages;

impl HeaderMessages {
    pub fn new(ctx: &mut Context, left: Option<IconButton>, right: Option<IconButton>, profiles: Vec<OrangeName>) -> Header {
        Header::new(HeaderIcon::new(left), HeaderContentMessages::new(ctx, profiles), HeaderIcon::new(right))
    }
}

pub struct HeaderContentMessages;

impl HeaderContentMessages {
    pub fn new(ctx: &mut Context, profiles: Vec<OrangeName>) -> HeaderContent {
        let profiles = profiles.iter().map(|orange_name| { 
            let profiles = ctx.state().get::<Profiles>();
            let profile = profiles.0.get(orange_name).unwrap();
            let username = profile.get("username").unwrap();
            (username.to_string(), AvatarContentProfiles::from_orange_name(ctx, &orange_name))
        }).collect::<Vec<_>>();

        let text_size = ctx.theme.fonts.size.h5;
        let title = if profiles.len() == 1 {&profiles[0].0.clone()} else {"Group Message"};
        let avatars = profiles.into_iter().map(|p| p.1).collect::<Vec<AvatarContent>>();
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent::new(
            Some(AvatarRow::new(ctx, avatars)),
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
        )
    }
}