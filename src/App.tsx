import { useState } from "react";
import Video from "./Video";
import {
  CollectionFill,
  StarFill,
  Collection,
  Star,
} from "react-bootstrap-icons";

function App() {
  class Tab {
    public tab: string;
    public item: JSX.Element;
    public icon: JSX.Element;
    public icon_selected: JSX.Element;

    constructor(
      tab: string,
      item: JSX.Element,
      icon: JSX.Element,
      icon_selected: JSX.Element,
    ) {
      this.tab = tab;
      this.item = item;
      this.icon = icon;
      this.icon_selected = icon_selected;
    }
  }

  const tabs = Array<Tab>(
    new Tab(
      "Video",
      <Video />,
      <Collection className="w-full" />,
      <CollectionFill className="w-full" />,
    ),
    new Tab(
      "Like",
      <div>Like</div>,
      <Star className="w-full" />,
      <StarFill className="w-full" />,
    ),
  );

  const [current_tab, set_current_tab] = useState<number>(0);
  let display = tabs[current_tab].item;

  return (
    <div className="min-h-screen w-screen">
      <div className="fixed flex h-12 w-full justify-center bg-white shadow z-10">
        <div className="flex w-1/3 justify-between">
          {tabs.map((tab, i) => (
            <button
              onClick={() => set_current_tab(i)}
              className="ml-1 mr-1 w-full hover:rounded-lg hover:pb-1"
            >
              {i == current_tab ? tab.icon_selected : tab.icon}
              {i == current_tab ? (
                <span className="text-sm font-bold">{tab.tab}</span>
              ) : (
                <span className="text-sm">{tab.tab}</span>
              )}
            </button>
          ))}
        </div>
      </div>
      <div className="w-full pt-12">{display}</div>
    </div>
  );
}

export default App;
