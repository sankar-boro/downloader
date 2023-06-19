/**
 * http://en.wikipedia.org/wiki/YouTube#Quality_and_formats
 */
static FORMATS: &str = r#"{

    {
      mimeType: 'video/flv; codecs="Sorenson H.283, mp3"',
      qualityLabel: '240p',
      bitrate: 250000,
      audioBitrate: 64,
    },
    {
      mimeType: 'video/flv; codecs="Sorenson H.263, mp3"',
      qualityLabel: '270p',
      bitrate: 800000,
      audioBitrate: 64,
    },
    {
      mimeType: 'video/3gp; codecs="MPEG-4 Visual, aac"',
      qualityLabel: null,
      bitrate: 500000,
      audioBitrate: null,
    },
    {
      mimeType: 'video/3gp; codecs="MPEG-4 Visual, aac"',
      qualityLabel: '144p',
      bitrate: 50000,
      audioBitrate: 24,
    },
    {
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '360p',
      bitrate: 500000,
      audioBitrate: 96,
    },
    {
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '720p',
      bitrate: 2000000,
      audioBitrate: 192,
    },
    {
      mimeType: 'video/flv; codecs="H.264, aac"',
      qualityLabel: '360p',
      bitrate: 500000,
      audioBitrate: 128,
    },
    {
      mimeType: 'video/flv; codecs="H.264, aac"',
      qualityLabel: '480p',
      bitrate: 800000,
      audioBitrate: 128,
    },
    {
      mimeType: 'video/3gp; codecs="MPEG-4 Visual, aac"',
      qualityLabel: '240p',
      bitrate: 175000,
      audioBitrate: 32,
    },
    {
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '1080p',
      bitrate: 3000000,
      audioBitrate: 192,
    },
    {
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '3072p',
      bitrate: 3500000,
      audioBitrate: 192,
    },
    {
      mimeType: 'video/webm; codecs="VP8, vorbis"',
      qualityLabel: '360p',
      bitrate: 500000,
      audioBitrate: 128,
    },
    {
      mimeType: 'video/webm; codecs="VP8, vorbis"',
      qualityLabel: '480p',
      bitrate: 1000000,
      audioBitrate: 128,
    },
    {
      mimeType: 'video/webm; codecs="VP8, vorbis"',
      qualityLabel: '720p',
      bitrate: 2000000,
      audioBitrate: 192,
    },
    {
      mimeType: 'audio/webm; codecs="vp8, vorbis"',
      qualityLabel: '1080p',
      bitrate: null,
      audioBitrate: 192,
    },
    {
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '360p',
      bitrate: 500000,
      audioBitrate: 96,
    },
    {
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '240p',
      bitrate: 500000,
      audioBitrate: 96,
    },{
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '720p',
      bitrate: 2000000,
      audioBitrate: 192,
    },{
      mimeType: 'video/mp4; codecs="H.264, aac"',
      qualityLabel: '1080p',
      bitrate: 3000000,
      audioBitrate: 192,
    },{
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '144p',
      bitrate: 100000,
      audioBitrate: 48,
    },{
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '240p',
      bitrate: 150000,
      audioBitrate: 48,
    },{
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '360p',
      bitrate: 500000,
      audioBitrate: 128,
    },{
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '480p',
      bitrate: 800000,
      audioBitrate: 128,
    },{
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '720p',
      bitrate: 1500000,
      audioBitrate: 256,
    },{
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '1080p',
      bitrate: 2500000,
      audioBitrate: 256,
    }, {
      mimeType: 'audio/webm; codecs="VP8, vorbis"',
      qualityLabel: '360p',
      bitrate: null,
      audioBitrate: 128,
    }, {
      mimeType: 'audio/webm; codecs="VP8, vorbis"',
      qualityLabel: '360p',
      bitrate: null,
      audioBitrate: 192,
    }, {
      mimeType: 'audio/webm; codecs="VP8, vorbis"',
      qualityLabel: '720p',
      bitrate: null,
      audioBitrate: 192,
    }, {
      mimeType: 'video/flv; codecs="H.264, aac"',
      qualityLabel: '720p',
      bitrate: 2000000,
      audioBitrate: 128,
    }, {
      mimeType: 'audio/ts; codecs="aac"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 96,
    }, {
      mimeType: 'audio/ts; codecs="aac"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 96,
    }, {
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '240p',
      bitrate: 150000,
      audioBitrate: 48,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '240p',
      bitrate: 200000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '360p',
      bitrate: 300000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '480p',
      bitrate: 500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '720p',
      bitrate: 1000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '1080p',
      bitrate: 2500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '4320p',
      bitrate: 13500000,
      audioBitrate: null,
    }, {
      mimeType: 'audio/mp4; codecs="aac"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 48,
    }, {
      mimeType: 'audio/m4a; codecs="aac"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 128,
    }, {
      mimeType: 'audio/mp4; codecs="aac"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 256,
    }, {
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '720p',
      bitrate: 50000,
      audioBitrate: 24,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '144p',
      bitrate: 100000,
      audioBitrate: null,
    }, {
      mimeType: 'audio/webm; codecs="vorbis"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 128,
    }, {
      mimeType: 'audio/webm; codecs="vorbis"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 192,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '240p',
      bitrate: 100000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '360p',
      bitrate: 250000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '480p',
      bitrate: 500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '720p',
      bitrate: 700000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '1080p',
      bitrate: 1500000,
      audioBitrate: null,
    }, {
      mimeType: 'audio/webm; codecs="opus"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 48,
    }, {
      mimeType: 'audio/webm; codecs="opus"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 64,
    }, {
      mimeType: 'audio/webm; codecs="opus"',
      qualityLabel: null,
      bitrate: null,
      audioBitrate: 160,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '1440p',
      bitrate: 4000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '2160p',
      bitrate: 12500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '1440p',
      bitrate: 9000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '4320p',
      bitrate: 20000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '144p 30fps',
      bitrate: 80000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '720p',
      bitrate: 3000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/mp4; codecs="H.264"',
      qualityLabel: '1080p',
      bitrate: 5500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/ts; codecs="H.264, aac"',
      qualityLabel: '720p',
      bitrate: 1318000,
      audioBitrate: 48,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '720p HFR',
      bitrate: 2500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '1080p HFR',
      bitrate: 5000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '1440p HFR',
      bitrate: 10000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '2160p',
      bitrate: 13000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '2160p HFR',
      bitrate: 20000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '144p HDR, HFR',
      bitrate: 80000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '240p HDR, HFR',
      bitrate: 100000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '360p HDR, HFR',
      bitrate: 250000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '240p HDR, HFR',
      bitrate: 500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '720p HDR, HFR',
      bitrate: 1000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '1080p HDR, HFR',
      bitrate: 1500000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '1440p HDR, HFR',
      bitrate: 5000000,
      audioBitrate: null,
    }, {
      mimeType: 'video/webm; codecs="VP9"',
      qualityLabel: '2160p HDR, HFR',
      bitrate: 12000000,
      audioBitrate: null,
    },
  
  }"#;
  