/* Hallmark · macrostructure: Component Playground · genre: modern-minimal · theme: Grid Dark (user override) · enrichment: none · nav: preserved · footer: preserved */
/* Hallmark · pre-emit critique: P5 H5 E5 S5 R5 V4 */
import { useState } from 'react';
import { 
  Check, 
  Copy, 
  ExternalLink, 
  FileCode
} from 'lucide-react';

export default function App() {
  const [activeTab, setActiveTab] = useState<'stream' | 'music' | 'channel' | 'livechat' | 'oauth'>('stream');
  const [copiedCommand, setCopiedCommand] = useState<'add' | 'install' | null>(null);
  const [copiedCode, setCopiedCode] = useState(false);

  const handleCopyCommand = (command: 'add' | 'install') => {
    navigator.clipboard.writeText(`cargo ${command} innertube-rs`);
    setCopiedCommand(command);
    setTimeout(() => setCopiedCommand(null), 2000);
  };

  const codeSnippets = {
    stream: {
      file: 'examples/download/download_audio.rs',
      title: 'Resolve an audio stream',
      code: `use innertube_rs::{Innertube, FormatFilter, FormatType, QualityPreference};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client and load the current player
    let yt = Innertube::new().await?;
    let video_id = "dQw4w9WgXcQ";

    // Resolve the highest-bitrate audio-only stream
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
      title: 'Read a YouTube Music artist',
      code: `use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Fetch the artist profile, releases, and top tracks
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
      title: 'Read channel community posts',
      code: `use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Fetch community posts, poll choices, and images
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
      title: 'Poll live chat messages',
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
      title: 'Sign in with the OAuth2 device flow',
      code: `use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Request a user code for google.com/device
    let code = yt.request_oauth_code().await?;
    println!("Visit: {} -> Enter: {}", code.verification_url, code.user_code);

    // Poll until the user authorizes the device
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
    <div className="min-h-screen bg-[var(--color-paper)] text-[var(--color-ink)] flex flex-col justify-between selection:bg-[var(--color-accent)] selection:text-[var(--color-accent-ink)]">
      
      {/* Header */}
      <header className="border-b border-[var(--color-rule)] bg-[var(--color-paper)] sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-14 flex items-center justify-between">
          
          {/* Brand Wordmark */}
          <div className="flex items-center gap-3">
            <span className="font-mono text-sm font-semibold tracking-tight text-[var(--color-ink)] whitespace-nowrap">
              / innertube-rs
            </span>
            <span className="hidden sm:inline font-mono text-[11px] text-[var(--color-ink-faint)] uppercase tracking-wider">
              v0.9.0
            </span>

            {/* Install Button */}
            <button 
              onClick={() => handleCopyCommand('add')}
              className="hidden sm:inline-flex items-center gap-1.5 px-2.5 py-1 text-[11px] font-mono font-medium border border-[var(--color-ink)] hover:bg-[var(--color-ink)] hover:text-[var(--color-on-strong)] transition-colors"
            >
              {copiedCommand === 'add' ? 'COPIED' : 'COPY CMD'}
            </button>

            {/* Crates.io Pill */}
            <a 
              href="https://crates.io/crates/innertube-rs"
              target="_blank" 
              rel="noopener noreferrer"
              className="hidden sm:inline-flex items-center gap-1.5 px-2 py-0.5 text-[11px] font-mono font-medium bg-[var(--color-ink)] text-[var(--color-on-strong)]"
            >
              <span>CRATES.IO</span>
            </a>
          </div>

          {/* Right header navigation */}
          <div className="flex items-center gap-3 sm:gap-4 font-mono text-xs">
            <a 
              href="https://docs.rs/innertube-rs" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-[var(--color-ink-muted)] hover:text-[var(--color-ink)] flex items-center gap-1 transition-colors"
            >
              <span>docs.rs</span>
              <ExternalLink className="w-3 h-3 text-[var(--color-ink-faint)]" />
            </a>

            <a 
              href="https://github.com/caya8205-2/innertube-rs" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-[var(--color-ink-muted)] hover:text-[var(--color-ink)] transition-colors"
            >
              github
            </a>
          </div>

        </div>
      </header>

      {/* Main content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16 w-full flex-grow">
        
        {/* Hero */}
        <section className="mb-16">
          
          {/* Category Tag */}
          <div className="flex items-center gap-2 mb-6">
            <span className="w-2.5 h-2.5 bg-[var(--color-accent)] inline-block"></span>
            <span className="font-mono text-xs uppercase tracking-widest text-[var(--color-ink-muted)] font-semibold">
              STREAMS · SEARCH · MUSIC · FEEDS · PLAYLISTS · COMMUNITY · ACCOUNT
            </span>
          </div>

          {/* Display heading */}
          <h1 className="font-display text-4xl sm:text-6xl lg:text-7xl font-bold text-[var(--color-ink)] tracking-tight leading-[1.05] max-w-5xl mb-8">
            a Rust client for YouTube’s InnerTube API.
          </h1>

          {/* Subtext */}
          <p className="text-base sm:text-xl text-[var(--color-ink-muted)] max-w-3xl leading-relaxed mb-10 font-normal">
            Read videos, feeds, channels, playlists, comments, live chat, transcripts, and YouTube Music. Resolve streams, search, authenticate, and perform account actions from Rust—without a Node.js or Python process.
          </p>

          {/* Cargo commands */}
          <div className="max-w-xl bg-[var(--color-surface)] border border-[var(--color-rule)] divide-y divide-[var(--color-rule)] font-mono text-xs shadow-xs">
            <div className="p-3 flex items-center justify-between gap-3">
              <div className="flex min-w-0 items-center gap-2.5 text-[var(--color-ink)] overflow-hidden">
                <span className="text-[var(--color-accent)] font-bold select-none">$</span>
                <span className="select-all font-semibold whitespace-nowrap">cargo add innertube-rs</span>
              </div>
              <button
                onClick={() => handleCopyCommand('add')}
                aria-label="Copy library dependency command"
                className="shrink-0 px-3 py-1 bg-[var(--color-paper)] hover:bg-[var(--color-ink)] hover:text-[var(--color-on-strong)] border border-[var(--color-rule)] text-[var(--color-ink)] text-[11px] transition-colors"
              >
                {copiedCommand === 'add' ? 'COPIED' : 'COPY'}
              </button>
            </div>
            <div className="p-3 flex items-center justify-between gap-3">
              <div className="flex min-w-0 items-center gap-2.5 text-[var(--color-ink)] overflow-hidden">
                <span className="text-[var(--color-accent)] font-bold select-none">$</span>
                <span className="select-all font-semibold whitespace-nowrap">cargo install innertube-rs</span>
              </div>
              <button
                onClick={() => handleCopyCommand('install')}
                aria-label="Copy binary install command"
                className="shrink-0 px-3 py-1 bg-[var(--color-paper)] hover:bg-[var(--color-ink)] hover:text-[var(--color-on-strong)] border border-[var(--color-rule)] text-[var(--color-ink)] text-[11px] transition-colors"
              >
                {copiedCommand === 'install' ? 'COPIED' : 'COPY'}
              </button>
            </div>
          </div>

        </section>

        {/* Runtime and API coverage */}
        <section className="mb-16 border-t border-[var(--color-rule)] pt-8">
          
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
            
            {/* Runtime model */}
            <div className="lg:col-span-5">
              <div className="font-mono text-xs font-semibold text-[var(--color-ink-faint)] uppercase tracking-wider mb-4">
                RUNTIME MODEL
              </div>
              <p className="text-xs text-[var(--color-ink-muted)] mb-6 leading-relaxed">
                One Rust process handles HTTP requests, response parsing, and player-script transforms.
              </p>

              <div className="border border-[var(--color-rule)] bg-[var(--color-surface)] divide-y divide-[var(--color-rule)] font-mono text-xs">
                
                <div className="p-3.5 flex justify-between items-center bg-[var(--color-surface-subtle)]">
                  <span className="text-[var(--color-ink-faint)]">COMPONENT</span>
                  <span className="text-[var(--color-ink)] font-bold">IMPLEMENTATION</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[var(--color-ink-muted)]">Signature transforms</span>
                  <span className="text-[var(--color-accent)] font-bold">Embedded QuickJS</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[var(--color-ink-muted)]">HTTP client</span>
                  <span className="text-[var(--color-accent)] font-bold">reqwest + HTTP/2</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[var(--color-ink-muted)]">Async runtime</span>
                  <span className="text-[var(--color-ink)] font-bold">Tokio</span>
                </div>

                <div className="p-3.5 flex justify-between items-center">
                  <span className="text-[var(--color-ink-muted)]">External sidecars</span>
                  <span className="text-[var(--color-ink)] font-bold">None</span>
                </div>

              </div>
            </div>

            {/* API coverage */}
            <div className="lg:col-span-7">
              <div className="font-mono text-xs font-semibold text-[var(--color-ink-faint)] uppercase tracking-wider mb-4">
                API COVERAGE
              </div>
              
              <div className="border border-[var(--color-rule)] bg-[var(--color-surface)] overflow-x-auto">
                <table className="w-full text-left font-mono text-xs border-collapse">
                  <thead>
                    <tr className="border-b border-[var(--color-rule)] bg-[var(--color-surface-subtle)] text-[var(--color-ink-faint)]">
                      <th className="p-3.5">AREA</th>
                      <th className="p-3.5 font-bold text-[var(--color-accent)]">OPERATIONS</th>
                      <th className="p-3.5">RETURNS</th>
                      <th className="p-3.5">EXAMPLE</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-[var(--color-rule)] text-[var(--color-ink-muted)]">
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Video</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">Metadata + streams</td>
                      <td className="p-3.5">Video info + formats</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">get_video_info</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Discovery</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">Search + suggestions + feeds</td>
                      <td className="p-3.5">Search + feed models</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">search_with_filters</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Music</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">Search + browse + lyrics</td>
                      <td className="p-3.5">Music models</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">get_music_artist</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Community</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">Channels + posts + comments</td>
                      <td className="p-3.5">Channel + post models</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">get_channel_videos</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Playlists</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">Read + create + edit</td>
                      <td className="p-3.5">Playlist models</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">get_playlist</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Live + text</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">Chat + transcripts</td>
                      <td className="p-3.5">Messages + timed text</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">get_live_chat</td>
                    </tr>
                    <tr>
                      <td className="p-3.5 font-semibold text-[var(--color-ink)]">Account</td>
                      <td className="p-3.5 text-[var(--color-accent)] font-bold">History + library + actions</td>
                      <td className="p-3.5">Feeds + action results</td>
                      <td className="p-3.5 text-[var(--color-ink-faint)]">get_history</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

          </div>

        </section>

        {/* Working examples */}
        <section className="mb-16 border-t border-[var(--color-rule)] pt-8">
          
          <div className="flex flex-wrap items-center justify-between gap-4 mb-6">
            <div>
              <div className="font-mono text-xs font-semibold text-[var(--color-ink-faint)] uppercase tracking-wider mb-1">
                WORKING EXAMPLES
              </div>
              <h2 className="font-display text-2xl font-bold text-[var(--color-ink)]">
                Start with the Rust client
              </h2>
            </div>

            {/* Tab Switches */}
            <div className="flex flex-wrap items-center gap-1 font-mono text-xs bg-[var(--color-surface)] border border-[var(--color-rule)] p-1 shadow-xs">
              {(['stream', 'music', 'channel', 'livechat', 'oauth'] as const).map((tab) => (
                <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`px-3 py-1 transition-colors ${
                    activeTab === tab 
                      ? 'bg-[var(--color-ink)] text-[var(--color-on-strong)] font-bold'
                      : 'text-[var(--color-ink-muted)] hover:text-[var(--color-ink)]'
                  }`}
                >
                  {tab.toUpperCase()}
                </button>
              ))}
            </div>
          </div>

          {/* Code Window Container */}
          <div className="border border-[var(--color-rule)] bg-[var(--color-surface)] shadow-xs">
            
            <div className="border-b border-[var(--color-rule)] px-3 sm:px-4 py-2.5 bg-[var(--color-surface-subtle)] flex min-w-0 items-center justify-between gap-3 font-mono text-xs text-[var(--color-ink-muted)]">
              <div className="flex min-w-0 flex-1 items-center gap-2 overflow-hidden">
                <FileCode className="w-3.5 h-3.5 shrink-0 text-[var(--color-accent)]" />
                <span
                  className="block min-w-0 truncate font-semibold text-[var(--color-ink)]"
                  title={codeSnippets[activeTab].file}
                >
                  {codeSnippets[activeTab].file}
                </span>
              </div>
              <button
                onClick={handleCopyCode}
                className="flex shrink-0 items-center gap-1 whitespace-nowrap px-2 py-0.5 border border-[var(--color-rule)] bg-[var(--color-surface)] hover:bg-[var(--color-ink)] hover:text-[var(--color-on-strong)] transition-colors text-[11px]"
              >
                {copiedCode ? (
                  <>
                    <Check className="w-3 h-3 text-[var(--color-success)]" />
                    <span className="text-[var(--color-success)]">COPIED</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3 h-3" />
                    <span>COPY CODE</span>
                  </>
                )}
              </button>
            </div>

            <div className="p-6 overflow-x-auto font-mono text-xs sm:text-sm leading-relaxed text-[var(--color-ink)] bg-[var(--color-surface)]">
              <pre><code>{codeSnippets[activeTab].code}</code></pre>
            </div>

          </div>

        </section>

        {/* Supported operations */}
        <section className="border-t border-[var(--color-rule)] pt-8">
          
          <div className="font-mono text-xs font-semibold text-[var(--color-ink-faint)] uppercase tracking-wider mb-6">
            SUPPORTED OPERATIONS
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 border border-[var(--color-rule)] divide-y md:divide-y-0 md:divide-x divide-[var(--color-rule)] bg-[var(--color-surface)]">
            
            {/* Capability 1 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[var(--color-accent)] font-bold mb-3">
                  STREAMS & SIGNATURES
                </div>
                <h3 className="font-display text-lg font-bold text-[var(--color-ink)] mb-2">
                  Stream URL resolution
                </h3>
                <p className="text-xs text-[var(--color-ink-muted)] leading-relaxed">
                  Select audio or video formats, resolve ciphered URLs, and apply signature and n-token transforms through embedded QuickJS.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[var(--color-rule)] font-mono text-[11px] text-[var(--color-ink-faint)]">
                REF: <code>src/core/player.rs</code>
              </div>
            </div>

            {/* Capability 2 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[var(--color-accent)] font-bold mb-3">
                  SEARCH & FEEDS
                </div>
                <h3 className="font-display text-lg font-bold text-[var(--color-ink)] mb-2">
                  Discovery endpoints
                </h3>
                <p className="text-xs text-[var(--color-ink-muted)] leading-relaxed">
                  Run filtered search, request suggestions, and browse home, trending, hashtag, subscription, channel, and playlist feeds.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[var(--color-rule)] font-mono text-[11px] text-[var(--color-ink-faint)]">
                REF: <code>src/endpoints/search.rs + feed.rs</code>
              </div>
            </div>

            {/* Capability 3 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[var(--color-accent)] font-bold mb-3">
                  YOUTUBE MUSIC
                </div>
                <h3 className="font-display text-lg font-bold text-[var(--color-ink)] mb-2">
                  Music endpoints
                </h3>
                <p className="text-xs text-[var(--color-ink-muted)] leading-relaxed">
                  Search music, read artist profiles, browse releases and charts, and retrieve synchronized lyrics.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[var(--color-rule)] font-mono text-[11px] text-[var(--color-ink-faint)]">
                REF: <code>src/endpoints/music.rs</code>
              </div>
            </div>

          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 border-x border-b border-[var(--color-rule)] divide-y md:divide-y-0 md:divide-x divide-[var(--color-rule)] bg-[var(--color-surface)]">
            
            {/* Capability 4 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[var(--color-accent)] font-bold mb-3">
                  CHANNELS & COMMUNITY
                </div>
                <h3 className="font-display text-lg font-bold text-[var(--color-ink)] mb-2">
                  Channels, comments & live chat
                </h3>
                <p className="text-xs text-[var(--color-ink-muted)] leading-relaxed">
                  Read videos, Shorts, posts, polls, comments, replies, live chat messages, and Super Chats through continuation tokens.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[var(--color-rule)] font-mono text-[11px] text-[var(--color-ink-faint)]">
                REF: <code>src/endpoints/channel.rs + live_chat.rs</code>
              </div>
            </div>

            {/* Capability 5 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[var(--color-accent)] font-bold mb-3">
                  ACCOUNT & ACTIONS
                </div>
                <h3 className="font-display text-lg font-bold text-[var(--color-ink)] mb-2">
                  Authenticated reads and writes
                </h3>
                <p className="text-xs text-[var(--color-ink-muted)] leading-relaxed">
                  Read history, library, and notifications; rate videos, subscribe, comment, and create or edit playlists.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[var(--color-rule)] font-mono text-[11px] text-[var(--color-ink-faint)]">
                REF: <code>src/core/actions.rs</code>
              </div>
            </div>

            {/* Capability 6 */}
            <div className="p-6 flex flex-col justify-between">
              <div>
                <div className="font-mono text-xs text-[var(--color-accent)] font-bold mb-3">
                  TRANSCRIPTS & MANIFESTS
                </div>
                <h3 className="font-display text-lg font-bold text-[var(--color-ink)] mb-2">
                  Transcripts & Manifests
                </h3>
                <p className="text-xs text-[var(--color-ink-muted)] leading-relaxed">
                  Read timed transcripts, export SRT or WebVTT, and parse DASH MPD and HLS M3U8 manifests.
                </p>
              </div>
              <div className="mt-6 pt-4 border-t border-[var(--color-rule)] font-mono text-[11px] text-[var(--color-ink-faint)]">
                REF: <code>src/endpoints/transcript.rs</code>
              </div>
            </div>

          </div>

        </section>

      </main>

      {/* Footer */}
      <footer className="border-t border-[var(--color-rule)] bg-[var(--color-paper)] py-8">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col sm:flex-row items-center justify-between gap-4 font-mono text-xs text-[var(--color-ink-faint)]">
          
          <div className="flex items-center gap-2 text-[var(--color-ink)]">
            <span className="w-2 h-2 bg-[var(--color-accent)] inline-block"></span>
            <span className="font-bold tracking-tight">/ innertube-rs</span>
            <span className="text-[var(--color-ink-faint)]">·</span>
            <span className="text-[var(--color-ink-muted)]">MIT LICENSE</span>
          </div>

          <div className="flex items-center gap-6">
            <a href="https://crates.io/crates/innertube-rs" target="_blank" rel="noopener noreferrer" className="hover:text-[var(--color-ink)] transition-colors">
              CRATES.IO
            </a>
            <a href="https://docs.rs/innertube-rs" target="_blank" rel="noopener noreferrer" className="hover:text-[var(--color-ink)] transition-colors">
              DOCS.RS
            </a>
            <a href="https://github.com/caya8205-2/innertube-rs" target="_blank" rel="noopener noreferrer" className="hover:text-[var(--color-ink)] transition-colors">
              GITHUB
            </a>
          </div>

        </div>
      </footer>

    </div>
  );
}
