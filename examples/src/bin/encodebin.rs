// This example demonstrates the use of the encodebin element.
// The example takes an arbitrary URI as input, which it will try to decode
// and finally reencode using the encodebin element.
// For more information about how the decodebin element works, have a look at
// the decodebin-example.
// Since we tell the encodebin what format we want to get out of it from the start,
// it provides the correct caps and we can link it before starting the pipeline.
// After the decodebin has found all streams and we piped them into the encodebin,
// the operated pipeline looks as follows:

//                  /-{queue}-{audioconvert}-{audioresample}-\
// {uridecodebin} -|                                          {encodebin}-{filesink}
//                  \-{queue}-{videoconvert}-{videoscale}----/

use gst::element_error;
use gst::element_warning;

use gst_pbutils::prelude::*;

#[cfg(feature = "v1_10")]
use glib::GBoxed;

use std::env;
#[cfg(feature = "v1_10")]
use std::sync::{Arc, Mutex};

use anyhow::Error;
use derive_more::{Display, Error};

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

#[cfg(feature = "v1_10")]
#[derive(Clone, Debug, GBoxed)]
#[gboxed(type_name = "ErrorValue")]
struct ErrorValue(Arc<Mutex<Option<Error>>>);

fn configure_encodebin(encodebin: &gst::Element) -> Result<(), Error> {
    // To tell the encodebin what we want it to produce, we create an EncodingProfile
    // https://gstreamer.freedesktop.org/data/doc/gstreamer/head/gst-plugins-base-libs/html/GstEncodingProfile.html
    // This profile consists of information about the contained audio and video formats
    // as well as the container format we want everything to be combined into.

    // Every audiostream piped into the encodebin should be encoded using vorbis.
    let audio_profile = gst_pbutils::EncodingAudioProfileBuilder::new()
        .format(&gst::Caps::new_simple("audio/x-vorbis", &[]))
        .presence(0)
        .build()?;

    // Every videostream piped into the encodebin should be encoded using theora.
    let video_profile = gst_pbutils::EncodingVideoProfileBuilder::new()
        .format(&gst::Caps::new_simple("video/x-theora", &[]))
        .presence(0)
        .build()?;

    // All streams are then finally combined into a matroska container.
    let container_profile = gst_pbutils::EncodingContainerProfileBuilder::new()
        .name("container")
        .format(&gst::Caps::new_simple("video/x-matroska", &[]))
        .add_profile(&(video_profile))
        .add_profile(&(audio_profile))
        .build()?;

    // Finally, apply the EncodingProfile onto our encodebin element.
    encodebin
        .set_property("profile", &container_profile)
        .expect("set profile property failed");

    Ok(())
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str;
    let output_file: &str;

    if args.len() == 3 {
        uri = args[1].as_ref();
        output_file = args[2].as_ref();
    } else {
        println!("Usage: encodebin URI output_file");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("uridecodebin", None)
        .map_err(|_| MissingElement("uridecodebin"))?;
    let encodebin =
        gst::ElementFactory::make("encodebin", None).map_err(|_| MissingElement("encodebin"))?;
    let sink =
        gst::ElementFactory::make("filesink", None).map_err(|_| MissingElement("filesink"))?;

    src.set_property("uri", &uri)
        .expect("setting URI Property failed");
    sink.set_property("location", &output_file)
        .expect("setting location property failed");

    // Configure the encodebin.
    // Here we tell the bin what format we expect it to create at its output.
    configure_encodebin(&encodebin)?;

    pipeline
        .add_many(&[&src, &encodebin, &sink])
        .expect("failed to add elements to pipeline");
    // It is clear from the start, that encodebin has only one src pad, so we can
    // directly link it to our filesink without problems.
    // The caps of encodebin's src-pad are set after we configured the encoding-profile.
    // (But filesink doesn't really care about the caps at its input anyway)
    gst::Element::link_many(&[&encodebin, &sink])?;

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Much of the following is the same code as in the decodebin example
    // so if you want more information on that front, have a look there.
    src.connect_pad_added(move |dbin, dbin_src_pad| {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };

        let (is_audio, is_video) = {
            let media_type = dbin_src_pad.current_caps().and_then(|caps| {
                caps.structure(0).map(|s| {
                    let name = s.name();
                    (name.starts_with("audio/"), name.starts_with("video/"))
                })
            });

            match media_type {
                None => {
                    element_warning!(
                        dbin,
                        gst::CoreError::Negotiation,
                        ("Failed to get media type from pad {}", dbin_src_pad.name())
                    );

                    return;
                }
                Some(media_type) => media_type,
            }
        };

        let link_to_encodebin = |is_audio, is_video| -> Result<(), Error> {
            if is_audio {
                let queue = gst::ElementFactory::make("queue", None)
                    .map_err(|_| MissingElement("queue"))?;
                let convert = gst::ElementFactory::make("audioconvert", None)
                    .map_err(|_| MissingElement("audioconvert"))?;
                let resample = gst::ElementFactory::make("audioresample", None)
                    .map_err(|_| MissingElement("audioresample"))?;

                let elements = &[&queue, &convert, &resample];
                pipeline
                    .add_many(elements)
                    .expect("failed to add audio elements to pipeline");
                gst::Element::link_many(elements)?;

                // Request a sink pad from our encodebin, that can handle a raw audiostream.
                // The encodebin will then automatically create an internal pipeline, that encodes
                // the audio stream in the format we specified in the EncodingProfile.
                let enc_sink_pad = encodebin
                    .request_pad_simple("audio_%u")
                    .expect("Could not get audio pad from encodebin");
                let src_pad = resample.static_pad("src").expect("resample has no srcpad");
                src_pad.link(&enc_sink_pad)?;

                for e in elements {
                    e.sync_state_with_parent()?;
                }

                // Get the queue element's sink pad and link the decodebin's newly created
                // src pad for the audio stream to it.
                let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                dbin_src_pad.link(&sink_pad)?;
            } else if is_video {
                let queue = gst::ElementFactory::make("queue", None)
                    .map_err(|_| MissingElement("queue"))?;
                let convert = gst::ElementFactory::make("videoconvert", None)
                    .map_err(|_| MissingElement("videoconvert"))?;
                let scale = gst::ElementFactory::make("videoscale", None)
                    .map_err(|_| MissingElement("videoscale"))?;

                let elements = &[&queue, &convert, &scale];
                pipeline
                    .add_many(elements)
                    .expect("failed to add video elements to pipeline");
                gst::Element::link_many(elements)?;

                // Request a sink pad from our encodebin, that can handle a raw videostream.
                // The encodebin will then automatically create an internal pipeline, that encodes
                // the audio stream in the format we specified in the EncodingProfile.
                let enc_sink_pad = encodebin
                    .request_pad_simple("video_%u")
                    .expect("Could not get video pad from encodebin");
                let src_pad = scale.static_pad("src").expect("videoscale has no srcpad");
                src_pad.link(&enc_sink_pad)?;

                for e in elements {
                    e.sync_state_with_parent()?
                }

                // Get the queue element's sink pad and link the decodebin's newly created
                // src pad for the video stream to it.
                let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                dbin_src_pad.link(&sink_pad)?;
            }

            Ok(())
        };

        if let Err(err) = link_to_encodebin(is_audio, is_video) {
            #[cfg(feature = "v1_10")]
            element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                details: gst::Structure::builder("error-details")
                            .field("error",
                                   &ErrorValue(Arc::new(Mutex::new(Some(err)))))
                            .build()
            );

            #[cfg(not(feature = "v1_10"))]
            element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                ["{}", err]
            );
        }
    });

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;

                #[cfg(feature = "v1_10")]
                {
                    match err.details() {
                        Some(details) if details.name() == "error-details" => details
                            .get::<&ErrorValue>("error")
                            .unwrap()
                            .clone()
                            .0
                            .lock()
                            .unwrap()
                            .take()
                            .map(Result::Err)
                            .expect("error-details message without actual error"),
                        _ => Err(ErrorMessage {
                            src: msg
                                .src()
                                .map(|s| String::from(s.path_string()))
                                .unwrap_or_else(|| String::from("None")),
                            error: err.error().to_string(),
                            debug: err.debug(),
                            source: err.error(),
                        }
                        .into()),
                    }?;
                }
                #[cfg(not(feature = "v1_10"))]
                {
                    return Err(ErrorMessage {
                        src: msg
                            .src()
                            .map(|s| String::from(s.path_string()))
                            .unwrap_or_else(|| String::from("None")),
                        error: err.error().to_string(),
                        debug: err.debug(),
                        source: err.error(),
                    }
                    .into());
                }
            }
            MessageView::StateChanged(s) => {
                println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.src().map(|s| s.path_string()),
                    s.old(),
                    s.current(),
                    s.pending()
                );
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
