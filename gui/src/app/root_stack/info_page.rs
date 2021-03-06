mod vulkan_info;

use daemon::gpu_controller::GpuInfo;
use gtk::*;
use vulkan_info::VulkanInfoFrame;

#[derive(Clone)]
pub struct InformationPage {
    pub container: Grid,
    gpu_name_label: Label,
    gpu_manufacturer_label: Label,
    vbios_version_label: Label,
    driver_label: Label,
    vram_size_label: Label,
    link_speed_label: Label,
    vulkan_info_frame: VulkanInfoFrame,
}

impl InformationPage {
    pub fn new() -> Self {
        let container = Grid::new();

        container.set_margin_start(5);
        container.set_margin_end(5);
        container.set_margin_bottom(5);
        container.set_margin_top(5);

        container.set_column_homogeneous(true);

        container.set_row_spacing(7);
        container.set_column_spacing(5);

        container.attach(
            &{
                let label = Label::new(Some("GPU Model:"));
                label.set_halign(Align::End);
                label
            },
            0,
            0,
            2,
            1,
        );

        let gpu_name_label = Label::new(None);
        gpu_name_label.set_halign(Align::Start);

        container.attach(&gpu_name_label, 2, 0, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("GPU Manufacturer:"));
                label.set_halign(Align::End);
                label
            },
            0,
            1,
            2,
            1,
        );

        let gpu_manufacturer_label = Label::new(None);
        gpu_manufacturer_label.set_halign(Align::Start);

        container.attach(&gpu_manufacturer_label, 2, 1, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("VBIOS Version:"));
                label.set_halign(Align::End);
                label
            },
            0,
            2,
            2,
            1,
        );

        let vbios_version_label = Label::new(None);
        vbios_version_label.set_halign(Align::Start);

        container.attach(&vbios_version_label, 2, 2, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("Driver in use:"));
                label.set_halign(Align::End);
                label
            },
            0,
            3,
            2,
            1,
        );

        let driver_label = Label::new(None);
        driver_label.set_halign(Align::Start);

        container.attach(&driver_label, 2, 3, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("VRAM Size:"));
                label.set_halign(Align::End);
                label
            },
            0,
            4,
            2,
            1,
        );

        let vram_size_label = Label::new(None);
        vram_size_label.set_halign(Align::Start);

        container.attach(&vram_size_label, 2, 4, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("Link speed:"));
                label.set_halign(Align::End);
                label
            },
            0,
            5,
            2,
            1,
        );

        let link_speed_label = Label::new(None);
        link_speed_label.set_halign(Align::Start);

        container.attach(&link_speed_label, 2, 5, 3, 1);

        let vulkan_info_frame = VulkanInfoFrame::new();
        container.attach(&vulkan_info_frame.container, 0, 6, 5, 1);

        Self {
            container,
            gpu_name_label,
            gpu_manufacturer_label,
            vbios_version_label,
            driver_label,
            vram_size_label,
            link_speed_label,
            vulkan_info_frame,
        }
    }

    pub fn set_info(&self, gpu_info: &GpuInfo) {
        self.gpu_name_label.set_markup(&format!(
            "<b>{}</b>",
            match &gpu_info.vendor_data.card_model {
                Some(card_model) => card_model.clone(),
                None => gpu_info.vendor_data.gpu_model.clone().unwrap_or_default(),
            }
        ));
        self.gpu_manufacturer_label.set_markup(&format!(
            "<b>{}</b>",
            gpu_info.vendor_data.card_vendor.clone().unwrap_or_default()
        ));
        self.vbios_version_label
            .set_markup(&format!("<b>{}</b>", gpu_info.vbios_version));
        self.driver_label
            .set_markup(&format!("<b>{}</b>", gpu_info.driver));
        self.vram_size_label
            .set_markup(&format!("<b>{}</b>", gpu_info.vram_size));
        self.link_speed_label.set_markup(&format!(
            "<b>{} x{}</b>",
            gpu_info.link_speed, gpu_info.link_width
        ));

        self.vulkan_info_frame.set_info(&gpu_info.vulkan_info);

        self.container.show_all();
    }
}
