use notify_rust::Notification;

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instance";

#[cfg(target_os = "windows")]
static SOUND: &str = "Mail";

pub fn show_initialized_service() { 
   let _notification_result = Notification::new()
    .summary("Service Started!")
    .body("I will keep track of your battery progress!")
    .sound_name(SOUND)
    .show()
    .unwrap(); 
}

pub fn show_almost_charged() {
   let _notification_result = Notification::new()
    .summary("Almost Carged!")
    .body("I do suggest to plug your laptop off, but if you want a complete charge it could take a while")
    .sound_name(SOUND)
    .show()
    .unwrap(); 
}

pub fn show_fully_charged() {
   let _notification_result = Notification::new()
    .summary("Fully Carged!")
    .body("Your laptop is fully charged!")
    .sound_name(SOUND)
    .show()
    .unwrap(); 
}