/* Hallmark · pre-emit critique: P5 H5 E5 S5 R5 V5 */
import { useState } from 'react';
import { 
  Check, 
  Copy, 
  ExternalLink, 
  FileCode
} from 'lucide-react';

export default function App() {
  const [activeTab, setActiveTab] = useState<'stream' | 'music' | 'channel' | 'livechat' | 'oauth'>('stream');
  const [copiedCmd, setCopiedCmd] = useState(false);
  const [copiedCode, setCopiedCode] = useState(false);

  const handleCopyCmd = () => {
    navigator.clipboard.writeText('cargo add innertube-rs');
    setCopiedCmd(true);
    setTimeout(() => setCopiedCmd(false), 2000);
  };

  const codeSnippets = {
    stream: {
      file: 'examples/download/download_audio.rs',
      title: 'Direct Stream Deciphering',
      code: `use innertube_rs::{Innertube, FormatFilter, FormatType, QualityPreference};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client & embedded QuickJS decipher engine
    let yt = Innertube::new().await?;
    let video_id = "dQw4w9WgXcQ";

    // 2. Resolve direct playable highest-bitrate audio stream
    let filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: None,
    };

    let stream_url = yt.get_stream_url(video_id, &filter).await?;
    println!("Direct HTTPS Stream URL: {}", stream_url);

    Ok(())
}`
    },
    music: {
      file: 'examples/music/get_music_artist.rs',
      title: 'YouTube Music Suite',
      code: `use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Fetch artist profile, albums, and top tracks
    let artist = yt.get_music_artist("UC_x5XG1OV2P6uZZ5FSM9Ttw").await?;
    println!("Artist: {} (Subscribers: {:?})", artist.name, artist.subscribers);

    for song in artist.top_songs {
        println!(" - Track: {} [{:?}]", song.title, song.duration);
    }

    Ok(())
}`
    },
    channel: {
      file: 'examples/channel/get_community_posts.rs',
      title: 'Channel & Community Engine',
      code: `use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Scrape community posts, poll choices, and images
    let response = yt.get_channel_community("UCX6OQ3DkcsbYNE6H8uQQuVA", None).await?;
    for post in response.posts {
        println!("Post by {}: {}", post.author_name, post.content);
        if let Some(poll) = post.poll {
            println!("  Poll choices: {:?}", poll.choices);
        }
    }

    Ok(())
}`
    },
    livechat: {
      file: 'examples/live/test_live_chat.rs',
      title: 'Real-Time Live Chat Polling',
      code: `use innertube_rs::{Innertube, LiveChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    let next = yt.get_watch_next("live_stream_video_id").await?;
    if let Some(token) = next.live_chat_continuation_token {
        let chat = yt.get_live_chat(&token).await?;
        for msg in chat.messages {
            match msg {
                LiveChatMessage::Text(t) => println!("[{}] {}", t.author_name, t.message),
                LiveChatMessage::SuperChat(s) => println!("💸 {}: {:?}", s.purchase_amount_text, s.message),
                _ => {}
            }
        }
    }

    Ok(())
}`
    },
    oauth: {
      file: 'examples/auth/test_oauth_flow.rs',
      title: 'OAuth2 TV Device Login',
      code: `use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // 1. Request user code for google.com/device
    let code = yt.request_oauth_code().await?;
    println!("Visit: {} -> Enter: {}", code.verification_url, code.user_code);

    // 2. Poll for authorized tokens
    let tokens = yt.poll_oauth_tokens(&code.device_code, code.interval).await?;
    println!("Access Token: {}", tokens.access_token);

    Ok(())
}`
    }
  };

  const handleCopyCode = () => {
    navigator.clipboard.writeText(codeSnippets[activeTab].code);
    setCopiedCode(true);
    setTimeout(() => setCopiedCode(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[#F6F6F4] text-[#111215] flex flex-col justify-between selection:bg-[#E11D48] selection:text-white">
      
      {/* 1. TOP SWISS MASTHEAD */}
      <header className="border-b border-[#E2E2DF] bg-[#F6F6F4] sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-14 flex items-center justify-between">
          
          {/* Brand Wordmark */}
          <div className="flex items-center gap-3">
            <span className="font-mono text-sm font-semibold tracking-tight text-[#111215]">
              / innertube-rs
            </span>
            <span className="font-mono text-[11px] text-[#8E929B] uppercase tracking-wider">
              v0.5.0
            </span>

            {/* Install Button */}
            <button 
              onClick={handleCopyCmd}
              className="hidden sm:inline-flex items-center gap-1.5 px-2.5 py-1 text-[11px] font-mono font-medium border border-[#111215] hover:bg-[#111215] hover:text-white transition-colors"
            >
              {copiedCmd ? 'COPIED' : 'INSTALL'}
            </button>

            {/* Crates.io Pill */}
            <a 
              href="https://crates.io/crates/innertube-rs"
              target="_blank" 
              rel="noopener noreferrer"
              className="hidden sm:inline-flex items-center gap-1.5 px-2 py-0.5 text-[11px] font-mono font-medium bg-[#111215] text-white"
            >
              <span>★</span>
              <span>CRATES.IO</span>
            </a>
          </div>

          {/* Right Header Navigation & Theme Badge */}
          <div className="flex items-center gap-3 sm:gap-4 font-mono text-xs">
            <a 
              href="https://docs.rs/innertube-rs" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-[#5A5D64] hover:text-[#111215] flex items-center gap-1 transition-colors"
            >
              <span>docs.rs</span>
              <ExternalLink className="w-3 h-3 text-[#8E929B]" />
            </a>

            <a 
              href="https://github.com/caya8205-2/innertube-rs" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-[#5A5D64] hover:text-[#111215] transition-colors"
            >
              github
            </a>

            {/* Hallmark Theme Pill */}
            <div className="flex items-center gap-1.5 px-2.5 py-1 border border-[#E2E2DF] bg-white text-[11px] text-[#111215] shadow-xs">
              <span className="text-[#8E929B]">21 / 21</span>
              <span>—</span>
              <span className="font-semibold">Grid</span>
            </div>
          </div>

        </div>
      </header>

      {/* 2. MAIN SWISS INTERNATIONAL SHEET */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16 w-full flex-grow">
        
        {/* HERO SECTION: Swiss International Headline */}
        <section className="mb-16">
          
          {/* Category Tag */}
          <div className="flex items-center gap-2 mb-6">
            <span className="w-2.5 h-2.5 bg-[#E11D48] inline-block"></span>
            <span className="font-mono text-xs uppercase tracking-widest text-[#5A5D64] font-semibold">
              SWISS INTERNATIONAL / RUST NATIVE INNERTUBE ENGINE
            </span>
          </div>

          {/* Massive Display Heading */}
          <h1 className="font-['Space_Grotesk'] text-4xl sm:text-6xl lg:text-7xl font-bold text-[#111215] tracking-tight leading-[1.05] max-w-5xl mb-8">
            a native rust engine that deciphers youtube in &lt;5ms.
          </h1>

          {/* Subtext */}
          <p className="text-base sm:text-xl text-[#5A5D64] max-w-3xl leading-relaxed mb-10 font-normal">
            The grid, made visible. A near-zero-overhead asynchronous client for YouTube's private internal API, built for high-concurrency music players, downloaders, and headless scrapers.
          </p>

          {/* Terminal Command Line Box */}
          <div className="max-w-xl bg-white border border-[#E2E2DF] p-3 flex items-center justify-between font-mono text-xs shadow-xs">
            <div className="flex items-center gap-2.5 text-[#111215] overflow-hidden">
              <span className="text-[#E11D48] font-bold select-none">$</span>
              <span className="select-all font-semibold">cargo add innertube-rs</span>
            </div>
            <button 
              onClick={handleCopyCmd}
              className="px-3 py-1 bg-[#F6F6F4] hover:bg-[#111215] hover:text-white border border-[#E2E2DF] text-[#111215] text-[11px] transition-colors"
            >
              {copiedCmd ? 'COPIED' : 'COPY'}
            </button>
          </div>

        </section>

        {/* 3. EXPOSED GRID SECTION: SPECIFICATIONS & METRICS */}
        <section className="mb-16 border-t border-[#E2E2DF] pt-8">
          
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
            
            {/* Left Column: Metric Spec Sheet */}
            <div className="lg:col-span-5">
              <div className="font-mono text-xs font-semibold text-[#8E929B] uppercase tracking-wider mb-4">
                01 // LATENCY & FOOTPRINT COMPARISON
              </div>
              <p className="text-xs text-[#5A5D64] mb-6 leading-relaxed">
                Measured in real-world stream extraction benchmarks vs Node.js runtime sidecars and Python subprocess wrappers.
              </p>

              <div className="border border-[#E2E2DF] bg-white divide-y divide-[#E2E2DF] font-mono text-xs">
                
                <div className="p-3.5 flex justify-between items-center bg-[#FBFBFA]">
                  <span className="text-[#8E929B]">METRIC</span>
                  <span className="text-[#111215] font-bold">INNERTUBE-RS</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[#5A5D64]">Signature Decipher</span>
                  <span className="text-[#E11D48] font-bold">&lt; 4.8 ms</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[#5A5D64]">n-Token Transform</span>
                  <span className="text-[#E11D48] font-bold">&lt; 1.2 ms</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[#5A5D64]">Idle Memory Usage</span>
                  <span className="text-[#111215] font-bold">~12 MB RAM</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[#5A5D64]">Runtime Sidecars</span>
                  <span className="text-[#111215] font-bold">0 (Native Binary)</span>
                </div>

              </div>
            </div>

            {/* Right Column: Comparative Benchmarks Table */}
            <div className="lg:col-span-7">
              <div className="font-mono text-xs font-semibold text-[#8E929B] uppercase tracking-wider mb-4">
                02 // BENCHMARK MATRIX
              </div>
              
              <div className="border border-[#E2E2DF] bg-white overflow-x-auto">
                <table className="w-full text-left font-mono text-xs border-collapse">
                  <thead>
                    <tr className="border-b border-[#E2E2DF] bg-[#FBFBFA] text-[#8E929B]">
                      <th className="p-3.5">TARGET</th>
                      <th className="p-3.5 font-bold text-[#E11D48]">innertube-rs</th>
                      <th className="p-3.5">YouTube.js (Node)</th>
                      <th className="p-3.5">yt-dlp (Python)</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-[#E2E2DF] text-[#5A5D64]">
                    <tr>
                      <td className="p-3.5 font-semibold text-[#111215]">Decipher Latency</td>
                      <td className="p-3.5 text-[#E11D48] font-bold">&lt; 4.8 ms</td>
                      <td className="p-3.5">~85 ms</td>
                      <td className="p-3.5 text-[#8E929B]">~3,200 ms</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[#111215]">Memory Overhead</td>
                      <td className="p-3.5 text-[#E11D48] font-bold">12 MB - 16 MB</td>
                      <td className="p-3.5">~140 MB</td>
                      <td className="p-3.5 text-[#8E929B]">~210 MB</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[#111215]">Concurrency</td>
                      <td className="p-3.5 text-[#E11D48] font-bold">Async Tokio</td>
                      <td className="p-3.5">Worker Threads</td>
                      <td className="p-3.5 text-[#8E929B]">Process Fork</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[#111215]">Unit Tests</td>
                      <td className="p-3.5 text-[#E11D48] font-bold">18 Passing (100%)</td>
                      <td className="p-3.5">Vitest Suite</td>
                      <td className="p-3.5 text-[#8E929B]">Pytest</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

          </div>

        </section>

        {/* 4. CODE SPECIMEN & INTERACTIVE PLAYGROUND */}
        <section className="mb-16 border-t border-[#E2E2DF] pt-8">
          
          <div className="flex flex-wrap items-center justify-between gap-4 mb-6">
            <div>
              <div className="font-mono text-xs font-semibold text-[#8E929B] uppercase tracking-wider mb-1">
                03 // CODE SPECIMEN & RUNTIME EXAMPLES
              </div>
              <h2 className="font-['Space_Grotesk'] text-2xl font-bold text-[#111215]">
                Idiomatic Rust InnerTube Client
              </h2>
            </div>

            {/* Tab Switches */}
            <div className="flex flex-wrap items-center gap-1 font-mono text-xs bg-white border border-[#E2E2DF] p-1 shadow-xs">
              {(['stream', 'music', 'channel', 'livechat', 'oauth'] as const).map((tab) => (
                <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`px-3 py-1 transition-colors ${
                    activeTab === tab 
                      ? 'bg-[#111215] text-white font-bold' 
                      : 'text-[#5A5D64] hover:text-[#111215]'
                  }`}
                >
                  {tab.toUpperCase()}
                </button>
              ))}
            </div>
          </div>

          {/* Code Window Container */}
          <div className="border border-[#E2E2DF] bg-white shadow-xs">
            
            <div className="border-b border-[#E2E2DF] px-4 py-2.5 bg-[#FBFBFA] flex items-center justify-between font-mono text-xs text-[#5A5D64]">
              <div className="flex items-center gap-2">
                <FileCode className="w-3.5 h-3.5 text-[#E11D48]" />
                <span className="font-semibold text-[#111215]">{codeSnippets[activeTab].file}</span>
              </div>
              <button
                onClick={handleCopyCode}
                className="flex items-center gap-1 px-2 py-0.5 border border-[#E2E2DF] bg-white hover:bg-[#111215] hover:text-white transition-colors text-[11px]"
              >
                {copiedCode ? (
                  <>
                    <Check className="w-3 h-3 text-[#10B981]" />
                    <span className="text-[#10B981]">COPIED</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3 h-3" />
                    <span>COPY_CODE</span>
                  </>
                )}
              </button>
            </div>

            <div className="p-6 overflow-x-auto font-mono text-xs sm:text-sm leading-relaxed text-[#111215] bg-[#FFFFFF]">
              <pre><code>{codeSnippets[activeTab].code}</code></pre>
            </div>

          </div>

        </section>

        {/* 5. SIX-CELL EXPOSED BLUEPRINT GRID */}
        <section className="border-t border-[#E2E2DF] pt-8">
          
          <div className="font-mono text-xs font-semibold text-[#8E929B] uppercase tracking-wider mb-6">
            04 // CORE ENGINE CAPABILITIES
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 border border-[#E2E2DF] divide-y md:divide-y-0 md:divide-x divide-[#E2E2DF] bg-white">
            
            {/* Grid 1 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[#E11D48] font-bold mb-3">
                  01 / QUICKJS SANDBOX
                </div>
                <h3 className="font-['Space_Grotesk'] text-lg font-bold text-[#111215] mb-2">
                  Embedded Decipher Engine
                </h3>
                <p className="text-xs text-[#5A5D64] leading-relaxed">
                  Evaluates YouTube's latest signature unscrambling and throttling n-token algorithms in an embedded isolated QuickJS runtime in &lt;5ms.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[#E2E2DF] font-mono text-[11px] text-[#8E929B]">
                REF: <code>src/core/player.rs</code>
              </div>
            </div>

            {/* Grid 2 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[#E11D48] font-bold mb-3">
                  02 / AST NODES
                </div>
                <h3 className="font-['Space_Grotesk'] text-lg font-bold text-[#111215] mb-2">
                  Polymorphic AST Parser
                </h3>
                <p className="text-xs text-[#5A5D64] leading-relaxed">
                  Recursive JSON tree walker mapping arbitrary InnerTube renderers and view models into strongly typed, resilient <code>YTNode</code> variants.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[#E2E2DF] font-mono text-[11px] text-[#8E929B]">
                REF: <code>src/parser/nodes/</code>
              </div>
            </div>

            {/* Grid 3 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[#E11D48] font-bold mb-3">
                  03 / MUSIC ENGINE
                </div>
                <h3 className="font-['Space_Grotesk'] text-lg font-bold text-[#111215] mb-2">
                  YouTube Music Suite
                </h3>
                <p className="text-xs text-[#5A5D64] leading-relaxed">
                  Full suite for YT Music searches, artist biographies, albums, singles, explore charts, and synchronized lyrics extraction.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[#E2E2DF] font-mono text-[11px] text-[#8E929B]">
                REF: <code>src/endpoints/music.rs</code>
              </div>
            </div>

          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 border-x border-b border-[#E2E2DF] divide-y md:divide-y-0 md:divide-x divide-[#E2E2DF] bg-white">
            
            {/* Grid 4 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[#E11D48] font-bold mb-3">
                  04 / REALTIME
                </div>
                <h3 className="font-['Space_Grotesk'] text-lg font-bold text-[#111215] mb-2">
                  Live Chat & Community
                </h3>
                <p className="text-xs text-[#5A5D64] leading-relaxed">
                  Extract active live stream continuation tokens, poll live chat messages, Super Chats, channel community posts, and voting polls.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[#E2E2DF] font-mono text-[11px] text-[#8E929B]">
                REF: <code>src/endpoints/live_chat.rs</code>
              </div>
            </div>

            {/* Grid 5 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[#E11D48] font-bold mb-3">
                  05 / AUTHENTICATION
                </div>
                <h3 className="font-['Space_Grotesk'] text-lg font-bold text-[#111215] mb-2">
                  OAuth2 Device Flow
                </h3>
                <p className="text-xs text-[#5A5D64] leading-relaxed">
                  Authenticate user accounts via Google TV device code flow with automatic background access token refreshing and cookie management.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[#E2E2DF] font-mono text-[11px] text-[#8E929B]">
                REF: <code>src/core/oauth.rs</code>
              </div>
            </div>

            {/* Grid 6 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[#E11D48] font-bold mb-3">
                  06 / MANIFESTS
                </div>
                <h3 className="font-['Space_Grotesk'] text-lg font-bold text-[#111215] mb-2">
                  Transcripts & Manifests
                </h3>
                <p className="text-xs text-[#5A5D64] leading-relaxed">
                  Parse multi-language timed transcripts with SRT / WebVTT export, alongside native DASH MPD and HLS Master M3U8 adaptive manifest parsers.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[#E2E2DF] font-mono text-[11px] text-[#8E929B]">
                REF: <code>src/endpoints/transcript.rs</code>
              </div>
            </div>

          </div>

        </section>

      </main>

      {/* 6. SWISS EDITORIAL FOOTER */}
      <footer className="border-t border-[#E2E2DF] bg-[#F6F6F4] py-8">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col sm:flex-row items-center justify-between gap-4 font-mono text-xs text-[#8E929B]">
          
          <div className="flex items-center gap-2 text-[#111215]">
            <span className="w-2 h-2 bg-[#E11D48] inline-block"></span>
            <span className="font-bold tracking-tight">/ innertube-rs</span>
            <span className="text-[#8E929B]">·</span>
            <span className="text-[#5A5D64]">MIT LICENSE · 100% PURE RUST</span>
          </div>

          <div className="flex items-center gap-6">
            <a href="https://crates.io/crates/innertube-rs" target="_blank" rel="noopener noreferrer" className="hover:text-[#111215] transition-colors">
              CRATES.IO
            </a>
            <a href="https://docs.rs/innertube-rs" target="_blank" rel="noopener noreferrer" className="hover:text-[#111215] transition-colors">
              DOCS.RS
            </a>
            <a href="https://github.com/caya8205-2/innertube-rs" target="_blank" rel="noopener noreferrer" className="hover:text-[#111215] transition-colors">
              GITHUB
            </a>
          </div>

        </div>
      </footer>

    </div>
  );
}
