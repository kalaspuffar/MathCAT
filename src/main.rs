/*

*** see https://rust-cli.github.io/book/tutorial/cli-args.html#parsing-cli-arguments-with-structopt
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
*/

// const LOG_FILE: &'static str = "./MathCAT.log";
// lazy_static! {
//   static ref LOG: slog::Logger = create_log();
// }

// fn create_log() -> slog::Logger {
  // let file = OpenOptions::new()
  //   .create(true)
  //   .write(true)
  //   .truncate(true)
  //   .open(LOG_FILE)
  //   .unwrap();

  // let decorator = slog_term::PlainDecorator::new(file);
  // let drain = slog_term::FullFormat::new(decorator).build().fuse();
  // let drain = slog_async::Async::new(drain).build().fuse();

  // let logger = slog::Logger::root(drain, o!());
  // return logger;
// }

// Maybe also have this speak to test the TTS generation.
// There is a rust winapi crate that mirrors the WinPAI and has "Speak(...)" in it
fn main() {
  use libmathcat::interface::*;
  use log::*;
  use std::time::{Instant};
  env_logger::builder()
      .format_timestamp(None)
      .format_module_path(false)
      .format_indent(None)
      .format_level(false)
      .init();

  //  let expr = "
  //     <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  //     <mrow>
  //      <mrow><mo>[</mo>
  //        <mtable>
  //         <mtr>
  //          <mtd>
  //           <mn>3</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>1</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>4</mn>
  //          </mtd>
  //         </mtr>
  //         <mtr>
  //          <mtd>
  //           <mn>0</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>2</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>6</mn>
  //          </mtd>
  //         </mtr>
  //        </mtable>
  //      <mo>]</mo></mrow></mrow>
  //    </math>
  // ";

  //     let expr = "
  //      <math xmlns='http://www.w3.org/1998/Math/MathML'>
  //      <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
  //      <mo>+</mo>
  //      <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
  // </math>"";
  // let expr = "<math><mi>c</mi><mo>=</mo><mn>4</mn><mspace width=\"thinmathspace\"></mspace><mn>598</mn>
  //                 <mspace width=\"thinmathspace\"></mspace><mn>037</mn>
  //                 <mspace width=\"thinmathspace\"></mspace><mn>234</mn></math>";
  // let expr = "<math><mn>𝟏𝟐𝟑</mn></math>";
  // let expr = "<math><mo>(</mo><mrow><mn>451</mn><mo>,</mo><mn>231</mn></mrow><mo>)</mo></math>";
  let expr = "<math display='block' id='M8frgf4y-0' data-id-added='true'>
  <mrow data-changed='added' id='M8frgf4y-1' data-id-added='true'>
    <mi id='M8frgf4y-2' data-id-added='true'>x</mi>
    <mo id='M8frgf4y-3' data-id-added='true'>=</mo>
    <mfrac data-mjx-texclass='ORD' id='M8frgf4y-4' data-id-added='true'>
      <mi id='M8frgf4y-5' data-id-added='true'>t</mi>
      <mrow id='M8frgf4y-6' data-id-added='true'>
        <mn id='M8frgf4y-7' data-id-added='true'>2</mn>
        <mo data-changed='added' id='M8frgf4y-8' data-id-added='true'>&#x2062;</mo>
        <mi id='M8frgf4y-9' data-id-added='true'>a</mi>
      </mrow>
    </mfrac>
  </mrow>
 </math>";
  let instant = Instant::now();
  SetMathML(expr.to_string()).unwrap();
  SetPreference("TTS".to_string(), StringOrFloat::AsString("SSML".to_string())).unwrap();
  SetPreference("Bookmark".to_string(), StringOrFloat::AsString("true".to_string())).unwrap();

  let speech_string = GetSpokenText().unwrap();
  info!("Computed speech string:\n   '{}'", speech_string);
  // let braille_string = GetBraille().unwrap();
  // info!("Computed braille string:\n   '{}'", braille_string);
  info!("Time taken: {}ms", instant.elapsed().as_millis());
  // let instant = Instant::now();
  // let _speech_string = speak_mathml(expr);
  // info!("Time taken (second time): {}ms", instant.elapsed().as_millis());
}
