import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  interface Video {
    cover: string;
    title: string;
    time: string;
    preview: string;
  }

  interface Result {
    videos: Video[];
  }

  const [videos, setVideos] = useState<Video[]>([]);

  async function getVideos() {
    invoke<Result>("get_videos")
      .then((res) => {
        setVideos(res.videos);
      })
      .catch((err) => {
        console.log(err);
      });
  }

  let ignore = false;
  useEffect(() => {
    if (!ignore) {
      getVideos();
    }

    return () => {
      ignore = true;
    };
  }, []);

  return (
    <div>
      {videos.map((video) => (
        <div>
          <img src={video.cover} />
          <p>{video.title}</p>
          <p>{video.time}</p>
          <video width="320" height="240" controls>
            <source src={video.preview} type="video/mp4" />
          </video>
          <video width="320" height="240" controls>
            <source
              src="https://ricodaniel.com/8f309e8e-3e33-4db1-b181-4213f20067ec/playlist.m3u8"
              type="application/x-mpegURL"
            />
          </video>
        </div>
      ))}
    </div>
  );
}

export default App;
