use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing YouTube Guide Navigation Menu (/guide) ===");

    let yt = Innertube::new().await?;
    let guide = yt.get_guide().await?;

    println!("\nTotal Guide Sections: {}", guide.sections.len());

    for (s_idx, section) in guide.sections.iter().enumerate() {
        let sec_title = section.title.as_deref().unwrap_or("Main Menu");
        println!("\n📁 Section [{}]: {}", s_idx + 1, sec_title);

        for (i_idx, item) in section.items.iter().enumerate() {
            let icon = item.icon_type.as_deref().unwrap_or("-");
            println!("   [{:2}] {} (Icon: {})", i_idx + 1, item.title, icon);
        }
    }

    Ok(())
}
