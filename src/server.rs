use crate::am03127::page_content::formatting::{Clock as ClockFormat, ColumnStart, Font};
use crate::am03127::page_content::{Lagging, Leading, WaitingModeAndSpeed};
use crate::am03127::realtime_clock::RealTimeClock;
use crate::{WEB_TASK_POOL_SIZE, am03127::page_content::PageContent, uart::Uart};
use core::convert::From;
use core::fmt::Write;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Duration;
use heapless::String;
use picoserve::extract::Json;
use picoserve::routing::post;
use picoserve::{
    AppRouter, AppWithStateBuilder,
    extract::State,
    routing::{PathRouter, get},
};
use serde::Deserialize;

const JSON_DESERIALIZE_BUFFER_SIZE: usize = 128;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct Clock {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub month: u8,
    pub second: u8,
    pub year: u8,
}

impl From<Clock> for RealTimeClock {
    fn from(clock: Clock) -> Self {
        RealTimeClock::default()
            .year(clock.year)
            .month(clock.month)
            .day(clock.day)
            .hour(clock.hour)
            .minute(clock.minute)
            .second(clock.second)
    }
}

impl From<FormattedText> for PageContent {
    fn from(formatted_text: FormattedText) -> Self {
        PageContent::default()
            .leading(formatted_text.leading)
            .lagging(formatted_text.lagging)
            .waiting_mode_and_speed(formatted_text.waiting_mode_and_speed)
            .message(&formatted_text.text)
    }
}
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct FormattedText {
    pub text: String<32>,
    #[serde(default)]
    pub leading: Leading,
    #[serde(default)]
    pub lagging: Lagging,
    #[serde(default)]
    pub waiting_mode_and_speed: WaitingModeAndSpeed,
}

#[derive(Clone, Copy)]
pub struct SharedUart(pub &'static Mutex<CriticalSectionRawMutex, Uart<'static>>);

#[derive(Clone)]
pub struct AppState {
    pub shared_uart: SharedUart,
}

impl picoserve::extract::FromRef<AppState> for SharedUart {
    fn from_ref(state: &AppState) -> Self {
        state.shared_uart
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppProps;

impl AppWithStateBuilder for AppProps {
    type State = AppState;
    type PathRouter = impl PathRouter<AppState>;

    fn build_app(self) -> picoserve::Router<Self::PathRouter, Self::State> {
        picoserve::Router::new()
            .route(
                "/clock",
                get(
                    |State(SharedUart(shared_uart)): State<SharedUart>| async move {
                        log::info!("Display clock");

                        let mut message = String::<64>::new();
                        write!(
                            &mut message,
                            "{}{}{}{}",
                            ClockFormat::Time,
                            Font::Narrow,
                            ColumnStart(41),
                            ClockFormat::Date
                        )
                        .unwrap();

                        let command = PageContent::default().message(&message.as_str()).command();
                        shared_uart
                            .lock()
                            .await
                            .write(command.as_bytes())
                            .await
                            .unwrap();
                    },
                )
                .post(
                    |State(SharedUart(shared_uart)): State<SharedUart>,
                     Json::<Clock, JSON_DESERIALIZE_BUFFER_SIZE>(clock)| async move {
                        log::info!("Set clock");

                        let command = RealTimeClock::from(clock).command();

                        shared_uart
                            .lock()
                            .await
                            .write(command.as_bytes())
                            .await
                            .unwrap();
                    },
                ),
            )
            .route(
                "/text",
                post(
                    |State(SharedUart(shared_uart)): State<SharedUart>,
                     Json::<FormattedText, JSON_DESERIALIZE_BUFFER_SIZE>(formatted_text)| async move {
                        log::info!("Setting Panel text");

                        let command = PageContent::from(formatted_text).command();

                        shared_uart
                            .lock()
                            .await
                            .write(command.as_bytes())
                            .await
                            .unwrap();
                    },
                ),
            )
    }
}

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: embassy_net::Stack<'static>,
    app: &'static AppRouter<AppProps>,
    config: &'static picoserve::Config<Duration>,
    state: AppState,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve_with_state(
        id,
        app,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
        &state,
    )
    .await
}
