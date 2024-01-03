import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function Video() {
  interface Video {
    cover: string;
    title: string;
    href: string;
    cost: string;
  }

  interface Data {
    videos: Video[];
  }

  interface Response {
    code: number;
    msg: string;
    data: Data;
  }

  const [videos, set_videos] = useState<Video[]>([]);
  const [page, set_page] = useState<number>(1);
  const [play, set_play] = useState<string>("");
  const [is_play, set_is_play] = useState<boolean>(false);

  async function getVideos() {
    invoke<Response>("porn_hot", { page })
      .then((res) => {
        set_videos(res.data.videos);
        if (page < 4) {
          set_page(page + 1);
        }
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
      {is_play ? (
        <div className="fixed z-10 flex h-screen w-screen justify-center">
          <div className="h-full align-middle">
            <video className="h-1/2 w-full bg-gray-200" controls>
              <source
                src="https://cdn2.jiuse2.cloud/hls/920371/index.m3u8?t=1704309135&m=DLGrvoGJJSL604yvpq_Ljg"
                type="application/x-mpegURL"
              />
            </video>
          </div>
          <div onClick={() => set_is_play(false)}></div>
        </div>
      ) : (
        <div></div>
      )}
      <div className="ml-24 mr-24 grid grid-cols-3">
        {videos.map((video) => (
          <div
            className="m-4 overflow-hidden rounded-md bg-gray-200 shadow-lg transition-all hover:-translate-y-1 hover:shadow-xl"
            onClick={() => {
              set_play(video.href);
              set_is_play(true);
            }}
          >
            <img
              src={video.cover}
              className="animate__animated animate__fadeIn h-44 w-full bg-black object-contain"
            />
            <p className="line-clamp-1 pl-2">{video.title}</p>
            <p className="line-clamp-1 pb-2 pl-2 text-gray-500">{video.cost}</p>
          </div>
        ))}
      </div>
    </div>
  );
}

export default Video;
