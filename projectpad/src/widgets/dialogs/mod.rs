#[macro_use]
pub mod dialog_helpers;
#[macro_use]
pub mod server_add_item_dlg;
mod auth_key_button;
mod pick_projectpad_item_button;
pub mod project_add_item_dlg;
pub mod project_poi_add_edit_dlg;
pub mod server_add_edit_dlg;
pub mod server_database_add_edit_dlg;
pub mod server_extra_user_add_edit_dlg;
pub mod server_note_add_edit_dlg;
pub mod server_poi_add_edit_dlg;
pub mod server_website_add_edit_dlg;
pub mod standard_dialogs;

pub enum ServerAddEditDialogComponent {
    Poi(relm::Component<server_poi_add_edit_dlg::ServerPoiAddEditDialog>),
    Db(relm::Component<server_database_add_edit_dlg::ServerDatabaseAddEditDialog>),
    User(relm::Component<server_extra_user_add_edit_dlg::ServerExtraUserAddEditDialog>),
    Website(relm::Component<server_website_add_edit_dlg::ServerWebsiteAddEditDialog>),
    Note(relm::Component<server_note_add_edit_dlg::ServerNoteAddEditDialog>),
}

impl ServerAddEditDialogComponent {
    fn get_widget(&self) -> &gtk::Grid {
        match self {
            ServerAddEditDialogComponent::Poi(ref x) => x.widget(),
            ServerAddEditDialogComponent::Db(ref x) => x.widget(),
            ServerAddEditDialogComponent::User(ref x) => x.widget(),
            ServerAddEditDialogComponent::Website(ref x) => x.widget(),
            ServerAddEditDialogComponent::Note(ref x) => x.widget(),
        }
    }
}

pub enum ProjectAddEditDialogComponent {
    Server(relm::Component<server_add_edit_dlg::ServerAddEditDialog>),
    ProjectPoi(relm::Component<project_poi_add_edit_dlg::ProjectPoiAddEditDialog>),
}

impl ProjectAddEditDialogComponent {
    fn get_widget(&self) -> &gtk::Grid {
        match self {
            ProjectAddEditDialogComponent::Server(ref x) => x.widget(),
            ProjectAddEditDialogComponent::ProjectPoi(ref x) => x.widget(),
        }
    }
}
