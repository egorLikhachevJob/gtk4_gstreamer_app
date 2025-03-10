mod custom_paintable;

// Используем необходимые модули из gstreamer и gtk4
//use gstreamer::{element_error, plugin_define, prelude::*};
use gstreamer::Pipeline;
use gstreamer::{element_error, prelude::*};
use gstreamer_app::AppSink;
// use std::thread;
// use std::time::Duration;
use custom_paintable::CustomPaintable;
use gstreamer_app::AppSinkCallbacks;
use gstreamer_video::video_meta::tags::Colorspace;
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::{glib, prelude::*};



struct Config {
    camera: CameraConfig,
}

struct CameraConfig {
    width: i32,
    height: i32,
    fps: i32,
}

// Функция для инициализации и запуска GStreamer
fn gstreamer_app() {
    // Пример инициализации CONFIG
    let config = Config {
        camera: CameraConfig {
            width: 480,
            height: 320,
            fps: 25,
        },
    };

    // Инициализируем GStreamer
    gstreamer::init().unwrap();

    // Создаем pipeline для вывода изображения с параметрами YUY2 (480x320, 25 fps)
    let pipeline_str = format!(
        "v4l2src device=/dev/video0 ! video/x-raw,format=YUY2,width={},height={},framerate={}/1 ! videoconvert ! video/x-raw,format=BGR ! appsink name=sink1",
        config.camera.width, config.camera.height, config.camera.fps,
    );
    // Парсим строку pipeline и создаем объект pipeline
    let pipeline = gstreamer::parse::launch(&pipeline_str)
        .expect(&format!("{}", "Can not create GStreamer with pipeline"));
    let pipeline = pipeline
        .dynamic_cast::<Pipeline>()
        .expect(&format!("{}", "Can not dynamic_cast pipeline"));

    // Получаем элемент appsink из pipeline
    let appsink = pipeline
        .by_name("sink1")
        .expect("Can not get appsink element")
        .dynamic_cast::<AppSink>()
        .expect("Can not dynamic_cast to AppSink");

    // appsink.set_callbacks(
    //     AppSinkCallbacks::builder()
    //         .new_sample(|_| {
    //             println!("Frame received");
    //             Ok(gstreamer::FlowSuccess::Ok)
    //         })
    //         .build(),
    // );

    // Устанавливаем обратные вызовы для appsink
    appsink.set_callbacks(
        AppSinkCallbacks::builder()
            .new_sample(|appsink| {
                let sample = appsink
                    .pull_sample()
                    .map_err(|_| gstreamer::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    element_error!(
                        appsink,
                        gstreamer::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gstreamer::FlowError::Error
                })?;

                // Читаем данные из буфера
                let map = buffer.map_readable().map_err(|_| {
                    element_error!(
                        appsink,
                        gstreamer::ResourceError::Failed,
                        ("Failed to map buffer readable")
                    );

                    gstreamer::FlowError::Error
                })?;

                // Выводим длину данных в буфере
                let samples = map.as_slice();
                println!("{}", samples.len());

                // let pixbuf = Pixbuf::new(
                //     GDK_COLORSPACE_RGB, 
                //     false,           
                //     8,               
                //     480,             
                //     320              
                // ).expect("Failed to create Pixbuf");

                Ok(gstreamer::FlowSuccess::Ok)
            })
            .build(),
    );

    // Запускаем pipeline
    pipeline.set_state(gstreamer::State::Playing).unwrap();

    // Ожидаем некоторое время, чтобы отобразить видео
    // thread::sleep(Duration::MAX);

    // Останавливаем pipeline
    // pipeline.set_state(gstreamer::State::Null).unwrap();

    // println!("Вывод изображения завершен");
}

// Функция для создания и отображения пользовательского интерфейса
fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("Custom Paintable"));
    window.set_default_size(480, 320);

    let paintable = CustomPaintable::default();

    let picture = gtk4::Picture::new();
    picture.set_halign(gtk4::Align::Center);
    picture.set_size_request(480, 320);
    picture.set_paintable(Some(&paintable));

    window.set_child(Some(&picture));
    window.present();
}

fn main() -> glib::ExitCode {
    gstreamer_app();
    let application = gtk4::Application::builder().build();
    application.connect_activate(build_ui);
    application.run()
}
