use async;
use error::*;
use std::sync::mpsc;
use uuid;

macro_rules! feature_default {
    () => {
        fn id(&self) -> ::uuid::Uuid {
            self.id
        }

        fn name(&self) -> &str {
            FEATURE_NAME
        }
    }
}

macro_rules! renderable_impl {
    ($name:ident) => {
        impl ::feature::Renderable for $name {
            fn render(&self) -> String {
                self.data.render()
            }
        }
    };
}

pub trait Renderable {
    fn render(&self) -> String;
}

pub trait Feature: Renderable {
    fn id(&self) -> uuid::Uuid;

    fn init_notifier(&self) -> Result<()>;

    fn name(&self) -> &str;

    fn update(&mut self) -> Result<()>;
}

pub trait FeatureConfig: Feature {
    type Settings;

    fn new(
        id: uuid::Uuid,
        tx: mpsc::Sender<async::Message>,
        settings: Self::Settings,
    ) -> Result<Self>
    where
        Self: Sized;
}
