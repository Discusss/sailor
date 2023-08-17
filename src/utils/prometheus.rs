use std::{env, time::Instant};

use prometheus::{opts, Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Method},
    route::{Handler, Outcome},
    Data, Request, Response, Route,
};

pub use prometheus;
use crate::utils::auth;

pub fn configure() -> PrometheusMetrics {
    let prom = PrometheusMetrics::new();

    prom
}

//Thanks to https://github.com/sd2k/rocket_prometheus

const NAMESPACE_ENV_VAR: &str = "ROCKET_PROMETHEUS_NAMESPACE";

#[derive(Clone)]
#[must_use = "must be attached and mounted to a Rocket instance"]

pub struct PrometheusMetrics {
    http_requests_total: IntCounterVec,
    http_requests_duration_seconds: HistogramVec,
    rocket_registry: Registry,
    custom_registry: Registry,
}

impl PrometheusMetrics {
    /// Create a new [`PrometheusMetrics`].
    pub fn new() -> Self {
        Self::with_registry(Registry::new())
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn with_registry(registry: Registry) -> Self {
        let rocket_registry = Registry::new();
        let namespace = env::var(NAMESPACE_ENV_VAR).unwrap_or_else(|_| "rocket".into());

        let http_requests_total_opts =
            opts!("http_requests_total", "Total number of HTTP requests")
                .namespace(namespace.clone());
        let http_requests_total =
            IntCounterVec::new(http_requests_total_opts, &["endpoint", "method", "status"])
                .unwrap();
        let http_requests_duration_seconds_opts = opts!(
            "http_requests_duration_seconds",
            "HTTP request duration in seconds for all requests"
        )
            .namespace(namespace);
        let http_requests_duration_seconds = HistogramVec::new(
            http_requests_duration_seconds_opts.into(),
            &["endpoint", "method", "status"],
        )
            .unwrap();

        rocket_registry
            .register(Box::new(http_requests_total.clone()))
            .unwrap();
        rocket_registry
            .register(Box::new(http_requests_duration_seconds.clone()))
            .unwrap();

        Self {
            http_requests_total,
            http_requests_duration_seconds,
            rocket_registry,
            custom_registry: registry,
        }
    }

    pub fn with_default_registry() -> Self {
        Self::with_registry(prometheus::default_registry().clone())
    }

    #[must_use]
    pub const fn registry(&self) -> &Registry {
        &self.custom_registry
    }

    /// Get the `http_requests_total` metric.
    pub fn http_requests_total(&self) -> &IntCounterVec {
        &self.http_requests_total
    }

    /// Get the `http_requests_duration_seconds` metric.
    pub fn http_requests_duration_seconds(&self) -> &HistogramVec {
        &self.http_requests_duration_seconds
    }
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
struct TimerStart(Option<Instant>);

enum StatusCode {
    Standard(rocket::http::Status),
    NonStandard(String),
}

const CODE_DIGITS: &str = "\
100101102103104105106107108109110111112113114115116117118119\
120121122123124125126127128129130131132133134135136137138139\
140141142143144145146147148149150151152153154155156157158159\
160161162163164165166167168169170171172173174175176177178179\
180181182183184185186187188189190191192193194195196197198199\
200201202203204205206207208209210211212213214215216217218219\
220221222223224225226227228229230231232233234235236237238239\
240241242243244245246247248249250251252253254255256257258259\
260261262263264265266267268269270271272273274275276277278279\
280281282283284285286287288289290291292293294295296297298299\
300301302303304305306307308309310311312313314315316317318319\
320321322323324325326327328329330331332333334335336337338339\
340341342343344345346347348349350351352353354355356357358359\
360361362363364365366367368369370371372373374375376377378379\
380381382383384385386387388389390391392393394395396397398399\
400401402403404405406407408409410411412413414415416417418419\
420421422423424425426427428429430431432433434435436437438439\
440441442443444445446447448449450451452453454455456457458459\
460461462463464465466467468469470471472473474475476477478479\
480481482483484485486487488489490491492493494495496497498499\
500501502503504505506507508509510511512513514515516517518519\
520521522523524525526527528529530531532533534535536537538539\
540541542543544545546547548549550551552553554555556557558559\
560561562563564565566567568569570571572573574575576577578579\
580581582583584585586587588589590591592593594595596597598599\
600601602603604605606607608609610611612613614615616617618619\
620621622623624625626627628629630631632633634635636637638639\
640641642643644645646647648649650651652653654655656657658659\
660661662663664665666667668669670671672673674675676677678679\
680681682683684685686687688689690691692693694695696697698699\
700701702703704705706707708709710711712713714715716717718719\
720721722723724725726727728729730731732733734735736737738739\
740741742743744745746747748749750751752753754755756757758759\
760761762763764765766767768769770771772773774775776777778779\
780781782783784785786787788789790791792793794795796797798799\
800801802803804805806807808809810811812813814815816817818819\
820821822823824825826827828829830831832833834835836837838839\
840841842843844845846847848849850851852853854855856857858859\
860861862863864865866867868869870871872873874875876877878879\
880881882883884885886887888889890891892893894895896897898899\
900901902903904905906907908909910911912913914915916917918919\
920921922923924925926927928929930931932933934935936937938939\
940941942943944945946947948949950951952953954955956957958959\
960961962963964965966967968969970971972973974975976977978979\
980981982983984985986987988989990991992993994995996997998999";

#[inline]
fn status_as_str(s: &rocket::http::Status) -> &'static str {
    let offset = (s.code - 100) as usize;
    let offset = offset * 3;

    #[cfg(debug_assertions)]
    {
        &CODE_DIGITS[offset..offset + 3]
    }

    #[cfg(not(debug_assertions))]
    #[allow(unsafe_code)]
    unsafe {
        CODE_DIGITS.get_unchecked(offset..offset + 3)
    }
}

impl StatusCode {
    fn as_str(&self) -> &str {
        match self {
            Self::Standard(s) => status_as_str(s),
            Self::NonStandard(s) => s.as_str(),
        }
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        rocket::http::Status::from_code(code)
            .map_or_else(|| Self::NonStandard(code.to_string()), Self::Standard)
    }
}

#[rocket::async_trait]
impl Fairing for PrometheusMetrics {
    fn info(&self) -> Info {
        Info {
            name: "Prometheus metric collection",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        req.local_cache(|| TimerStart(Some(Instant::now())));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
        // Don't touch metrics if the request didn't match a route.
        if req.route().is_none() {
            return;
        }

        let endpoint = req.route().unwrap().uri.as_str();
        let method = req.method().as_str();
        let status = StatusCode::from(response.status().code);
        self.http_requests_total
            .with_label_values(&[endpoint, method, status.as_str()])
            .inc();

        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(duration) = start_time.0.map(|st| st.elapsed()) {
            let duration_secs = duration.as_secs_f64();
            self.http_requests_duration_seconds
                .with_label_values(&[endpoint, method, status.as_str()])
                .observe(duration_secs);
        }
    }
}

#[rocket::async_trait]
impl Handler for PrometheusMetrics {
    async fn handle<'r>(&self, req: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {

        let auth = req.headers().get_one("Authorization");
        if auth.is_none() {
            return Outcome::Failure(rocket::http::Status::Unauthorized);
        }

        let is_allowed = auth::is_authorised(auth.unwrap().to_string());
        if !is_allowed {
            return Outcome::Failure(rocket::http::Status::Unauthorized);
        }

        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        encoder
            .encode(&self.custom_registry.gather(), &mut buffer)
            .unwrap();
        encoder
            .encode(&self.rocket_registry.gather(), &mut buffer)
            .unwrap();
        let body = String::from_utf8(buffer).unwrap();
        Outcome::from(
            req,
            (
                ContentType::new("text", "plain")
                    .with_params([("version", "0.0.4"), ("charset", "utf-8")]),
                body,
            ),
        )
    }
}

impl From<PrometheusMetrics> for Vec<Route> {
    fn from(other: PrometheusMetrics) -> Self {
        vec![Route::new(Method::Get, "/", other)]
    }
}

#[cfg(test)]
mod test {
    use super::PrometheusMetrics;

    #[test]
    fn test_multiple_instantiations() {
        let _pm1 = PrometheusMetrics::with_default_registry();
        let _pm2 = PrometheusMetrics::with_default_registry();
    }
}