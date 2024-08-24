import { createSignal, For } from "solid-js";
import {
  HapticFeedbackPattern,
  isSupported,
  perform,
  PerformanceTime,
} from "tauri-plugin-macos-haptics-api";

export default function StepSlider(props: {
  min: number;
  max: number;
  step: number;
  initialValue?: number;
}) {
  const [sliderValue, setSliderValue] = createSignal(0);
  const [isDragging, setIsDragging] = createSignal(false);

  const updateSlider = async (event: InputEvent) => {
    const target = event.target as HTMLInputElement;
    const value = parseInt(target.value);

    if (isDragging() && (await isSupported()) && sliderValue() !== value) {
      perform(HapticFeedbackPattern.Alignment, PerformanceTime.Now);
    }

    setSliderValue(value);
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const generateSteps = () => {
    return Array.from(
      { length: (props.max - props.min) / props.step + 1 },
      (_, i) => props.min + i * props.step,
    );
  };

  return (
    <div class="w-6/12">
      <input
        type="range"
        min={props.min}
        max={props.max}
        value={sliderValue()}
        class="range"
        step={props.step}
        onInput={updateSlider}
        onMouseDown={(_) => {
          setIsDragging(false);
        }}
        onMouseMove={(_) => {
          if (!isDragging()) {
            setIsDragging(true);
          }
        }}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      />
      <div class="flex w-full justify-between px-2 text-xs">
        <For each={generateSteps()}>{() => <span>|</span>}</For>
      </div>
    </div>
  );
}
