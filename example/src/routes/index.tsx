import { open } from "@tauri-apps/plugin-shell";
import {
  HapticFeedbackPattern,
  perform,
  PerformanceTime,
} from "tauri-plugin-macos-haptics-api";
import StepSlider from "~/components/step-slider";
import Github from "~/icons/github";

export default function () {
  // document.documentElement.oncontextmenu = (e: MouseEvent) =>
  //   e.preventDefault();

  return (
    <>
      <div
        data-tauri-drag-region
        class="user-select-none fixed left-0 right-0 top-0 flex h-11 items-center justify-end"
      >
        <button
          type="button"
          class="btn btn-xs btn-ghost mr-3 mt-0.5"
          style={{
            "border-radius": "6px",
          }}
          onClick={(_) =>
            open("https://github.com/itseeleeya/tauri-plugin-macos-haptics")
          }
        >
          <Github class="h-4 w-4 fill-current" />
          Star on Github!
        </button>
      </div>
      <div class="flex min-h-screen w-full flex-col items-center justify-center gap-6 text-center">
        <h1 class="text-center text-4xl font-semibold ">Tauri macOS Haptics</h1>

        <StepSlider min={0} max={100} step={10} initialValue={50} />

        <input
          type="checkbox"
          class="toggle"
          checked={false}
          onChange={(_) => {
            setTimeout(() => {
              perform(HapticFeedbackPattern.LevelChange, PerformanceTime.Now);
            }, 180);
          }}
        />
      </div>
    </>
  );
}
